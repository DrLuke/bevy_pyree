use std::str::FromStr;
use bevy::prelude::*;
use bevy_rosc::OscMethod;
use rosc::address::OscAddress;
use rosc::OscType;
use crate::clip::Clip;
use crate::clip::clip::ClipSelected;

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
            "mix" => Ok(BlendMode::Mix),
            "multiply" => Ok(BlendMode::Multiply),
            "screen" => Ok(BlendMode::Screen),
            "add" => Ok(BlendMode::Add),
            "subtract" => Ok(BlendMode::Subtract),
            "difference" => Ok(BlendMode::Difference),
            _ => Err(()),
        }
    }
}

impl TryFrom<i32> for BlendMode {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BlendMode::Normal),
            1 => Ok(BlendMode::Mix),
            2 => Ok(BlendMode::Multiply),
            3 => Ok(BlendMode::Screen),
            4 => Ok(BlendMode::Add),
            5 => Ok(BlendMode::Subtract),
            6 => Ok(BlendMode::Difference),
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
    pub fn get_render_targets(&self) -> Vec<Option<Handle<Image>>> { self.render_targets.to_vec() }
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
        debug!("Clip layer {} receives osc message: {:?}", self.layer, osc_message);
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
            } else if let OscType::Int(blend_mode) = osc_message.args[0] {
                if let Ok(new_blend_mode) = BlendMode::try_from(blend_mode) {
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

/// Propagate selected clip component to clips and all child entities
pub fn update_clip_selected_system(
    mut commands: Commands,
    clip_layer_query: Query<&ClipLayer, Changed<ClipLayer>>,
    selected_clips_query: Query<Entity, (With<Clip>, With<ClipSelected>)>,
    children_query: Query<&Children>,
) {
    for changed_clip_layer in &clip_layer_query {
        // First get all selected clips that are part of this clip layer
        let layer_clips: Vec<Option<Entity>> = changed_clip_layer.get_clips();
        let active_clip = layer_clips[changed_clip_layer.get_active_clip() as usize];
        let selected_layer_clips: Vec<Entity> = layer_clips.iter().filter_map(
            |clip_entity| selected_clips_query.get((*clip_entity)?).ok()
        ).collect();

        // Remove the SelectedClip component from all clips except for the active clip
        for selected_layer_clip in selected_layer_clips {
            if Some(selected_layer_clip) != active_clip {
                recursively_remove_clip_selected(&mut commands, &children_query, selected_layer_clip)
            }
        }
        // Selected clip doesn't have the SelectedClip component yet, so add it recursively
        if let Some(active_clip) = active_clip {
            if !selected_clips_query.contains(active_clip) {
                recursively_add_clip_selected(&mut commands, &children_query, active_clip)
            }
        }
    }
}


fn recursively_remove_clip_selected(
    commands: &mut Commands,
    children_query: &Query<&Children>,
    target: Entity,
) {
    commands.entity(target).remove::<ClipSelected>();
    for child_entity in children_query.iter_descendants(target) {
        recursively_remove_clip_selected(commands, children_query, child_entity);
    }
}

fn recursively_add_clip_selected(
    commands: &mut Commands,
    children_query: &Query<&Children>,
    target: Entity,
) {
    commands.entity(target).insert(ClipSelected);
    for child_entity in children_query.iter_descendants(target) {
        recursively_add_clip_selected(commands, children_query, child_entity);
    }
}