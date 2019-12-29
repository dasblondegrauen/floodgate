#[macro_use]
extern crate clap;

mod stage;
mod picture;
mod naive;

use clap::App;
use image::GenericImageView;
use crate::stage::{Generator, Renderer};
use crate::picture::PictureGenerator;
use crate::naive::NaiveRenderer;

fn main() {
    let clap_yml = load_yaml!("clap.yml");
    let matches = App::from_yaml(clap_yml).get_matches();

    if let Some(host) = matches.value_of("host") {
        let port = matches.value_of("port").unwrap_or_default();

        if let Some(subcommand_matches) = matches.subcommand_matches("picture") {
            if let Some(filename) = subcommand_matches.value_of("file") {
                let generator = PictureGenerator::load_picture(String::from(filename));
                let mut renderer = NaiveRenderer::new();

                renderer.render_command(&generator.get_image());
                let cmd = renderer.get_command();
                println!("{}", &cmd);
            } else {
                println!("No picture file specified");
            }
        }
    } else {
        println!("No server address specified");
    }

}
