use image::DynamicImage;
use crate::stage::Renderer;

pub struct NaiveRenderer {
    command: String
}

impl Renderer for NaiveRenderer {
    fn render_command(mut self, frame: &DynamicImage) {
        self.command = String::new();
    }

    fn get_command(&self) -> Option<String> {
        None
    }
}
