use bevy::prelude::Mesh;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

use bevy::render::mesh::shape::Cube;

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
        const UV_1: f32 = 1. / 3.;
        const UV_2: f32 = 2. / 3.;
        const UV_3: f32 = 1.;

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
                let normal = raw_to_normal(raw);

                points.push(normal.map(|e| e * r));
                normals.push(normal);
                uvs.push([uv_scale(x), UV_3 - uv_scale(y)]);
            }
        }
        // Right
        for iz in 0..n {
            let z = 1. - (iz as f32) * sep;
            for iy in 0..n {
                let y = (iy as f32) * sep - 1.;

                let raw = [1., y, z];
                let normal = raw_to_normal(raw);

                points.push(normal.map(|e| e * r));
                normals.push(normal);
                uvs.push([UV_1 + uv_scale(z), UV_2 + uv_scale(y)]);
            }
        }
        // Back
        for ix in 0..n {
            let x = 1. - (ix as f32) * sep;
            for iy in 0..n {
                let y = (iy as f32) * sep - 1.;

                let raw = [x, y, -1.];
                let normal = raw_to_normal(raw);

                points.push(normal.map(|e| e * r));
                normals.push(normal);
                uvs.push([uv_scale(x), uv_scale(y)]);
            }
        }
        // Left
        for iz in 0..n {
            let z = (iz as f32) * sep - 1.;
            for iy in 0..n {
                let y = (iy as f32) * sep - 1.;

                let raw = [-1., y, z];
                let normal = raw_to_normal(raw);

                points.push(normal.map(|e| e * r));
                normals.push(normal);
                uvs.push([UV_1 + uv_scale(z), UV_1 - uv_scale(y)]);
            }
        }
        // Top
        for ix in 0..n {
            let x = (ix as f32) * sep - 1.;
            for iz in 0..n {
                let z = 1. - (iz as f32) * sep;

                let raw = [x, 1., z];
                let normal = raw_to_normal(raw);

                points.push(normal.map(|e| e * r));
                normals.push(normal);
                uvs.push([uv_scale(x), UV_1 + uv_scale(z)]);
            }
        }
        // Bottom
        for ix in 0..n {
            let x = (ix as f32) * sep - 1.;
            for iz in 0..n {
                let z = (iz as f32) * sep - 1.;

                let raw = [x, -1., z];
                let normal = raw_to_normal(raw);

                points.push(normal.map(|e| e * r));
                normals.push(normal);
                uvs.push([UV_1 + uv_scale(z), UV_1 + uv_scale(x)]);
            }
        }

        for f in 0..FACE_CNT as u32 {
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

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        mesh
    }
}

// convert cube vertex [-1, 1] to normal
// from https://catlikecoding.com/unity/tutorials/cube-sphere/
fn raw_to_normal(i: [f32; 3]) -> [f32; 3] {
    let x = i[0];
    let y = i[1];
    let z = i[2];

    let x2 = x.powf(2.);
    let y2 = y.powf(2.);
    let z2 = z.powf(2.);

    let nx = x * (1. - y2 / 2. - z2 / 2. + y2 * z2 / 3.).sqrt();
    let ny = y * (1. - x2 / 2. - z2 / 2. + x2 * z2 / 3.).sqrt();
    let nz = z * (1. - x2 / 2. - y2 / 2. + x2 * y2 / 3.).sqrt();

    return [nx, ny, nz];
}

// convert from [-1, 1] to [0, 1/3]
fn uv_scale(i: f32) -> f32 {
    (i + 1.) / 6.
}
