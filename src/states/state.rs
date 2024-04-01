use std::collections::HashMap;
use crate::models::chat::Chat;

#[derive(Debug, Clone)]
pub enum WebSocketStatus {
    Uninitalized,
    Connecting,
    Connected { addr: String },
    Failed { err: String },
}

#[derive(Debug, Clone)]
pub enum View {
    Login,
    Chat,
}

#[derive(Debug, Clone)]
pub struct State {
    pub server_connection_status: WebSocketStatus,
    pub active_room: Option<String>,
    pub user_id: String,
    pub chat_rooms: HashMap<String, Chat>,
    pub current_view: View,
    /// Timer since app was opened
    pub timer: usize,
}

impl Default for State {
    fn default() -> Self {
        State {
            server_connection_status: WebSocketStatus::Uninitalized,
            active_room: None,
            user_id: String::new(),
            chat_rooms: HashMap::new(),
            current_view: View::Login,
            timer: 0,
        }
    }
}