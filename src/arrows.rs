use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
pub fn arrow(shaft_ratio: f32, shaft_width: f32, head_width: f32 ) -> Mesh {

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let head_start = 2.0*shaft_ratio - 1.0;
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-1.0, -shaft_width/2.0, 0.0],
            [head_start, -shaft_width/2.0, 0.0],
            [head_start, -head_width/2.0, 0.0],
            [1.0, 0.0, 0.0],
            [head_start, head_width/2.0, 0.0],
            [head_start, shaft_width/2.0, 0.0],
            [-1.0, shaft_width/2.0, 0.0],
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

    mesh.set_indices(Some(Indices::U32(vec![0, 1, 6, 1, 5, 6, 2, 3, 4])));
    mesh
}

