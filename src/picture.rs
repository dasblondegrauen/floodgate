use image::DynamicImage;
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
    pub fn load_picture(file: String) -> PictureGenerator {
        let img = image::open(&file).expect(&format!("Could not load {}", &file));
        PictureGenerator {
            filename: file,
            picture: img
        }
    }
}
