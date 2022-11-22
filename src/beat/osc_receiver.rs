///! Receive beat signals via OSC

use bevy::prelude::*;
use bevy_rosc::OscMethod;
use crate::beat::{BeatCounter, BeatEvent};

#[derive(Component)]
struct OscBeatReceiver;

/// Whenever a message is received at the provided beat address, increment the beat counter and send an event
fn osc_beat_receiver_system(
    mut beat_counter: ResMut<BeatCounter>,
    mut beat_writer: EventWriter<BeatEvent>,
    mut query: Query<(&OscBeatReceiver, &OscMethod), Changed<OscMethod>>,
) {
    for (_, osc_method) in query.iter_mut() {
        while let Some(_) = osc_method.get_message() {
            beat_counter.count += 1;
            beat_writer.send(BeatEvent { count: beat_counter.count });
        }
    }
}

pub struct OscBeatReceiverPlugin {
    /// Address at which the osc beat signal comes in
    address: str,
}

impl Plugin for OscBeatReceiverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(|mut commands: Commands| {
                commands.spawn((
                    OscBeatReceiver {},
                    OscMethod::new(vec![&self.address]).unwrap()
                ));
            });
    }
}

