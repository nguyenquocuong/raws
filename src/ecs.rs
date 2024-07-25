use async_trait::async_trait;
use clap::Args;
use std::io::{stdout, Result};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Alignment, Constraint, Layout, Rect},
    widgets::{Block, Padding, Paragraph},
    Frame, Terminal,
};

use crate::traits::CommandExecute;

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct EcsArgs;

#[derive(Default)]
struct App {
    exit: bool,
}

#[async_trait]
impl CommandExecute for EcsArgs {
    async fn execute(&self) -> Result<()> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        App::default().run(&mut terminal)?;

        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        while !self.exit {
            self.draw(terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|frame| self.render_frame(frame))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.exit = true;
                    return Ok(());
                }
            }
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        let layout = Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(frame.size());

        self.draw_context(frame, layout[0]);
        self.draw_content_block(frame, layout[1]);
    }

    fn draw_context(&self, frame: &mut Frame, area: Rect) {
        let [context_area] = Layout::horizontal([
            Constraint::Percentage(40),
            //Constraint::Percentage(20),
            //Constraint::Percentage(20),
            //Constraint::Percentage(40),
        ])
        .areas(area);

        frame.render_widget(Paragraph::new("IAM ARN:"), context_area);
    }

    fn draw_content_block(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Block::bordered()
                .title(" Clusters ")
                .title_alignment(Alignment::Center),
            area,
        );
    }
}
