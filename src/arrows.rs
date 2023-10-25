use bevy::prelude::{Mesh};
use bevy::render::mesh::Indices;
use bevy::sprite::{MaterialMesh2dBundle};
use bevy::render::render_resource::PrimitiveTopology;
pub fn arrow() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-1.0, -0.05, 0.0],
            [0., -0.05, 0.0],
            [0., -0.15, 0.0],
            [1.0, 0.0, 0.0],
            [0., 0.15, 0.0],
            [0., 0.05, 0.0],
            [-1.0, 0.05, 0.0],
        ],
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [0.0, 0.0],
            [0.5, 0.0],
            [0.5, 0.25],
            [1.0, 0.5],
            [0.5, 0.75],
            [0.5, 1.0],
            [0.0, 1.0],
        ],
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
    );

    mesh.set_indices(
        Some(Indices::U32(vec![
            0, 1, 6,
            1, 5, 6,
            2, 3, 4
    ])));
    mesh
}