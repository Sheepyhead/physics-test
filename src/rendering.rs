use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

pub struct Rendering;

impl Plugin for Rendering {
    fn build(&self, _app: &mut App) {}
}

pub struct Pyramid {
    pub base_side_length: f32,
    pub height: f32,
}

impl From<Pyramid> for Mesh {
    fn from(pyramid: Pyramid) -> Self {
        let base_he = pyramid.base_side_length / 2.0;
        let height = pyramid.height;
        let vertices = &[
            // Bottom
            ([-base_he, 0.0, base_he], [0., 0., -1.0], [1.0, 0.]),
            ([-base_he, 0.0, -base_he], [0., 0., -1.0], [1.0, 1.0]),
            ([base_he, 0.0, -base_he], [0., 0., -1.0], [0., 1.0]),
            ([base_he, 0.0, base_he], [0., 0., -1.0], [0., 0.]),
            // Right
            ([base_he, 0.0, -base_he], [height, base_he, 0.], [0., 0.]),
            ([0.0, height, 0.0], [height, base_he, 0.], [1.0, 1.0]),
            ([base_he, 0.0, base_he], [height, base_he, 0.], [1.0, 0.]),
            // Left
            ([-base_he, 0.0, base_he], [-height, -base_he, 0.], [1.0, 0.]),
            ([0.0, height, 0.0], [-height, -base_he, 0.], [1.0, 1.0]),
            ([-base_he, 0.0, -base_he], [-height, -base_he, 0.], [0., 0.]),
            // Front
            ([-base_he, 0.0, base_he], [-height, base_he, 0.], [0., 0.]),
            ([base_he, 0.0, base_he], [-height, base_he, 0.], [1.0, 0.]),
            ([0.0, height, 0.0], [-height, base_he, 0.], [1.0, 1.0]),
            // Back
            ([-base_he, 0.0, -base_he], [-height, base_he, 0.], [0., 0.]),
            ([0.0, height, 0.0], [-height, base_he, 0.], [1.0, 1.0]),
            ([base_he, 0.0, -base_he], [-height, base_he, 0.], [1.0, 0.]),
        ];

        let mut positions = Vec::with_capacity(16);
        let mut normals = Vec::with_capacity(16);
        let mut uvs = Vec::with_capacity(16);

        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        #[rustfmt::skip]
        let indices = Indices::U32(vec![
            0, 1, 2, 2, 3, 0,   // bottom
            4, 5, 6,            // right
            7, 8, 9,            // left
            10, 11, 12,         // front
            13, 14, 15,         // back
        ]);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(indices));
        mesh
    }
}
