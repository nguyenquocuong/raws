use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use super::state::State;

pub struct StateStore {
    state_tx: UnboundedSender<State>,
}

impl StateStore {
    pub fn new() -> (Self, UnboundedReceiver<State>) {
        let (state_tx, state_rx) = mpsc::unbounded_channel();
        (StateStore { state_tx }, state_rx)
    }
}
