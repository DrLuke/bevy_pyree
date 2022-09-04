use bevy::prelude::*;

/// Deck contains 4 slots for Clips/FX to mix together
#[derive(Default)]
pub struct Deck {
    pub slots: [Option<Entity>; 4],
}