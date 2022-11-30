use bevy::{input::keyboard::KeyboardInput, log::LogPlugin, prelude::*};

mod components;
mod events;
mod state;
mod systems;

use state::{CliInput, GameState};

fn main() {
    let mut app = App::new();
    add_plugins(&mut app);
    init_resources(&mut app);
    add_events(&mut app);
    add_systems(&mut app);
    app.run();
}

fn add_plugins(app: &mut App) {
    app.add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default());
}

fn init_resources(app: &mut App) {
    app.init_resource::<GameState>()
        .init_resource::<CliInput>()
        .init_resource::<Input<KeyCode>>();
}

fn add_events(app: &mut App) {
    use events::*;
    app
        // Card Events
        .add_event::<DrawCard>()
        .add_event::<CardDrawn>()
        .add_event::<ShuffleCardDeck>()
        .add_event::<PlayCard>()
        .add_event::<BurnCard>()
        // Combat Events
        .add_event::<HealthPointsIncrease>()
        .add_event::<HealthPointsDecrease>()
        // Player Events
        .add_event::<PlayerCreate>()
        .add_event::<PlayerJoined>()
        .add_event::<PlayerPaused>()
        .add_event::<PlayerExited>()
        .add_event::<PlayerDied>()
        // Keyboard Events
        .add_event::<ReceivedCharacter>();
}

fn add_systems(app: &mut App) {
    use systems::cli_input::*;
    use systems::players::*;
    app
        // Player Systems
        .add_system(player_create)
        .add_system(player_created_debug)
        .add_system(cli_input);
}
