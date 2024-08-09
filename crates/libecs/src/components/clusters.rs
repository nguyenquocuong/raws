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

use crate::state_store::{action::Action, ClusterItem, State};

use super::Component;

#[derive(Default)]
struct Props {
    clusters: Vec<ClusterItem>,
}

impl From<&State> for Props {
    fn from(state: &State) -> Self {
        let clusters = state
            .cluster_arns
            .clone()
            .iter()
            .map(|arn| ClusterItem::from(arn.clone()))
            .collect::<Vec<ClusterItem>>();

        //clusters.sort_by(|room_a, room_b| room_a.name.cmp(&room_b.name));

        Self { clusters }
    }
}

pub struct Clusters {
    action_tx: Option<UnboundedSender<Action>>,
    props: Props,
    table_state: TableState,
}

impl Clusters {
    pub fn new() -> Self {
        Self {
            action_tx: None,
            props: Props::default(),
            table_state: TableState::default().with_selected(None),
        }
    }

    fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.props.clusters.len() - 1 {
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
            None => self.props.clusters.len() - 1,
        };
        self.table_state.select(Some(i))
    }
}

impl Component for Clusters {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn init(&mut self) -> Result<()> {
        if let Some(tx) = self.action_tx.clone() {
            tx.send(Action::GetClusters)?;
        }
        Ok(())
    }

    fn set_state(&mut self, state: &State) {
        self.props = Props::from(state);
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
        //println!("{:?}", self.props.clusters);
        let selected_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(tailwind::CYAN.c100);

        let header = ["NAME"].into_iter().map(Cell::from).collect::<Row>().style(
            Style::default()
                .bold()
                .bg(tailwind::CYAN.c200)
                .fg(tailwind::BLACK),
        );
        let rows = self.props.clusters.iter().map(|data| {
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
                    .title(format!(" Clusters[{}] ", self.props.clusters.len()))
                    .title_alignment(Alignment::Center)
                    .style(Style::default().bold().fg(tailwind::CYAN.c200)),
            ),
            rect,
            &mut self.table_state,
        )
    }
}
