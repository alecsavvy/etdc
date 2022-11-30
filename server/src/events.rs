use serde::{Deserialize, Serialize};

// Card Events
#[derive(Debug, Serialize, Deserialize)]
pub struct DrawCard {
    pub player_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardDrawn {
    pub player_name: String,
    pub card_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShuffleCardDeck {
    pub player_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayCard {
    pub player_name: String,
    pub card_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BurnCard {
    pub player_name: String,
    pub card_id: String,
}

// Combat Events
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthPointsIncrease {
    pub player_name: String,
    pub value: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthPointsDecrease {
    pub player_name: String,
    pub value: usize,
}

// Player State Events
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerCreate {
    pub player_name: String,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerJoined {
    pub player_name: String,
}

pub struct PlayerPaused {
    pub player_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerExited {
    pub player_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerDied {
    pub player_name: String,
}

// Item State Events
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemDrawn {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemTradeInitiated {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemTradeDenied {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemTradeAccepted {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemUsed {}
