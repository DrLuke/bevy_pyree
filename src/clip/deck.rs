use bevy::prelude::*;

/// Simple crossfader with 2 slots
#[derive(Default)]
pub struct Deck2 {
    pub slots: [Option<Entity>; 2],
}