use image::DynamicImage;

pub trait Generator {
    fn get_image(&self) -> DynamicImage;
}

pub trait Renderer {
    fn render_command(&mut self, frame: &DynamicImage);
    fn get_command(&self) -> String;
}

pub trait Sender {
    fn send_tcp(&mut self, command: &str);
}
