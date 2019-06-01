extern crate image;

use image::{GenericImageView, DynamicImage, Pixel};

fn main() {
    let filename = std::env::args().nth(1);
    let filename = match filename {
        Some(x) => x,
        None => {
            println!("No filename given");
            std::process::exit(0);
        },
    };

    let img = image::open(&filename);
    let img = match img {
        Ok(x) => x,
        Err(error) => {
            println!("Could not open {}: {}", &filename, error);
            std::process::exit(0);
        },
    };


    let (x, y) = img.dimensions();
    println!("{}x{}", x, y);

    let cmd = generate_command(&img);
    println!("Generated command:\n{}", cmd);
}

fn generate_command(img: &DynamicImage) -> String {
    let mut cmd = String::from("");
    for (x, y, pix) in img.pixels() {
        let rgba = pix.to_rgba();
        if rgba[0] != 0 {
            cmd.push_str(&format!("PX {} {} {:02X}{:02X}{:02X}{:02X}\n", x, y, rgba[0], rgba[1], rgba[2], rgba[3]));
        }
    }

    cmd
}
