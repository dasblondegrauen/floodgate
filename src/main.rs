#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let clap_yml = load_yaml!("clap.yml");
    let matches = App::from_yaml(clap_yml).get_matches();

    if let Some(host) = matches.value_of("host") {
        println!("Pixelflut server address: {}", host);

        let port = matches.value_of("port").unwrap_or_default();
        println!("Pixelflut server port: {}", port);

        if let Some(subcommand_matches) = matches.subcommand_matches("image") {
            if let Some(filename) = subcommand_matches.value_of("file") {
                println!("Image file: {}", filename);
            } else {
                println!("No image file specified");
            }
        }
    } else {
        println!("No server address specified");
    }

}
