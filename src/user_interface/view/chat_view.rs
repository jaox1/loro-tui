
use crate::user_interface::component::Component;

pub struct ChatView {
}

impl ChatView {
    pub fn new() -> Self {
        return ChatView {};
    }
    
}

impl Component for ChatView {
    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        
    }
    fn render(&self, frame: &mut ratatui::Frame) {
        
    }
    
}