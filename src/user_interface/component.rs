use crossterm::event::KeyEvent;
use ratatui::Frame;

pub trait Component {
    fn handle_key_event(&mut self, key: KeyEvent);
    fn render(&self, frame: &mut Frame);
}
