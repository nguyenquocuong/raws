use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Rect},
    prelude::Stylize,
    style::{palette::tailwind, Modifier, Style},
    text::Text,
    widgets::{Block, Cell, Row, Table, TableState},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::state_store::ClusterItem;

use super::{Action, Component};

pub struct Clusters {
    command_tx: Option<UnboundedSender<Action>>,
    table_state: TableState,
    items: Vec<ClusterItem>,
}

impl Clusters {
    pub fn new() -> Self {
        Self {
            command_tx: None,
            table_state: TableState::default().with_selected(None),
            items: vec![
                ClusterItem {
                    name: "develop".to_string(),
                },
                ClusterItem {
                    name: "staging".to_string(),
                },
                ClusterItem {
                    name: "production".to_string(),
                },
            ],
        }
    }

    fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i))
    }

    fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    i
                } else {
                    i - 1
                }
            }
            None => self.items.len() - 1,
        };
        self.table_state.select(Some(i))
    }
}

impl Component for Clusters {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn init(&mut self) -> Result<()> {
        if let Some(tx) = self.command_tx.clone() {
            tx.send(Action::GetClusters)?;
        }
        Ok(())
    }

    fn update(&mut self, action: Action) -> Action {
        println!("{action:?}");
        action
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Action {
        if key.kind != KeyEventKind::Press {
            return Action::Noop;
        }

        match key.code {
            KeyCode::Char('k') => {
                self.previous();
                Action::Noop
            }
            KeyCode::Char('j') => {
                self.next();
                Action::Noop
            }
            _ => Action::Noop,
        }
    }

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let selected_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(tailwind::CYAN.c100);

        let header = ["NAME"].into_iter().map(Cell::from).collect::<Row>().style(
            Style::default()
                .bold()
                .bg(tailwind::CYAN.c200)
                .fg(tailwind::BLACK),
        );
        let rows = self.items.iter().map(|data| {
            let item = [&data.name];
            item.into_iter()
                .map(|content| Cell::from(Text::from(content.clone())))
                .collect::<Row>()
        });

        let t = Table::new(rows, [Constraint::Min(0)])
            .header(header)
            .highlight_style(selected_style);

        frame.render_stateful_widget(
            t.block(
                Block::bordered()
                    .title(format!(" {} ", "Clusters[3]"))
                    .title_alignment(Alignment::Center)
                    .style(Style::default().bold().fg(tailwind::CYAN.c200)),
            ),
            rect,
            &mut self.table_state,
        )
    }
}
