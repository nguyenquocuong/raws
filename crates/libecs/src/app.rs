use std::io::stdout;

use aws_config::SdkConfig;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Layout, Rect},
    Frame, Terminal,
};

use crate::ui::{ContentWidget, ContextWidget, KeybindingsWidget, LogoWidget};

pub async fn run_app() -> Result<(), std::io::Error> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let config = aws_config::load_from_env().await;
    let mut app = App::new(config);

    app.get_caller_identity().await;
    app.run(&mut terminal)?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

#[derive(Clone)]
pub struct App {
    config: SdkConfig,
    iam_arn: String,
    exit: bool,
}

impl App {
    pub fn new(config: SdkConfig) -> Self {
        Self {
            config,
            iam_arn: String::from(""),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<(), std::io::Error> {
        while !self.exit {
            self.draw(terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> Result<(), std::io::Error> {
        terminal.draw(|frame| self.render_frame(frame))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<(), std::io::Error> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.exit();
                    return Ok(());
                }
            }
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
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

        frame.render_widget(
            ContextWidget::default().iam_arn(self.iam_arn.clone()),
            context_area,
        );
        frame.render_widget(KeybindingsWidget::default(), keybindings_area);
        frame.render_widget(LogoWidget::default(), logo_area);
    }

    fn draw_content_block(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(ContentWidget::default(), area);
    }
}

impl App {
    pub async fn get_caller_identity(&mut self) {
        let sts_client = aws_sdk_sts::Client::new(&self.config);

        let caller_identity = sts_client.get_caller_identity().send().await.unwrap();

        self.iam_arn = caller_identity.arn().unwrap().to_string();
    }
}
