use bevy::prelude::Mesh;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

use bevy::prelude::shape::Cube;

#[derive(Debug, Clone, Copy)]
pub struct CubeSphere {
    pub radius: f32,
    pub resolution: usize,
}

impl Default for CubeSphere {
    fn default() -> Self {
        CubeSphere {
            radius: 1.0,   // radius
            resolution: 8, // number of vertices per side
        }
    }
}

impl From<CubeSphere> for Mesh {
    fn from(sphere: CubeSphere) -> Self {
        if sphere.resolution < 2 || sphere.resolution > 103 {
            panic!(
                "Cannot create an cube sphere with resolution {}. (Limited to 65535 vertices)",
                sphere.resolution
            );
        }

        const FACE_CNT: usize = 6;

        let mut points: Vec<[f32; 3]> = Vec::with_capacity(FACE_CNT * sphere.resolution.pow(2));
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(FACE_CNT * sphere.resolution.pow(2));
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(FACE_CNT * sphere.resolution.pow(2));
        let mut indices: Vec<u32> =
            Vec::with_capacity(FACE_CNT * 2 * (sphere.resolution - 1).pow(2));

        let r = sphere.radius;
        let n = sphere.resolution as u32;
        let sep = 2_f32 / (n as f32 - 1.);

        // Front
        for ix in 0..n {
            let x = (ix as f32) * sep - 1.;
            for iy in 0..n {
                let y = (iy as f32) * sep - 1.;

                let raw = [x, y, 1.];

                // TODO: fix
                points.push(raw);
                normals.push([0.0, 0.0, 1.0]);
                uvs.push([x, y]);
            }
        }
        // // Right
        // for iz in 0..n {
        //     let z = 1. - (iz as f32) * sep;
        //     for iy in 0..n {
        //         let y = (iy as f32) * sep - 1.;
        //         let raw = [1., y, z];

        //         // TODO: fix
        //         points.push(raw);
        //         normals.push([1.0, 0.0, 0.0]);
        //         uvs.push([z, y]);
        //     }
        // }
        // // Back
        // for ix in 0..n {
        //     let x = 1. - (ix as f32) * sep;
        //     for iy in 0..n {
        //         let y = (iy as f32) * sep - 1.;
        //         let raw = [x, y, -1.];

        //         // TODO: fix
        //         points.push(raw);
        //         normals.push([0.0, 0.0, -1.0]);
        //         uvs.push([x, y]);
        //     }
        // }
        // // Left
        // for iz in 0..n {
        //     let z = (iz as f32) * sep - 1.;
        //     for iy in 0..n {
        //         let y = (iy as f32) * sep - 1.;
        //         let raw = [-1., y, z];

        //         // TODO: fix
        //         points.push(raw);
        //         normals.push([-1.0, 0.0, 0.0]);
        //         uvs.push([z, y]);
        //     }
        // }

        for f in 0..2 {
            for x in 0..n - 1 {
                for y in 0..n - 1 {
                    let i = f * n.pow(2) + y * n + x;

                    // counter-clockwise = front
                    indices.push(i);
                    indices.push(i + n);
                    indices.push(i + 1);

                    indices.push(i + 1);
                    indices.push(i + n);
                    indices.push(i + n + 1);
                }
            }
        }

        // // Right

        //     // Top
        //     ([sp.min_x, sp.min_y, sp.max_z], [0., 0., 1.0], [0., 0.]),
        //     ([sp.max_x, sp.min_y, sp.max_z], [0., 0., 1.0], [1.0, 0.]),
        //     ([sp.max_x, sp.max_y, sp.max_z], [0., 0., 1.0], [1.0, 1.0]),
        //     ([sp.min_x, sp.max_y, sp.max_z], [0., 0., 1.0], [0., 1.0]),
        //     // Bottom
        //     ([sp.min_x, sp.max_y, sp.min_z], [0., 0., -1.0], [1.0, 0.]),
        //     ([sp.max_x, sp.max_y, sp.min_z], [0., 0., -1.0], [0., 0.]),
        //     ([sp.max_x, sp.min_y, sp.min_z], [0., 0., -1.0], [0., 1.0]),
        //     ([sp.min_x, sp.min_y, sp.min_z], [0., 0., -1.0], [1.0, 1.0]),
        //     // Right
        //     ([sp.max_x, sp.min_y, sp.min_z], [1.0, 0., 0.], [0., 0.]),
        //     ([sp.max_x, sp.max_y, sp.min_z], [1.0, 0., 0.], [1.0, 0.]),
        //     ([sp.max_x, sp.max_y, sp.max_z], [1.0, 0., 0.], [1.0, 1.0]),
        //     ([sp.max_x, sp.min_y, sp.max_z], [1.0, 0., 0.], [0., 1.0]),
        //     // Left
        //     ([sp.min_x, sp.min_y, sp.max_z], [-1.0, 0., 0.], [1.0, 0.]),
        //     ([sp.min_x, sp.max_y, sp.max_z], [-1.0, 0., 0.], [0., 0.]),
        //     ([sp.min_x, sp.max_y, sp.min_z], [-1.0, 0., 0.], [0., 1.0]),
        //     ([sp.min_x, sp.min_y, sp.min_z], [-1.0, 0., 0.], [1.0, 1.0]),
        //     // Front
        //     ([sp.max_x, sp.max_y, sp.min_z], [0., 1.0, 0.], [1.0, 0.]),
        //     ([sp.min_x, sp.max_y, sp.min_z], [0., 1.0, 0.], [0., 0.]),
        //     ([sp.min_x, sp.max_y, sp.max_z], [0., 1.0, 0.], [0., 1.0]),
        //     ([sp.max_x, sp.max_y, sp.max_z], [0., 1.0, 0.], [1.0, 1.0]),
        //     // Back
        //     ([sp.max_x, sp.min_y, sp.max_z], [0., -1.0, 0.], [0., 0.]),
        //     ([sp.min_x, sp.min_y, sp.max_z], [0., -1.0, 0.], [1.0, 0.]),
        //     ([sp.min_x, sp.min_y, sp.min_z], [0., -1.0, 0.], [1.0, 1.0]),
        //     ([sp.max_x, sp.min_y, sp.min_z], [0., -1.0, 0.], [0., 1.0]),

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        mesh
    }
}

// convert cube vertex [-1, 1] to normal
fn to_normal(i: [f32; 3]) -> [f32; 3] {
    todo!()
}
