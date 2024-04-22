use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::clone::Clone;

pub struct InputBox {
    text: String,
    cursor_position: usize,
    field_name: String,
    selected: bool,
}

impl InputBox {
    pub fn new(field_name: String, selected: bool) -> Self {
        InputBox {
            cursor_position: 0,
            text: String::new(),
            field_name,
            selected,
        }
    }

    pub fn focus(&mut self) {
        self.selected = true
    }

    pub fn unfocus(&mut self) {
        self.selected = false
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        if self.selected {
            match key.code {
                KeyCode::Char(c) => {
                    self.enter_char(c);
                }
                KeyCode::Backspace => self.delete_char(),
                KeyCode::Delete => self.delete_char(),
                _ => (),
            }
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let input = Paragraph::new(self.text.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .fg(Color::Yellow)
                    .title(self.field_name.as_str()),
            );

        frame.render_widget(input, area);

        if self.selected {
            let x = area.x + self.cursor_position as u16 + 1;
            let y = area.y + 1;
            frame.set_cursor(x, y)
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, new_text: &str) {
        self.text = String::from(new_text);
        self.cursor_position = self.text.len();
    }

    pub fn reset(&mut self) {
        self.cursor_position = 0;
        self.text.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.text.len())
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        self.text.insert(self.cursor_position, new_char);
        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            let last_idx = self.text.len() - 1;
            self.text.remove(last_idx);
            self.move_cursor_left();
        }
    }
}
