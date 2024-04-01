use std::io::Stdout;

use crossterm::event::{Event, EventStream, KeyEvent};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_stream::StreamExt;

use crate::states::action::Action;
use crate::states::state::{State, View};

use super::component::Component;
use super::view::chat_view::ChatView;
use super::view::login_view::LoginView;

enum UIError {
    Default
}

pub struct UiHandler {
    action_sender: UnboundedSender<Action>,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    current_view: Box<dyn Component>
}

impl UiHandler {
    pub async fn run(&mut self, mut state_receiver: UnboundedReceiver<State>) -> Result<(), UIError> {
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

