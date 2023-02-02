use bevy::prelude::Mesh;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

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

#[derive(Debug, Clone, Copy)]
enum CubeFace {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

const CUBE_FACE_COUNT: usize = 6;

impl From<CubeSphere> for Mesh {
    fn from(sphere: CubeSphere) -> Self {
        if sphere.resolution < 2 || sphere.resolution > 103 {
            panic!(
                "Cannot create an cube sphere with resolution {}. (Limited to 65535 vertices)",
                sphere.resolution
            );
        }

        let total_point_count = CUBE_FACE_COUNT * sphere.resolution.pow(2);
        let mut points: Vec<[f32; 3]> = Vec::with_capacity(total_point_count);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(total_point_count);
        let mut tangents: Vec<[f32; 4]> = Vec::with_capacity(total_point_count);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(total_point_count);

        let n = sphere.resolution as u32;
        let sep = 2_f32 / (n as f32 - 1.);

        // Front
        for ix in 0..n {
            let x = (ix as f32) * sep - 1.;
            for iy in 0..n {
                let y = (iy as f32) * sep - 1.;

                let unit_cube_point = [x, y, 1.];
                insert_attributes(
                    &unit_cube_point,
                    sphere.radius,
                    CubeFace::Front,
                    &mut points,
                    &mut normals,
                    &mut tangents,
                    &mut uvs,
                );
            }
        }
        // Back
        for ix in 0..n {
            let x = 1. - (ix as f32) * sep;
            for iy in 0..n {
                let y = (iy as f32) * sep - 1.;

                let unit_cube_point = [x, y, -1.];
                insert_attributes(
                    &unit_cube_point,
                    sphere.radius,
                    CubeFace::Back,
                    &mut points,
                    &mut normals,
                    &mut tangents,
                    &mut uvs,
                );
            }
        }
        // Left
        for iz in 0..n {
            let z = (iz as f32) * sep - 1.;
            for iy in 0..n {
                let y = (iy as f32) * sep - 1.;

                let unit_cube_point = [-1., y, z];
                insert_attributes(
                    &unit_cube_point,
                    sphere.radius,
                    CubeFace::Left,
                    &mut points,
                    &mut normals,
                    &mut tangents,
                    &mut uvs,
                );
            }
        }
        // Right
        for iz in 0..n {
            let z = 1. - (iz as f32) * sep;
            for iy in 0..n {
                let y = (iy as f32) * sep - 1.;

                let unit_cube_point = [1., y, z];
                insert_attributes(
                    &unit_cube_point,
                    sphere.radius,
                    CubeFace::Right,
                    &mut points,
                    &mut normals,
                    &mut tangents,
                    &mut uvs,
                );
            }
        }
        // Top
        for ix in 0..n {
            let x = (ix as f32) * sep - 1.;
            for iz in 0..n {
                let z = 1. - (iz as f32) * sep;

                let unit_cube_point = [x, 1., z];
                insert_attributes(
                    &unit_cube_point,
                    sphere.radius,
                    CubeFace::Top,
                    &mut points,
                    &mut normals,
                    &mut tangents,
                    &mut uvs,
                );
            }
        }
        // Bottom
        for ix in 0..n {
            let x = (ix as f32) * sep - 1.;
            for iz in 0..n {
                let z = (iz as f32) * sep - 1.;

                let unit_cube_point = [x, -1., z];
                insert_attributes(
                    &unit_cube_point,
                    sphere.radius,
                    CubeFace::Bottom,
                    &mut points,
                    &mut normals,
                    &mut tangents,
                    &mut uvs,
                );
            }
        }

        // construct faces
        let total_index_count = CUBE_FACE_COUNT * 2 * (sphere.resolution - 1).pow(2);
        let mut indices: Vec<u32> = Vec::with_capacity(total_index_count);
        insert_indices(sphere.resolution as u32, &mut indices);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_TANGENT, tangents);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        mesh
    }
}

// convert cube vertex [-1, 1] to sphere vertex [-1, 1]
// from https://catlikecoding.com/unity/tutorials/cube-sphere/
fn unit_cube_point_to_unit_sphere_point(pt: &[f32; 3]) -> [f32; 3] {
    let x = pt[0];
    let y = pt[1];
    let z = pt[2];

    let x2 = x.powi(2);
    let y2 = y.powi(2);
    let z2 = z.powi(2);

    let nx = x * (1. - y2 / 2. - z2 / 2. + y2 * z2 / 3.).sqrt();
    let ny = y * (1. - x2 / 2. - z2 / 2. + x2 * z2 / 3.).sqrt();
    let nz = z * (1. - x2 / 2. - y2 / 2. + x2 * y2 / 3.).sqrt();

    return [nx, ny, nz];
}

fn unit_sphere_point_to_sphere_point(pt: &[f32; 3], r: f32) -> [f32; 3] {
    return pt.map(|e| e * r);
}

fn unit_sphere_point_to_normal(pt: &[f32; 3]) -> [f32; 3] {
    return *pt;
}

fn unit_sphere_point_to_tangent(pt: &[f32; 3], f: CubeFace) -> [f32; 4] {
    let normal = unit_sphere_point_to_normal(pt);
    let other = match f {
        CubeFace::Front | CubeFace::Back | CubeFace::Left | CubeFace::Right => {
            [0_f32, 1_f32, 0_f32]
        }
        CubeFace::Top | CubeFace::Bottom => [0_f32, 0_f32, 1_f32],
    };

    let x = normal[0];
    let y = normal[1];
    let z = normal[2];

    let u = other[0];
    let v = other[1];
    let w = other[2];

    let a = w * y - v * z;
    let b = u * z - w * x;
    let c = v * w - u * y;
    let norm = (a.powi(2) + b.powi(2) + c.powi(2)).sqrt();

    [a / norm, b / norm, c / norm, 1.]
}

// http://hydra.nat.uni-magdeburg.de/packing/csq/csq.html
const UV_SPHERE_RADIUS: f32 = 0.1876806;
const UV_COORDINATE_0: [f32; 2] = [0.0000000, 0.0000000];
const UV_COORDINATE_1: [f32; 2] = [0.6246388, 0.0000000];
const UV_COORDINATE_2: [f32; 2] = [0.3123194, 0.2082129];
const UV_COORDINATE_3: [f32; 2] = [0.0000000, 0.4164259];
const UV_COORDINATE_4: [f32; 2] = [0.6246388, 0.4164259];
const UV_COORDINATE_5: [f32; 2] = [0.3123194, 0.6246388];

fn unit_sphere_point_to_uv(pt: &[f32; 3], f: CubeFace) -> [f32; 2] {
    let x = pt[0];
    let y = pt[1];
    let z = pt[2];

    fn scale(i: f32) -> f32 {
        (i + 1.) * UV_SPHERE_RADIUS
    }

    fn flip(i: f32) -> f32 {
        UV_SPHERE_RADIUS * 2. - i
    }

    match f {
        CubeFace::Front => [
            UV_COORDINATE_0[0] + scale(x),
            1. - (UV_COORDINATE_0[1] + scale(y)),
        ],
        CubeFace::Back => [
            UV_COORDINATE_3[0] + flip(scale(x)),
            1. - (UV_COORDINATE_3[1] + scale(y)),
        ],
        CubeFace::Left => [
            UV_COORDINATE_2[0] + scale(z),
            1. - (UV_COORDINATE_2[1] + scale(y)),
        ],
        CubeFace::Right => [
            UV_COORDINATE_5[0] + flip(scale(z)),
            1. - (UV_COORDINATE_5[1] + scale(y)),
        ],
        CubeFace::Top => [
            UV_COORDINATE_1[0] + scale(x),
            1. - (UV_COORDINATE_1[1] + flip(scale(z))),
        ],
        CubeFace::Bottom => [
            UV_COORDINATE_4[0] + scale(x),
            1. - (UV_COORDINATE_4[1] + scale(z)),
        ],
    }
}

fn insert_attributes(
    unit_cube_point: &[f32; 3],
    radius: f32,
    face: CubeFace,
    points: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    tangents: &mut Vec<[f32; 4]>,
    uvs: &mut Vec<[f32; 2]>,
) {
    let unit_sphere_point = unit_cube_point_to_unit_sphere_point(unit_cube_point);
    points.push(unit_sphere_point_to_sphere_point(
        &unit_sphere_point,
        radius,
    ));
    normals.push(unit_sphere_point_to_normal(&unit_sphere_point));
    tangents.push(unit_sphere_point_to_tangent(&unit_sphere_point, face));
    uvs.push(unit_sphere_point_to_uv(&unit_sphere_point, face))
}

fn insert_indices(n: u32, indices: &mut Vec<u32>) {
    let edge_count = n - 1;
    let face_count = CUBE_FACE_COUNT as u32;

    for f in 0..face_count as u32 {
        for x in 0..edge_count {
            for y in 0..edge_count {
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
}

impl CubeSphere {
    // unit sphere point to all possible uv location
    fn point_to_uvs(pt: &[f32; 3]) -> [[f32; 2]; 3] {
        todo!()
    }

    fn uv_to_point(uv: &[f32; 2]) -> Option<[f32; 3]> {
        todo!()
    }

    fn uv_to_normal(pt: &[f32; 3]) -> Option<[f32; 3]> {
        todo!()
    }

    fn uv_to_tangent(pt: &[f32; 3]) -> Option<[f32; 3]> {
        todo!()
    }
}
