use bevy::prelude::*;
use crate::beat::osc_receiver::osc_beat_receiver_system;
use crate::beat::{BeatCounter, BeatEvent, OscBeatReceiver};
use bevy_rosc::{SingleAddressOscMethod};

pub struct OscBeatReceiverPlugin {
    /// Address at which the osc beat signal comes in
    pub address: String,
}

impl Default for OscBeatReceiverPlugin {
    fn default() -> Self {
        Self {
            address: "/beat".to_owned()
        }
    }
}

impl Plugin for OscBeatReceiverPlugin {
    fn build(&self, app: &mut App) {
        let osc_address = self.address.clone();

        app
            .insert_resource(BeatCounter::default())
            .add_event::<BeatEvent>()
            .add_startup_system(move |mut commands: Commands| {
                commands.spawn((
                    OscBeatReceiver {},
                    SingleAddressOscMethod::new(osc_address.clone()).unwrap()
                ));
            })
            .add_system(osc_beat_receiver_system)
        ;
    }
}