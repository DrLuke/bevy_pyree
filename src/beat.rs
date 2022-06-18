//! Components and systems related to beat signals

use bevy::prelude::*;

/// Resource that counts how many beats have been received
#[derive(Component)]
pub struct BeatCounter {
    // Counter is increased on every beat
    count: u64,
}

/// Event that is emitted when a beat occurs
pub struct BeatEvent{
    // Value from BeatCounter
    count: u64
}