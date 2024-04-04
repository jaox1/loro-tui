use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};
use ratatui::widgets::Paragraph;
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
    fn handle_key_event(&mut self, key: KeyEvent) {}
    fn render(&self, frame: &mut ratatui::Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Min(1),
                Constraint::Ratio(1, 3),
            ])
            .split(frame.size());
        let sub_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Min(1),
                Constraint::Ratio(1, 3),
            ])
            .split(layout[1]);
        let center = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .split(sub_layout[1]);

        self.input_box.render(frame, center[0]);

        let help_text = Paragraph::new(Text::from(Line::from(vec![
            "Press ".into(),
            "<Enter>".bold(),
            " to login or ".into(),
            "<Esc>".bold(),
            " to exit".into(),
        ])));
        frame.render_widget(help_text, center[1]);
    }
}
