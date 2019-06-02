extern crate image;

use image::{GenericImageView, DynamicImage, Pixel};
use std::net::UdpSocket;

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

    let target = std::env::args().nth(2);
    let target = match target {
        Some(value) => value,
        None => {
            println!("No target given");
            std::process::exit(0);
        },
    };

    let img = image::open(&filename);
    let img = match img {
        Ok(value) => value,
        Err(error) => {
            println!("Could not open {}: {}", &filename, error);
            std::process::exit(0);
        },
    };

    let cmd = generate_command(&img, (0, 0));
    let socket = UdpSocket::bind("localhost:6666").expect("Could not bind locally");

    let mut i = 0;
    loop {
        socket.send_to(cmd[i..i+512].as_bytes(), &target).expect(&format!("Could not send command"));
        i += 512;
        if i < cmd.len() {
            i = 0;
        }
    }
}
