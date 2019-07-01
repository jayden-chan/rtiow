use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn gen_ppm(image: Vec<Vec<Pixel>>) {
    let path = Path::new("out/image.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => {
            panic!("Couldn't create {}: {}", display, why.description())
        }
        Ok(file) => file,
    };

    let height = image.len();

    assert!(height > 0);

    let width = image[0].len();

    let img_header = format!("P6\n{} {}\n255\n", width, height);
    let mut img_buffer = Vec::from(img_header.as_bytes());

    for (idx, row) in image.iter().enumerate() {
        assert!(row.len() == width);

        print!(
            "Writing file... {}%\r",
            (((idx + 1) as f32 / height as f32) * 100.0) as u8
        );

        for pixel in row {
            img_buffer.push(pixel.r);
            img_buffer.push(pixel.g);
            img_buffer.push(pixel.b);
        }
    }

    match file.write_all(&img_buffer) {
        Err(why) => {
            panic!("\nCouldn't write to {}: {}", display, why.description())
        }
        Ok(_) => println!("\nImage written to {}", display),
    }
}
