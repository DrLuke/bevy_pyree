use std::borrow::Cow;

use bevy::{
    prelude::*,
    ecs::component::Component,
    render::{
        camera::{ActiveCameras},
        pass::{
            LoadOp, Operations, PassDescriptor, RenderPassColorAttachmentDescriptor,
            RenderPassDepthStencilAttachmentDescriptor, TextureAttachment,
        },
        render_graph::{
            base::{node::MAIN_PASS},
            CameraNode, Node, PassNode, RenderGraph, ResourceSlotInfo,
        },
        renderer::{RenderResourceId, RenderResourceType},
        texture::{
            Extent3d, SamplerDescriptor, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsage, SAMPLER_ASSET_INDEX, TEXTURE_ASSET_INDEX,
        },
    }
};

pub const TEXTURE_NODE: &str = "texure_node";
pub const DEPTH_TEXTURE_NODE: &str = "depth_texure_node";
pub const FIRST_PASS: &str = "first_pass";

pub struct TextureNode {
    pub texture_descriptor: TextureDescriptor,
    pub sampler_descriptor: Option<SamplerDescriptor>,
    pub handle: Option<Handle<Texture>>,
}

impl TextureNode {
    pub const TEXTURE: &'static str = "texture";

    pub fn new(
        texture_descriptor: TextureDescriptor,
        sampler_descriptor: Option<SamplerDescriptor>,
        handle: Option<Handle<Texture>>,
    ) -> Self {
        Self {
            texture_descriptor,
            sampler_descriptor,
            handle,
        }
    }
}

impl Node for TextureNode {
    fn output(&self) -> &[ResourceSlotInfo] {
        static OUTPUT: &[ResourceSlotInfo] = &[ResourceSlotInfo {
            name: Cow::Borrowed(TextureNode::TEXTURE),
            resource_type: RenderResourceType::Texture,
        }];
        OUTPUT
    }

    fn update(
        &mut self,
        _world: &World,
        render_context: &mut dyn bevy::render::renderer::RenderContext,
        _input: &bevy::render::render_graph::ResourceSlots,
        output: &mut bevy::render::render_graph::ResourceSlots,
    ) {
        if output.get(0).is_none() {
            let render_resource_context = render_context.resources_mut();
            let texture_id = render_resource_context.create_texture(self.texture_descriptor);
            if let Some(handle) = &self.handle {
                render_resource_context.set_asset_resource_untyped(
                    handle.clone_weak_untyped(),
                    RenderResourceId::Texture(texture_id),
                    TEXTURE_ASSET_INDEX,
                );
                if let Some(sampler_descriptor) = self.sampler_descriptor {
                    let sampler_id = render_resource_context.create_sampler(&sampler_descriptor);
                    render_resource_context.set_asset_resource_untyped(
                        handle.clone_weak_untyped(),
                        RenderResourceId::Sampler(sampler_id),
                        SAMPLER_ASSET_INDEX,
                    );
                }
            }
            output.set(0, RenderResourceId::Texture(texture_id));
        }
    }
}

pub struct RenderToTextureGraph {
    pub texture_handle: Handle<Texture>,
    pub name: &'static str,
    pub camera_name: &'static str,
    pub width: u32,
    pub height: u32
}

pub trait RenderToTextureGraphBuilder {
    fn add_render_to_texture_graph<T: Component>(&mut self, active_cameras: &mut ActiveCameras, render_graph_info: &RenderToTextureGraph) -> &mut Self;
}

impl RenderToTextureGraphBuilder for RenderGraph {
    fn add_render_to_texture_graph<T: Component>(&mut self, active_cameras: &mut ActiveCameras, render_graph_info: &RenderToTextureGraph) -> &mut Self {
        let mut pass_node = PassNode::<&T>::new(PassDescriptor {
            color_attachments: vec![RenderPassColorAttachmentDescriptor {
                attachment: TextureAttachment::Input("color_attachment".to_string()),
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::rgb(0.8, 0.2, 0.3)),
                    store: true,
                },
            }],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachmentDescriptor {
                attachment: TextureAttachment::Input("depth".to_string()),
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
            sample_count: 1,
        });
        pass_node.add_camera(render_graph_info.camera_name);

        self.add_node([render_graph_info.name, FIRST_PASS].concat(), pass_node);
        self.add_system_node(render_graph_info.camera_name, CameraNode::new(render_graph_info.camera_name));

        active_cameras.add(render_graph_info.camera_name);
        self.add_node_edge(render_graph_info.camera_name, [render_graph_info.name, FIRST_PASS].concat()).unwrap();

        self.add_node(
            [render_graph_info.name, TEXTURE_NODE].concat(),
            TextureNode::new(
                TextureDescriptor {
                    size: Extent3d::new(render_graph_info.width, render_graph_info.height, 1),
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format: Default::default(),
                    usage: TextureUsage::OUTPUT_ATTACHMENT | TextureUsage::SAMPLED,
                },
                Some(SamplerDescriptor::default()),
                Some(render_graph_info.texture_handle.clone()),
            ),
        );

        self.add_node(
            [render_graph_info.name, DEPTH_TEXTURE_NODE].concat(),
            TextureNode::new(
                TextureDescriptor {
                    size: Extent3d::new(render_graph_info.width, render_graph_info.height, 1),
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::Depth32Float,
                    usage: TextureUsage::OUTPUT_ATTACHMENT | TextureUsage::SAMPLED,
                },
                None,
                None,
            ),
        );

        self.add_node_edge([render_graph_info.name, TEXTURE_NODE].concat(), [render_graph_info.name, FIRST_PASS].concat()).unwrap();
        self.add_slot_edge(
            [render_graph_info.name, TEXTURE_NODE].concat(),
            TextureNode::TEXTURE,
            [render_graph_info.name, FIRST_PASS].concat(),
            "color_attachment",
        )
        .unwrap();
        self.add_slot_edge(
            [render_graph_info.name, DEPTH_TEXTURE_NODE].concat(),
            TextureNode::TEXTURE,
            [render_graph_info.name, FIRST_PASS].concat(),
            "depth",
        )
        .unwrap();
        self.add_node_edge([render_graph_info.name, FIRST_PASS].concat(), MAIN_PASS).unwrap();
        self.add_node_edge("transform", [render_graph_info.name, FIRST_PASS].concat()).unwrap();
        self
    }
}