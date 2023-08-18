// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageOutputFormat};
use std::io::Cursor;

mod commands;
mod cpp;
mod statistics;

use std::thread;

fn test() {
    let mut device = cpp::Detector::new();
    device.signal_accumulation(100, 10);
}

fn main() {
    let mut device = cpp::Detector::new();

    thread::spawn(|| test());
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::image,
            commands::get_captures
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
