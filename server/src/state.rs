use bevy::prelude::*;

#[derive(Debug, Resource, Default)]
pub struct GameState {
    pub current_chapter: String,
    pub total_players: usize,
}
