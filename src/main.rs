mod image;
mod util;
mod vector3;

use image::{gen_ppm, Pixel};
use vector3::Vector;

const IMG_SIZE: usize = 255;

fn main() {
    let mut image = Vec::new();

    for h in 0..IMG_SIZE {
        image.push(Vec::new());

        print!(
            "Rendering... {}%\r",
            ((h as f32 / IMG_SIZE as f32) * 100.0) as u8
        );

        for w in 0..IMG_SIZE {
            image[h].push(Pixel {
                r: h as u8,
                g: 255 - (h as u8),
                b: w as u8,
            });
        }
    }

    println!("\nFinished rendering.");
    gen_ppm(image);
}
