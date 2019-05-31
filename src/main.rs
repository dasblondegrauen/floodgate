extern crate image;

use image::GenericImageView;

fn main() {
    let filename = std::env::args().nth(1).expect("No filename given");
    let img = image::open(filename).expect("Could not open image");

    let (x, y) = img.dimensions();
    println!("{}x{}", x, y);
}
