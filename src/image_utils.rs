// Image utilities for StampAlbumGen2
use image::{DynamicImage, io::Reader as ImageReader};
use std::path::Path;

pub fn load_image<P: AsRef<Path>>(path: P) -> image::ImageResult<DynamicImage> {
    ImageReader::open(path)?.decode()
}

// Add more image-related functions as needed
