//! Resource that keeps track of beat detection signals

use bevy::prelude::*;

/// A resource that keeps track of beat signals
#[derive(Component)]
pub struct Beat {
    // True if a beat was received at beginning of frame
    beat: bool,
    // A counter that is increased on every beat
    beat_counter: u64,
}

impl Beat {
    /// Returns true if a beat occured at beginning of frame
    pub fn get_beat(&self) -> bool { self.beat }
    /// Returns the beat counter
    pub fn get_beat_counter(&self) -> u64 { self.beat_counter }
}