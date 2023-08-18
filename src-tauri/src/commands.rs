use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageOutputFormat};
use std::io::Cursor;

#[derive(Clone, serde::Serialize)]
pub struct Capture {
    thumbnail64: &'static str,
    width: u16,
    height: u16,
    depth: u16,
}
