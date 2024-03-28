#[derive(Debug, Clone)]
pub enum Action {
    Authenticate { addr: String },
    SendMessage { content: String },
    SelectChat { room: String },
    Exit,
}
