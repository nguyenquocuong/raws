use color_eyre::Result;
use ratatui::{
    layout::{Alignment, Constraint, Rect},
    prelude::Stylize,
    style::{palette::tailwind, Style},
    text::Text,
    widgets::{Block, Cell, Row, Table, TableState},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use super::{Action, Component};

pub struct ClusterItem {
    cluster: String,
}

pub struct Clusters {
    command_tx: Option<UnboundedSender<Action>>,
    state: TableState,
    items: Vec<ClusterItem>,
}

impl Clusters {
    pub fn new() -> Self {
        Self {
            command_tx: None,
            state: TableState::default().with_selected(0),
            items: vec![
                ClusterItem {
                    cluster: "develop".to_string(),
                },
                ClusterItem {
                    cluster: "staging".to_string(),
                },
                ClusterItem {
                    cluster: "production".to_string(),
                },
            ],
        }
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

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let header = ["NAME"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(
                Style::default()
                    .bold()
                    .bg(tailwind::CYAN.c200)
                    .fg(tailwind::BLACK),
            )
            .height(1);
        let rows = self.items.iter().map(|data| {
            let item = [&data.cluster];
            item.into_iter()
                .map(|content| Cell::from(Text::from(content.clone())))
                .collect::<Row>()
        });

        let t = Table::new(rows, [Constraint::Min(0)]).header(header);

        frame.render_stateful_widget(
            t.block(
                Block::bordered()
                    .title(format!(" {} ", "Clusters[3]"))
                    .title_alignment(Alignment::Center)
                    .style(Style::default().bold().fg(tailwind::CYAN.c200)),
            ),
            rect,
            &mut self.state,
        )
    }
}
