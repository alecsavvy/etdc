use bevy::prelude::*;
use rand::seq::SliceRandom;

#[derive(Debug, Component)]
pub struct CharacterDice(Vec<CharacterDiceSide>);

#[derive(Debug, Component)]
pub struct CharacterDiceSide(CharacterTrait, Quantity);

#[derive(Debug, Component)]
pub enum Quantity {
    Single,
    Double,
}

#[derive(Debug, Component)]
pub enum CharacterTrait {
    Might,
    Cunning,
    Wisdom,
}

impl CharacterDice {
    pub fn roll(&self) -> &CharacterDiceSide {
        self.0
            .choose(&mut rand::thread_rng())
            .expect("dice has no sides")
    }
}
