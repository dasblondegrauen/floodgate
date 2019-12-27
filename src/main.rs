#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let clap_yml = load_yaml!("clap.yml");
    let matches = App::from_yaml(clap_yml).get_matches();

    if let Some(host) = matches.value_of("host") {
        println!("Pixelflut host: {}", host);
    } else {
        println!("No host given");
    }
}
