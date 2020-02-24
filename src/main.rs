#[macro_use]
extern crate clap;

mod stage;
mod image;
mod naive;
mod default;

use clap::App;
use crate::stage::{Generator, Renderer, Sender};
use crate::image::PictureGenerator;
use crate::naive::NaiveRenderer;
use crate::default::DefaultSender;

fn main() {
    let clap_yml = load_yaml!("clap.yml");
    let matches = App::from_yaml(clap_yml).get_matches();

    if let Some(host) = matches.value_of("host") {
        let port = matches.value_of("port").unwrap_or_default();

        if let Some(subcommand_matches) = matches.subcommand_matches("image") {
            if let Some(filename) = subcommand_matches.value_of("file") {
                println!("Loading image");
                let generator = PictureGenerator::load_image(String::from(filename));

                println!("Rendering command");
                let mut renderer = NaiveRenderer::new();
                renderer.render_command(&generator.get_image());
                let cmd = renderer.get_command();

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
