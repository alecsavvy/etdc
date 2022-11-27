use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Name(String);

#[derive(Debug, Component, Default)]
pub struct HealthPoints {
    pub value: usize,
}
