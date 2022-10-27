use bevy::prelude::*;

mod components;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(read_castle)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Castle)
        .insert(Name("Default Castle".to_string()));
}

fn read_castle(query: Query<&Name, With<Castle>>) {
    for name in query.iter() {
        println!("found castle {}!", name.0);
    }
}

#[derive(Debug, Component)]
struct Castle;

#[derive(Debug, Component)]
struct Name(String);
