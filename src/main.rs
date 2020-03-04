#[macro_use]
extern crate clap;

mod stage;
mod image_generator;
mod naive;
mod default;

use clap::App;
use image::DynamicImage;
use crate::stage::{Generator, Renderer, Sender};
use crate::image_generator::ImageGenerator;
use crate::default::DefaultSender;

fn main() {
    let clap_yml = load_yaml!("clap.yml");
    let matches = App::from_yaml(clap_yml).get_matches();
    let generator: Option<Box<dyn Fn() -> DynamicImage>>;
    let renderer: Option<Box<dyn Fn(&DynamicImage) -> String>>;

    if let Some(host) = matches.value_of("host") {
        let port = matches.value_of("port").unwrap_or_default();

        if let Some(subcommand_matches) = matches.subcommand_matches("image") {
            if let Some(filename) = subcommand_matches.value_of("file") {
                println!("Loading image");
                let image_generator = ImageGenerator::new(filename.to_string());
                generator = Some(Box::new(move || image_generator.get_image()));

                println!("Rendering command");
                renderer = Some(Box::new(naive::render_command));
                let cmd = renderer.unwrap()(&generator.unwrap()());

                println!("Connecting to server");
                let mut sender = DefaultSender::connect(host, port);

                println!("Sending image");
                sender.send_tcp(&cmd);
            } else {
                println!("No image file specified");
            }
        }
    } else {
        println!("No server address specified");
    }

}
