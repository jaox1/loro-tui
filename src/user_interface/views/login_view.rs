use std::borrow::Borrow;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Borders, Paragraph};
use tokio::sync::mpsc::UnboundedSender;

use crate::states::action::Action;
use crate::states::state::State;
use crate::user_interface::component::Component;

use super::widgets::input_box::InputBox;

pub struct LoginView {
    action_sender: UnboundedSender<Action>,
    state: State,
    nickname_box: InputBox,
    password_box: InputBox,
    focus_widget: u8,
}

impl LoginView {
    // new function always takes 2 arguments: action_sender and state.
    // action_sender is an UnboundedSender<Action> and state is the source of truth for the view.
    pub fn new(action_sender: UnboundedSender<Action>, state: &State) -> Self {
        return LoginView {
            action_sender,
            state: state.clone(),
            nickname_box: InputBox::new("NICKNAME".to_string(), true),
            password_box: InputBox::new("PASSWORD".to_string(), false),
            focus_widget: 0,
        };
    }

    pub fn move_focus(&mut self) {
        self.focus_widget = (self.focus_widget + 1) % 2;
        if self.focus_widget == 0 {
            self.nickname_box.focus();
            self.password_box.unfocus();
        } else {
            self.nickname_box.unfocus();
            self.password_box.focus();
        }
    }
}

impl Component for LoginView {
    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => {
                self.move_focus();
            }
            _ => (),
        }
        self.nickname_box.handle_key_event(key);
        self.password_box.handle_key_event(key);
    }

    fn render(&self, frame: &mut ratatui::Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(frame.size());

        let login_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(layout[1]);

        //self.nickname_box.render(frame, login_area[0]);
        //self.password_box.render(frame, login_area[1]);

        //let help_text = Paragraph::new("Press <Enter> to login or <Esc> to exit")
        //.block(Block::default().borders(Borders::ALL).fg(Color::Yellow));
        frame.render_widget(Block::default().borders(Borders::all()), login_area[1]);
    }
}
