use ratatui::{
    layout::{Alignment, Direction},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
    prelude::{Constraint, Layout},
};

use crate::app::{ App, View};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    match app.view {
        View::Home => render_home( app, frame),
        View::Login => render_login(frame),
    }
}

pub fn render_home(app: &mut App , frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(70),
            ])
        .split(frame.size());

    let chat = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ])
        .split(layout[1]);

    frame.render_widget(
        Paragraph::new(format!(
            "This is a chat.\n\
                Press `Esc` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.counter
        ))
        .block(
            Block::bordered()
                .title("Template")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .centered(),
        chat[0],
    );
}

pub fn render_login(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Login")
            .block(
                Block::bordered()
                    .title("Login")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .centered(),
        frame.size(),
    )
}
