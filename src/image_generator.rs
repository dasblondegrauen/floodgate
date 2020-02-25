use image::DynamicImage;

pub(crate) struct ImageGenerator {
    filename: String,
    image: DynamicImage
}

impl ImageGenerator {
    pub fn new(filename: String) -> ImageGenerator {
        let img = image::open(&filename).expect(&format!("Could not open {}", &filename));

        ImageGenerator {
            filename: filename,
            image: img
        }
    }

    pub fn get_image(&self) -> DynamicImage {
        self.image.clone()
    }
}
