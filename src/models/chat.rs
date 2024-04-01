use std::collections::HashSet;


#[derive(Debug, Clone)]
pub enum Communication {
    Message { user_id: String, content: String },
    Notification(String),
}

const MAX_MESSAGES_TO_STORE_PER_ROOM: usize = 100;


#[derive(Debug, Clone)]
pub struct Chat {
    /// The name of the room
    pub name: String,
    /// The description of the Room
    pub description: String,
    /// List of users in the room
    pub users: HashSet<String>,
    /// History of recorded messages
    pub messages: Vec<Communication>,
    /// Has joined the room
    pub has_joined: bool,
    /// Has unread messages
    pub has_unread: bool,
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            users: HashSet::new(),
            messages: Vec::new(),
            has_joined: false,
            has_unread: false,
        }
    }
}

impl Chat {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            ..Default::default()
        }
    }
}
