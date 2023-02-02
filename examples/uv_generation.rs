use bevy_cube_sphere::CubeSphere;
use image::{Rgba, RgbaImage};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

fn main() {
    let mut image = RgbaImage::new(WIDTH, HEIGHT);

    for ix in 0..WIDTH {
        for iy in 0..HEIGHT {
            let x = ix as f32 / WIDTH as f32;
            let y = 1. - iy as f32 / WIDTH as f32;

            let face = CubeSphere::uv_to_point(&[x, y]);

            match face {
                Some(_) => *image.get_pixel_mut(ix, iy) = Rgba([0, 0, 255, 255]),
                None => *image.get_pixel_mut(ix, iy) = Rgba([0, 0, 0, 0]),
            }
        }
    }

    // write it out to a file
    image.save("output.png").unwrap();
}
