// Card Events
pub struct DrawCard {
    pub player_name: String,
}

pub struct CardDrawn {
    pub player_name: String,
    pub card_id: String,
}

pub struct ShuffleCardDeck {
    pub player_name: String,
}

pub struct PlayCard {
    pub player_name: String,
    pub card_id: String,
}

pub struct BurnCard {
    pub player_name: String,
    pub card_id: String,
}

// Combat Events
pub struct HealthPointsIncrease {
    pub player_name: String,
    pub value: usize,
}

pub struct HealthPointsDecrease {
    pub player_name: String,
    pub value: usize,
}

// Player State Events
pub struct PlayerCreate {
    pub player_name: String,
    pub secret: String,
}

pub struct PlayerJoined {
    pub player_name: String,
}

pub struct PlayerPaused {
    pub player_name: String,
}

pub struct PlayerExited {
    pub player_name: String,
}

pub struct PlayerDied {
    pub player_name: String,
}

// Item State Events
pub struct ItemDrawn {}

pub struct ItemTradeInitiated {}

pub struct ItemTradeDenied {}

pub struct ItemTradeAccepted {}

pub struct ItemUsed {}
