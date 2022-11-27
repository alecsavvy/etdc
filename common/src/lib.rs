use serde::{Deserialize, Serialize};

// Envelope for all messages, structure for metadata that all commands and events need
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope<T> {
    game_id: Option<String>, // so different games don't get random messages
    sender: String,          // user id or server id
    msg: T,                  // command or event
}

// Commands are things that one part of the system wants to have happen
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Commands {
    CreateGame,
    JoinGame,
    CreateCharacter,
    PauseGame,
    QuitGame,
}

// Events are things that have happened, these can be the results of commands
// or something system or automatically generated
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Events {
    GameCreated,
    GameJoined,
    CharacterCreated,
    GamePaused,
    GameEnded,
}
