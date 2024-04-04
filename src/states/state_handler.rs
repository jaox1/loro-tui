use std::error::Error;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::models::error::AppError;

use super::{action::Action, state::State};

pub struct StateHandler {
    state_sender: UnboundedSender<State>,
}

impl StateHandler {
    pub fn new() -> (Self, UnboundedReceiver<State>) {
        let (state_sender, state_receiver) = tokio::sync::mpsc::unbounded_channel();
        (Self { state_sender }, state_receiver)
    }

    pub async fn run(self, mut action_receiver: UnboundedReceiver<Action>) -> Result<(), AppError> {
        loop {
            tokio::select! {
                Some(action) = action_receiver.recv() => {
                    self.handle_action(action).await?;
                }
            }
        }
        Ok(())
    }

    async fn handle_action(&self, action: Action) -> Result<(), AppError> {
        match action {
            Action::SendMessage { content } => {}
            Action::Authenticate { username, password } => {}
            Action::Exit => {}
            Action::JoinChat { chatId } => {}
            Action::CreateChat { receiver } => {}
        }
        Ok(())
    }
}
