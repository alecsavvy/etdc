#[derive(Debug, Copy, Clone)]
pub enum GameMode {
    Solo,
    Duo,
    Trio,
    Squad,
}

pub struct Config {
    num_players: GameMode,
    starting_hp: usize,
}

impl Config {
    pub fn from_player_count(num_players: GameMode) -> Self {
        let mut conf = Self {
            num_players,
            starting_hp: 0,
        };
        match num_players {
            GameMode::Solo => conf.starting_hp = 18,
            GameMode::Duo => conf.starting_hp = 18,
            GameMode::Trio => conf.starting_hp = 14,
            GameMode::Squad => conf.starting_hp = 12,
        };
        conf
    }
}
