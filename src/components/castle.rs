use bevy::prelude::*;

use super::{
    boss_card::BossCard, chapter_card::ChapterCard, item_card::ItemCard, start_card::StartCard,
};

#[derive(Debug, Component)]
pub struct Castle {
    // chapter_cards
    castle_deck: Vec<ChapterCard>,
    boss_card: BossCard,
    start_card: StartCard,
    item_deck: Vec<ItemCard>,
}
