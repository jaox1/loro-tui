use std::io::Stdout;

use crossterm::event::{Event, EventStream, KeyEvent};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_stream::StreamExt;

use crate::models::error::AppError;
use crate::states::action::Action;
use crate::states::state::{State, View};

use super::component::Component;
use super::views::chat_view::ChatView;
use super::views::login_view::LoginView;

pub struct UiHandler {
    action_sender: UnboundedSender<Action>,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    current_view: Box<dyn Component>,
}

impl UiHandler {
    pub fn new(terminal: Terminal<CrosstermBackend<Stdout>>) -> (Self, UnboundedReceiver<Action>) {
        let (action_sender, action_receiver) = tokio::sync::mpsc::unbounded_channel();
        let current_view = Box::new(LoginView::new(action_sender.clone(), &State::default()));
        (
            UiHandler {
                action_sender,
                terminal,
                current_view,
            },
            action_receiver,
        )
    }

    pub async fn run(
        &mut self,
        mut state_receiver: UnboundedReceiver<State>,
    ) -> Result<(), AppError> {
        let mut crossterm_events = EventStream::new();
        loop {
            tokio::select! {
                Some(state) = state_receiver.recv() => {
                    self.current_view = match state.current_view {
                        View::Login => Box::new(LoginView::new(self.action_sender.clone(), &state)),
                        View::Chat => Box::new(ChatView::new())
                    };
                },

                event = crossterm_events.next() => match event {
                    Some(Ok(Event::Key(key))) => {
                        self.current_view.handle_key_event(key);
                    },
                    _ => (),
                },

            }
            let _ = self.terminal.draw(|frame| {
                self.current_view.render(frame);
            });
        }
    }
}
