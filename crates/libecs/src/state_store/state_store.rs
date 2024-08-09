use aws_config::SdkConfig;
use color_eyre::Result;
use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
};

use crate::termination::{Interrupted, Terminator};

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

    pub async fn event_loop(
        &self,
        mut terminator: Terminator,
        mut action_rx: UnboundedReceiver<Action>,
        mut interrupt_rx: broadcast::Receiver<Interrupted>,
    ) -> Result<Interrupted> {
        let mut state = State::default();

        self.state_tx.send(state.clone())?;

        let result = loop {
            tokio::select! {
                Some(action) = action_rx.recv() => match action {
                    Action::GetContextInfo => {
                        let sts_client = aws_sdk_sts::Client::new(&self.config);
                        let caller_identity = sts_client.get_caller_identity().send().await.unwrap();

                        state.caller_arn = caller_identity.clone().arn;

                        self.state_tx.send(state.clone())?;
                    }
                    Action::Quit => {
                        let _ = terminator.terminate(Interrupted::UserInt);
                        break Interrupted::UserInt;
                    }
                    _ => {}
                },
                Ok(interrupted) = interrupt_rx.recv() => {
                    break interrupted;
                }
            }

            self.state_tx.send(state.clone())?;
        };

        Ok(result)
    }
}
