use aws_config::SdkConfig;
use color_eyre::Result;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::state_store::ContextInfo;

use super::{action::Action, state::State};

pub struct StateStore {
    config: SdkConfig,
    state_tx: UnboundedSender<State>,
}

impl StateStore {
    pub fn new(config: SdkConfig) -> (Self, UnboundedReceiver<State>) {
        let (state_tx, state_rx) = mpsc::unbounded_channel();
        (StateStore { config, state_tx }, state_rx)
    }

    pub async fn event_loop(&self, mut action_rx: UnboundedReceiver<Action>) -> Result<()> {
        let mut state = State::default();

        self.state_tx.send(state.clone())?;

        loop {
            tokio::select! {
                Some(action) = action_rx.recv() => match action {
                    Action::GetClusters => {
                        let sts_client = aws_sdk_sts::Client::new(&self.config);
                        let caller_identity = sts_client.get_caller_identity().send().await.unwrap();
                        state.context_info = Some(ContextInfo {caller_identity});
                    }
                    Action::Quit => {
                        break;
                    }
                },
            }
        }

        Ok(())
    }
}
