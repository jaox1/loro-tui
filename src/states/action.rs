#[derive(Debug, Clone)]
pub enum Action {
    Authenticate { username: String, password: String },
    SendMessage { content: String },
    JoinChat { chatId: String },
    Exit,
    CreateChat { receiver: String }
}
