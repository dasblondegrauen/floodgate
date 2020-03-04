use image::{DynamicImage, GenericImageView, Pixel};

pub fn render_command(frame: &DynamicImage) -> String{
    let mut command = String::new();

    for (x, y, pix) in frame.pixels() {
        let rgba = pix.to_rgba();
        if rgba[0] != 0 {
            command.push_str(&format!("PX {} {} {:02X}{:02X}{:02X}{:02X}\n", x, y, rgba[0], rgba[1], rgba[2], rgba[3]));
        }
    }

    command
}
