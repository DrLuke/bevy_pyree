
use bevy::render::mesh::{Mesh, Indices};
use bevy::render::pipeline::PrimitiveTopology;


#[derive(Debug, Copy, Clone, Default)]
pub struct FsQuad {
}

impl From<FsQuad> for Mesh {
    fn from(_plane: FsQuad) -> Self {
        let vertices = [
            ([1.0, -1.0, 0.0], [0.0, 0.0, -1.0], [1.0, 1.0]),
            ([1.0, 1.0, 0.0], [0.0, 0.0, -1.0], [1.0, -1.0]),
            ([-1.0, 1.0, 0.0], [0.0, 0.0, -1.0], [-1.0, -1.0]),
            ([-1.0, -1.0, 0.0], [0.0, 0.0, -1.0], [-1.0, 1.0]),
        ];

        let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3]);

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}