
use std::error::Error;

use tokio::sync::mpsc::{ UnboundedReceiver, UnboundedSender};

use super::{action::Action, state::State};

enum ErrorState {
    Default,
}

pub struct StateHandler {
    state_sender: UnboundedSender<State>
}

impl StateHandler {
    pub async fn run(self, mut action_receiver: UnboundedReceiver<Action>) -> Result<(), ErrorState> {
        loop {
            tokio::select! {
                Some(action) = action_receiver.recv() => {
                    self.handle_action(action).await?;
                }
            }
        }
        Ok(())
    }

    async fn handle_action(&self, action: Action) -> Result<(), ErrorState> {
        match action {
            Action::SendMessage { content } => {
                
            }
            Action::Authenticate { username, password } => {
                
            }
            Action::Exit => {
                
            }
            Action::JoinChat { chatId } => {
                
            }
            Action::CreateChat { receiver } => {

            }
        }
        Ok(())
    }
}
