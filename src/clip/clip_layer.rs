use std::str::FromStr;
use bevy::prelude::*;
use bevy_rosc::OscMethod;
use rosc::address::OscAddress;
use rosc::OscType;

#[derive(PartialEq, Debug, Clone)]
pub enum BlendMode {
    // f = b*x
    Normal,
    // f = (1-x)*a + x*b
    Mix,
    // f = a*b
    Multiply,
    // f = 1 - (1-a)*(1-b)
    Screen,
    // f = a+b*x
    Add,
    // f = a-b*x
    Subtract,
    // f = b-a*x
    Difference,
}

impl FromStr for BlendMode {
    type Err = ();

    fn from_str(input: &str) -> Result<BlendMode, Self::Err> {
        match input {
            "normal" => Ok(BlendMode::Normal),
            "mix" => Ok(BlendMode::Normal),
            "multiply" => Ok(BlendMode::Normal),
            "screen" => Ok(BlendMode::Normal),
            "add" => Ok(BlendMode::Normal),
            "subtract" => Ok(BlendMode::Normal),
            "difference" => Ok(BlendMode::Normal),
            _ => Err(()),
        }
    }
}


/// A Clip layer contains clips. One of those clips is active and selected to be displayed to the output at any given time
#[derive(Component)]
pub struct ClipLayer {
    /// The index of this layer
    pub layer: u8,
    /// All clip entities in this layer
    clips: Vec<Option<Entity>>,
    /// The render targets of the clips in this layer
    render_targets: Vec<Option<Handle<Image>>>,
    /// The selected clip, if any
    active_clip: u8,
    /// How much this layer's clip is supposed to be blended into the output
    pub blend: f32,
    /// How to blend
    pub blend_mode: BlendMode,
}

impl ClipLayer {
    pub fn new(layer: u8) -> Self {
        ClipLayer {
            layer,
            clips: vec![None; u8::MAX as usize],
            render_targets: vec![None; u8::MAX as usize],
            active_clip: 0,
            blend: 0.5,
            blend_mode: BlendMode::Normal,
        }
    }

    pub fn add_clip(&mut self, index: u8, clip_entity: Entity, render_target: Handle<Image>) {
        self.clips.insert(index as usize, Some(clip_entity));
        self.render_targets.insert(index as usize, Some(render_target));
    }
    pub fn remove_clip(&mut self, index: u8) {
        self.clips.insert(index as usize, None);
        self.render_targets.insert(index as usize, None);
    }
    pub fn get_active_render_target(&mut self) -> Option<Handle<Image>> { self.render_targets[self.active_clip as usize].clone() }
    pub fn set_active_clip(&mut self, index: u8) { self.active_clip = index }
    pub fn get_active_clip(&self) -> u8 { self.active_clip }
    pub fn get_clips(&self) -> Vec<Option<Entity>> { self.clips.clone() }
    pub fn get_render_targets(&self) -> Vec<Option<Handle<Image>>> { self.render_targets.iter().map(|x| x.clone()).collect() }
}

impl OscMethod for ClipLayer {
    fn get_addresses(&self) -> Vec<OscAddress> {
        vec![
            OscAddress::new(format!("/clip_layer/{}/blend", self.layer)).unwrap(),
            OscAddress::new(format!("/clip_layer/{}/blend_mode", self.layer)).unwrap(),
            OscAddress::new(format!("/clip_layer/{}/active_clip", self.layer)).unwrap(),
        ]
    }

    fn receive_message(&mut self, osc_message: rosc::OscMessage) {
        let addr = osc_message.addr.clone();
        if addr.ends_with("/blend") && osc_message.args.len() == 1 {
            if let OscType::Float(blend) = osc_message.args[0] { self.blend = blend }
        } else if addr.ends_with("/blend_mode") && osc_message.args.len() == 1 {
            if let OscType::String(blend_mode) = &osc_message.args[0] {
                if let Ok(new_blend_mode) = BlendMode::from_str(blend_mode.as_str()) {
                    self.blend_mode = new_blend_mode;
                } else {
                    warn!("Clip Layer {} received unknown blend mode: {:?}", self.layer, blend_mode)
                }
            }
        } else if addr.ends_with("/active_clip") && osc_message.args.len() == 1 {
            if let OscType::Int(clip) = osc_message.args[0] {
                if let 0..=255 = clip {
                    // Only set if
                    self.set_active_clip(clip as u8)
                }
            }
        } else {
            warn!("Clip Layer {} received unprocessed OSC message: {:?}", self.layer, osc_message)
        }
    }
}