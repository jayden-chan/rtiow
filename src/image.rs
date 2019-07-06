use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub x: usize,
    pub y: usize,
}

/// Writes a 2 dimensional vector of Pixels to a P6 PPM file
pub fn gen_ppm(image: Vec<Vec<Pixel>>) -> Result<(), String> {
    let path = Path::new("out/image.ppm");
    let display = path.display();
    let height = image.len();

    if height == 0 {
        return Err(String::from("Height must be greater than 0"));
    }

    let width = image[0].len();

    File::create(&path)
        .map_err(|why| {
            format!("Couldn't create {}: {}", display, why.description())
        })
        .and_then(|mut file| {
            let img_header = format!("P6\n{} {}\n255\n", width, height);
            let mut img_buffer = Vec::from(img_header.as_bytes());

            image.iter().for_each(|row| {
                assert!(row.len() == width);
                row.iter().for_each(|pixel| {
                    img_buffer.push(pixel.r);
                    img_buffer.push(pixel.g);
                    img_buffer.push(pixel.b);
                })
            });

            file.write_all(&img_buffer).map_err(|why| {
                format!("Couldn't create {}: {}", display, why.description())
            })
        })
}
