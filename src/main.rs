extern crate image;

use image::GenericImageView;

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
}
