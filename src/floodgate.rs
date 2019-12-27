extern crate image;

use image::{GenericImageView, DynamicImage, Pixel};
use std::net::TcpStream;
use std::io::Write;

enum ArgumentType {
    Required(String),
    Optional(String),
}

fn get_argument(position: usize, argument_type: ArgumentType) -> String {
    match std::env::args().nth(position) {
        Some(value) => value,
        None => {
            match argument_type {
                ArgumentType::Required(error_message) => {
                    println!("{}", &error_message);
                    std::process::exit(0);
                },
                ArgumentType::Optional(default_value) => default_value.clone(),
            }
        }
    }
}

fn generate_command(img: &DynamicImage, offset: (u32, u32)) -> String {
    let mut cmd = String::from("");
    for (x, y, pix) in img.pixels() {
        let rgba = pix.to_rgba();
        if rgba[0] != 0 {
            cmd.push_str(&format!("PX {} {} {:02X}{:02X}{:02X}{:02X}\n", x+offset.0, y+offset.1, rgba[0], rgba[1], rgba[2], rgba[3]));
        }
    }

    cmd
}

fn main() {
    let filename = get_argument(1, ArgumentType::Required(String::from("No filename given")));
    let target = get_argument(2, ArgumentType::Required(String::from("No target given")));
    let offset: (u32, u32) = (get_argument(3, ArgumentType::Optional("0".to_string())).parse().unwrap(), get_argument(3, ArgumentType::Optional("0".to_string())).parse().unwrap());

    let img = image::open(&filename);
    let img = match img {
        Ok(value) => value,
        Err(error) => {
            println!("Could not open {}: {}", &filename, error);
            std::process::exit(0);
        },
    };

    let cmd = generate_command(&img, offset);
    let mut stream = TcpStream::connect(&target).expect(&format!("Could not connect to {}", &target));

    loop {
        stream.write_all(cmd.as_bytes()).expect("Could not send command");
    }
}
