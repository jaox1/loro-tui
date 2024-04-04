mod models;
use std::io::stdout;

use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::states::state_handler::StateHandler;
use crate::user_interface::ui_handler::UiHandler;

mod states;
mod user_interface;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut stdout = stdout();
    enable_raw_mode();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture);

    let terminal = Terminal::new(CrosstermBackend::new(stdout)).unwrap();

    let (state_handler, state_receiver) = StateHandler::new();
    let (mut ui_handler, action_receiver) = UiHandler::new(terminal);

    tokio::try_join!(
        state_handler.run(action_receiver),
        ui_handler.run(state_receiver)
    );

    Ok(())
}
