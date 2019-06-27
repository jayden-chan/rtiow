mod image;
mod util;
mod vector3;

use image::{gen_ppm, Pixel};
use vector3::Vector;

const IMG_SIZE: usize = 4096;

fn main() {
    let mut image = Vec::new();

    for h in 0..IMG_SIZE {
        image.push(Vec::new());

        print!(
            "Rendering... {}%\r",
            ((h as f32 / IMG_SIZE as f32) * 100.0) as u8
        );

        for w in 0..IMG_SIZE {
            let c = ((h as f32 / IMG_SIZE as f32) * 255 as f32) as u8;
            let c2 = ((w as f32 / IMG_SIZE as f32) * 255 as f32) as u8;
            image[h].push(Pixel {
                r: c,
                g: 255 - c,
                b: c2,
            });
        }
    }

    println!("\nFinished rendering.");
    gen_ppm(image);
}
