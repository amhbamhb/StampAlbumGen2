// Image utilities for StampAlbumGen2
use image::{DynamicImage, ImageBuffer, Rgba, io::Reader as ImageReader};
use rusttype::{Font, Scale};
use std::fs::File;
use std::path::Path;

pub fn load_image<P: AsRef<Path>>(path: P) -> image::ImageResult<DynamicImage> {
    ImageReader::open(path)?.decode()
}

// Add more image-related functions as needed
