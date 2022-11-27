use bevy::log::*;
use bevy::prelude::*;

use crate::{
    components::{common::HealthPoints, player::Player},
    events::{PlayerCreate, PlayerJoined},
    state::GameState,
};

// player create system, reacts to player create events, creates them, then publishes a joined event
pub fn player_create(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut ev_pc: EventReader<PlayerCreate>,
    mut ev_pj: EventWriter<PlayerJoined>,
) {
    // read player create events and spawn them
    let mut players = vec![];
    let mut player_joined = vec![];
    for new_player in ev_pc.iter() {
        players.push((
            Player {
                name: new_player.player_name.clone(),
            },
            HealthPoints::default(),
        ));
        player_joined.push(PlayerJoined {
            player_name: new_player.player_name.clone(),
        })
    }

    let new_player_count = players.len();
    commands.spawn_batch(players);

    // update count in game state
    game_state.total_players += new_player_count;

    // emit character joined events
    ev_pj.send_batch(player_joined);
}

pub fn player_created_debug(game_state: ResMut<GameState>, mut ev_pj: EventReader<PlayerJoined>) {
    for ev in ev_pj.iter() {
        info!(
            "Player '{}' has been created, total player count '{}'",
            ev.player_name, game_state.total_players
        );
    }
}
