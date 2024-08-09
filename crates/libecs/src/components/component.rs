use color_eyre::Result;
use ratatui::crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;
use ratatui::Frame;
use tokio::sync::mpsc::UnboundedSender;

use crate::state_store::action::Action;
use crate::state_store::State;

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Init,
    Quit,
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Paste(String),
    FocusGained,
    FocusLost,
    Resize(u16, u16),
    Error,
}

pub trait Component {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        let _ = tx; // to appease clippy
        Ok(())
    }
    fn init(&mut self) -> Result<()> {
        Ok(())
    }
    fn move_with_state(&mut self, state: &State);
    fn handle_events(&mut self, event: Option<Event>) -> Action {
        match event {
            Some(Event::Key(key_event)) => self.handle_key_event(key_event),
            Some(Event::Mouse(mouse_event)) => self.handle_mouse_event(mouse_event),
            _ => Action::Noop,
        }
    }
    fn handle_key_event(&mut self, key: KeyEvent) -> Action {
        let _ = key;
        Action::Noop
    }
    fn handle_mouse_event(&mut self, mouse: MouseEvent) -> Action {
        let _ = mouse;
        Action::Noop
    }
    fn update(&mut self, action: Action) -> Action {
        let _ = action;
        Action::Noop
    }
    fn draw(&mut self, frame: &mut Frame, rect: Rect);
}
