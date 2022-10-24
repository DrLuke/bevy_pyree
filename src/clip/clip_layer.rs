use bevy::prelude::*;

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
    blend: f32,
}

impl ClipLayer {
    pub fn new(layer: u8) -> Self {
        ClipLayer {
            layer,
            clips: vec![None; u8::MAX as usize],
            render_targets: vec![None; u8::MAX as usize],
            active_clip: 0,
            blend: 0.0,
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