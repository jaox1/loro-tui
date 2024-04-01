
use tokio::sync::mpsc::UnboundedSender;

use crate::states::action::Action;
use crate::states::state::State;
use crate::user_interface::component::Component;

pub struct LoginView {
    action_sender: UnboundedSender<Action>,
    state: State
}

impl LoginView {
    pub fn new(action_sender: UnboundedSender<Action>, state: &State) -> Self {
        return LoginView {
            action_sender,
            state: state.clone()
        };
    }
    
}

impl Component for LoginView {
    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        
    }
    fn render(&self, frame: &mut ratatui::Frame) {
        
    }
}