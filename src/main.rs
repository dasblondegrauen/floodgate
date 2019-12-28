#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let clap_yml = load_yaml!("clap.yml");
    let matches = App::from_yaml(clap_yml).get_matches();

    if let Some(host) = matches.value_of("host") {
        println!("Pixelflut server address: {}", host);
    } else {
        println!("No server address specified");
    }

    let port = matches.value_of("port").unwrap_or_default();
    println!("Pixelflut server port: {}", port);

    if let Some(image) = matches.value_of("image") {
        println!("Image file: {}", image);
    } else {
        println!("No image file specified");
    }
}
