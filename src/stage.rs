use image::DynamicImage;

pub trait Generator {
    fn get_image(&self) -> DynamicImage;
}

pub trait Renderer {
    fn render_command(&self);
}

pub trait Sender {
    fn send_tcp(&self);
}
