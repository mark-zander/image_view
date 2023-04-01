use image_view::cli;
use image::io::Reader as ImageReader;
use anyhow::*;

pub fn read_image(image_name: PathBuf) -> image::DynamicImage {
    let image = ImageReader::open(image_name)?.decode()?;
}
