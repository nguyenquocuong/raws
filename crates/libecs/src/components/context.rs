use color_eyre::Result;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{List, ListItem},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::state_store::{action::Action, State};

use super::Component;

#[derive(Default)]
struct Props {
    arn: Option<String>,
}

impl From<&State> for Props {
    fn from(state: &State) -> Self {
        Props {
            arn: state.caller_arn.clone(),
        }
    }
}

#[derive(Default)]
pub struct Context {
    command_tx: Option<UnboundedSender<Action>>,
    props: Props,
}

impl Component for Context {
    fn set_state(&mut self, state: &State) {
        self.props = Props::from(state);
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn init(&mut self) -> Result<()> {
        if let Some(tx) = self.command_tx.clone() {
            tx.send(Action::GetContextInfo)?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let outer_area = rect;

        let [title_area, value_area] =
            Layout::horizontal([Constraint::Length(9), Constraint::Percentage(100)])
                .areas(outer_area);

        let title_items: Vec<ListItem> = vec![
            ListItem::new(Line::from("IAM ARN: ")),
            ListItem::new(Line::from("Cluster: ")),
        ];
        let value_items: Vec<ListItem> = vec![ListItem::new(Line::yellow(
            self.props.arn.clone().unwrap_or("".to_string()).into(),
        ))];

        frame.render_widget(List::new(title_items), title_area);
        frame.render_widget(List::new(value_items), value_area);
    }
}
