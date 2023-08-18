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

#[tauri::command]
pub fn image() -> String {
    let img2 = ImageReader::open("C:/dev/data/sl_test.tif")
        .unwrap()
        .decode()
        .unwrap();

    let mut image_data: Vec<u8> = Vec::new();
    img2.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
        .unwrap();
    let res_base64 = base64::encode(image_data);
    format!("data:image/png;base64,{}", res_base64)
}

#[tauri::command]
pub fn get_captures() -> Vec<Vec<String>> {
    let mut my_vector: Vec<Vec<String>> = vec![vec!["test".to_string(), "test".to_string()]];

    my_vector
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
