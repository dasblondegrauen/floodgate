use image::DynamicImage;
use crate::stage::Generator;

pub struct PictureGenerator {
    filename: String,
    image: DynamicImage
}

impl Generator for PictureGenerator {
    fn get_image(&self) -> DynamicImage {
        self.image.clone()
    }
}

impl PictureGenerator {
    pub fn load_image(file: String) -> PictureGenerator {
        let img = image::open(&file).expect(&format!("Could not load {}", &file));
        PictureGenerator {
            filename: file,
            image: img
        }
    }
}
