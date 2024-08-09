use color_eyre::Result;
use std::io::stdout;

use crossterm::event::{Event as CrosstermEvent, EventStream, KeyCode, KeyEvent};
use futures::{FutureExt, StreamExt};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::KeyEventKind,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Layout, Rect},
    Frame, Terminal,
};
use tokio::{
    sync::{
        broadcast,
        mpsc::{self, UnboundedReceiver, UnboundedSender},
    },
    task::JoinHandle,
};

use crate::{
    components::{clusters::Clusters, context::Context, Component, Event},
    state_store::{action::Action, State, StateStore},
    termination::{create_termination, Interrupted},
    ui::{KeybindingsWidget, LogoWidget},
};

pub async fn run_app() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let config = aws_config::load_from_env().await;

    let (terminator, mut interrupt_rx) = create_termination();
    let (state_store, state_rx) = StateStore::new(config);
    let (mut app, action_rx) = App::new(state_rx);

    tokio::try_join!(
        state_store.event_loop(terminator, action_rx, interrupt_rx.resubscribe()),
        app.run(&mut terminal, interrupt_rx.resubscribe()),
    )?;

    if let Ok(reason) = interrupt_rx.recv().await {
        match reason {
            Interrupted::UserInt => println!("exited per user request"),
            Interrupted::OsSigInt => println!("exited because of an os sig int"),
        }
    } else {
        println!("exited because of an unexpected error");
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

pub struct App {
    task: JoinHandle<()>,
    event_tx: UnboundedSender<Event>,
    event_rx: UnboundedReceiver<Event>,
    action_tx: UnboundedSender<Action>,
    state_rx: UnboundedReceiver<State>,

    context_component: Context,
    cluster_component: Clusters,
}

impl App {
    pub fn new(state_rx: UnboundedReceiver<State>) -> (Self, UnboundedReceiver<Action>) {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let (action_tx, action_rx) = mpsc::unbounded_channel();

        (
            Self {
                task: tokio::spawn(async {}),
                event_tx,
                event_rx,
                action_tx,
                state_rx,
                context_component: Context::default(),
                cluster_component: Clusters::new(),
            },
            action_rx,
        )
    }

    pub async fn run(
        &mut self,
        terminal: &mut Terminal<impl Backend>,
        mut interrupt_rx: broadcast::Receiver<Interrupted>,
    ) -> Result<Interrupted> {
        self.task = tokio::spawn(Self::event_loop(self.event_tx.clone()));

        self.context_component
            .register_action_handler(self.action_tx.clone())?;
        self.cluster_component
            .register_action_handler(self.action_tx.clone())?;

        self.context_component.init()?;
        self.cluster_component.init()?;

        let result: Result<Interrupted> = loop {
            tokio::select! {
                Some(event) = self.event_rx.recv() => {
                    let action_tx = self.action_tx.clone();

                    match event {
                        Event::Quit => action_tx.send(Action::Quit)?,
                        Event::Key(key) => self.handle_key_event(key)?,
                        _ => {}
                    };

                    self.context_component.handle_events(Some(event.clone()));
                    self.cluster_component.handle_events(Some(event.clone()));
                },
                Some(state) = self.state_rx.recv() => {
                    //println!("{state:?}");
                    self.context_component.set_state(&state);
                    self.cluster_component.set_state(&state);
                },
                Ok(interrupted) = interrupt_rx.recv() => {
                    break Ok(interrupted);
                }
            }

            self.draw(terminal)?;
        };

        self.task.abort();

        result
    }

    async fn event_loop(event_tx: UnboundedSender<Event>) {
        let mut event_stream = EventStream::new();

        loop {
            let event = tokio::select! {
                crossterm_event = event_stream.next().fuse() => match crossterm_event {
                    Some(Ok(event)) => match event {
                        CrosstermEvent::Key(key) if key.kind == KeyEventKind::Press => Event::Key(key),
                        CrosstermEvent::Mouse(mouse) => Event::Mouse(mouse),
                        CrosstermEvent::Resize(x, y) => Event::Resize(x, y),
                        CrosstermEvent::FocusLost => Event::FocusLost,
                        CrosstermEvent::FocusGained => Event::FocusGained,
                        CrosstermEvent::Paste(s) => Event::Paste(s),
                        _ => continue,
                    },
                    Some(Err(_)) => Event::Error,
                    None => break,
                }
            };
            if event_tx.send(event).is_err() {
                break;
            }
        }
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|frame| self.render_frame(frame))?;
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        let action_tx = self.action_tx.clone();

        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
            action_tx.send(Action::Quit)?;
        }

        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let [context, content] =
            Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)])
                .areas(frame.size());

        self.draw_context(frame, context);
        self.draw_content_block(frame, content);
    }

    fn draw_context(&mut self, frame: &mut Frame, area: Rect) {
        let [context_area, keybindings_area, logo_area] = Layout::horizontal([
            Constraint::Percentage(40),
            Constraint::Percentage(10),
            Constraint::Percentage(50),
        ])
        .areas(area);

        self.context_component.draw(frame, context_area);

        frame.render_widget(KeybindingsWidget::default(), keybindings_area);
        frame.render_widget(LogoWidget::default(), logo_area);
    }

    fn draw_content_block(&mut self, frame: &mut Frame, area: Rect) {
        self.cluster_component.draw(frame, area);
    }
}
