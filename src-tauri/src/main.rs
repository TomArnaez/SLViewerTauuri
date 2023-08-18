// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageOutputFormat};
use std::io::Cursor;

#[tauri::command]
fn image() -> String {
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
fn get_captures() -> Vec<Vec<String>> {
    let mut my_vector: Vec<Vec<String>> = vec![vec!["test".to_string(), "test".to_string()]];

    my_vector
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn test() {
    println!("Hello, ! You've been greeted from Rust!");
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, test, image, get_captures])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
