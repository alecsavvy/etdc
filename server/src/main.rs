use bevy::{log::LogPlugin, prelude::*};

mod adapters;
mod components;
mod events;
mod state;
mod systems;

use state::GameState;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(adapters::ws::handle_connection(stream));
    }

    let mut app = App::new();
    add_plugins(&mut app);
    init_resources(&mut app);
    add_events(&mut app);
    add_systems(&mut app);
    app.run();

    Ok(())
}

fn add_plugins(app: &mut App) {
    app.add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default());
}

fn init_resources(app: &mut App) {
    app.init_resource::<GameState>();
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
        .add_event::<PlayerDied>();
}

fn add_systems(app: &mut App) {
    use systems::players::*;
    app
        // Player Systems
        .add_system(player_create)
        .add_system(player_created_debug);
}
