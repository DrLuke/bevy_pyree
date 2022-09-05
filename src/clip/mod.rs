use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

mod clip;
mod clip_render;
mod deck;

pub use clip::Clip;
pub use clip_render::ClipRender;
pub use clip_render::setup_clip_renderer;
pub use deck::Deck2;