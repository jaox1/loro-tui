use ratatui::layout::{Constraint, Direction, Layout};
use tokio::sync::mpsc::UnboundedSender;

use crate::states::action::Action;
use crate::states::state::State;
use crate::user_interface::component::Component;

use super::widgets::input_box::InputBox;

pub struct LoginView {
    action_sender: UnboundedSender<Action>,
    state: State,
    input_box: InputBox,
}

impl LoginView {
    // new function always takes 2 arguments: action_sender and state.
    // action_sender is an UnboundedSender<Action> and state is the source of truth for the view.
    pub fn new(action_sender: UnboundedSender<Action>, state: &State) -> Self {
        return LoginView {
            action_sender,
            state: state.clone(),
            input_box: InputBox::new(),
        };
    }
}

impl Component for LoginView {
    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {}
    fn render(&self, frame: &mut ratatui::Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Min(1),
                Constraint::Ratio(1, 3),
            ])
            .split(frame.size());
    }
}
