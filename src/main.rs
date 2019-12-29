#[macro_use]
extern crate clap;

mod stage;
mod picture;

use clap::App;
use image::GenericImageView;
use crate::stage::Generator;
use crate::picture::PictureGenerator;

fn main() {
    let clap_yml = load_yaml!("clap.yml");
    let matches = App::from_yaml(clap_yml).get_matches();

    if let Some(host) = matches.value_of("host") {
        println!("Pixelflut server address: {}", host);

        let port = matches.value_of("port").unwrap_or_default();
        println!("Pixelflut server port: {}", port);

        if let Some(subcommand_matches) = matches.subcommand_matches("picture") {
            if let Some(filename) = subcommand_matches.value_of("file") {
                println!("Picture file: {}", filename);
                let generator = PictureGenerator::load_picture(String::from(filename));
                let dimensions = generator.get_image().dimensions();
                println!("{}x{}", dimensions.0, dimensions.1);
            } else {
                println!("No picture file specified");
            }
        }
    } else {
        println!("No server address specified");
    }

}
