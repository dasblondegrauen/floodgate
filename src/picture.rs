use image::{GenericImageView, DynamicImage};
use crate::stage::Generator;

pub struct PictureGenerator {
    filename: String,
    picture: DynamicImage
}

impl Generator for PictureGenerator {
    fn get_image(&self) -> DynamicImage {
        self.picture.clone()
    }
}

impl PictureGenerator {
    fn load_picture(mut self, file: String) {
        self.filename = file;

        if let Ok(img) = image::open(&self.filename) {
            self.picture = img;
        }
    }
}
