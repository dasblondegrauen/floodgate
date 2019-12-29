use image::{DynamicImage, GenericImageView, Pixel};
use crate::stage::Renderer;

pub struct NaiveRenderer {
    command: String
}

impl Renderer for NaiveRenderer {
    fn render_command(&mut self, frame: &DynamicImage) {
        self.command = String::new();

        for (x, y, pix) in frame.pixels() {
            let rgba = pix.to_rgba();
            if rgba[0] != 0 {
                // cmd.push_str(&format!("PX {} {} {:02X}{:02X}{:02X}{:02X}\n", x+offset.0, y+offset.1, rgba[0], rgba[1], rgba[2], rgba[3]));
                self.command.push_str(&format!("PX {} {} {:02X}{:02X}{:02X}{:02X}\n", x, y, rgba[0], rgba[1], rgba[2], rgba[3]));
            }
        }
    }

    fn get_command(&self) -> String {
        self.command.clone()
    }
}

impl NaiveRenderer {
    pub fn new() -> NaiveRenderer {
        NaiveRenderer{command: String::new()}
    }
}
