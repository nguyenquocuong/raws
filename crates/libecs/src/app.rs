use color_eyre::Result;
use std::io::stdout;

use aws_config::SdkConfig;
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
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

use crate::{
    components::{clusters::Clusters, Component, Event},
    state_store::{action::Action, State, StateStore},
    ui::{Context, KeybindingsWidget, LogoWidget},
};

pub async fn run_app() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let config = aws_config::load_from_env().await;
    let (state_store, state_tx) = StateStore::new(config);
    let (mut app, action_rx) = App::new(state_tx);

    tokio::try_join!(app.run(&mut terminal), state_store.event_loop(action_rx))?;
    //app.run(&mut terminal).await?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

pub struct App {
    should_quit: bool,
    task: JoinHandle<()>,
    event_tx: UnboundedSender<Event>,
    event_rx: UnboundedReceiver<Event>,
    action_tx: UnboundedSender<Action>,
    state_rx: UnboundedReceiver<State>,

    cluster_component: Clusters,
}

impl App {
    pub fn new(state_rx: UnboundedReceiver<State>) -> (Self, UnboundedReceiver<Action>) {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let (action_tx, action_rx) = mpsc::unbounded_channel();

        (
            Self {
                should_quit: false,
                task: tokio::spawn(async {}),
                event_tx,
                event_rx,
                action_tx,
                state_rx,
                cluster_component: Clusters::new(),
            },
            action_rx,
        )
    }

    pub async fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        let event_loop = Self::event_loop(self.event_tx.clone());

        self.task = tokio::spawn(async {
            event_loop.await;
        });

        loop {
            self.draw(terminal)?;
            self.handle_events().await?;
            //self.handle_actions().await?;
            if self.should_quit {
                break;
            }
        }
        self.task.abort();
        Ok(())
    }

    async fn event_loop(event_tx: UnboundedSender<Event>) {
        let mut event_stream = EventStream::new();

        event_tx
            .send(Event::Init)
            .expect("failed to send init event");
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

    async fn handle_events(&mut self) -> Result<()> {
        let Some(event) = self.event_rx.recv().await else {
            return Ok(());
        };
        let action_tx = self.action_tx.clone();

        match event {
            Event::Quit => action_tx.send(Action::Quit)?,
            //Event::Tick => action_tx.send(Action::Tick)?,
            Event::Key(key) => self.handle_key_event(key)?,
            _ => {}
        };

        self.cluster_component.handle_events(Some(event.clone()));

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        let action_tx = self.action_tx.clone();

        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
            action_tx.send(Action::Quit)?;
        }

        Ok(())
    }

    //async fn handle_actions(&mut self) -> Result<()> {
    //    while let Ok(action) = self.action_rx.try_recv() {
    //        if action != Action::Tick {
    //            println!("{action:?}");
    //        }
    //
    //        match action {
    //            Action::Quit => self.should_quit = true,
    //            Action::Render => {}
    //            _ => {}
    //        }
    //    }
    //
    //    Ok(())
    //}

    fn render_frame(&mut self, frame: &mut Frame) {
        let [context, content] =
            Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)])
                .areas(frame.size());

        self.draw_context(frame, context);
        self.draw_content_block(frame, content);
    }

    fn draw_context(&self, frame: &mut Frame, area: Rect) {
        let [context_area, keybindings_area, logo_area] = Layout::horizontal([
            Constraint::Percentage(40),
            Constraint::Percentage(10),
            Constraint::Percentage(50),
        ])
        .areas(area);

        let mut context = Context::default();
        context.init().unwrap();
        context
            //.iam_arn(self.iam_arn.clone())
            .draw(frame, context_area);

        frame.render_widget(KeybindingsWidget::default(), keybindings_area);
        frame.render_widget(LogoWidget::default(), logo_area);
    }

    fn draw_content_block(&mut self, frame: &mut Frame, area: Rect) {
        self.cluster_component.draw(frame, area);
    }
}
