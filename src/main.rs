use bevy::prelude::*;

mod components;
mod config;

fn main() {
    App::new().add_startup_system(setup).run();
}

fn setup(mut commands: Commands) {}
