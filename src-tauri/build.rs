use std::env;
use std::path::PathBuf;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() -> miette::Result<()> {
    tauri_build::build();

    let mut paths: Vec<PathBuf> = Vec::new();

    /*
    if let Ok(path_value) = env::var("SL_INCLUDE") {
        let path = PathBuf::from(path_value);
    } else {
    }
    */

    let path = std::path::PathBuf::from("src");
    let include_path = std::path::PathBuf::from("C:\\SLDevice\\SDK\\headers");
    let mut b = autocxx_build::Builder::new("src/cpp.rs", [&path, &include_path]).build()?;
    b.flag_if_supported("-std=c++14").compile("autocxx-demo");

    let lib_path = "C:\\SLDevice\\SDK\\lib\\x64\\Release";
    let pleora_lib_path = std::path::PathBuf::from("C:\\SLDevice\\SDK\\lib\\PleoraLibs");
    println!("cargo:rustc-link-search={}", lib_path);
    println!("cargo:rustc-link-lib=SLDeviceLib");
    println!("cargo:rustc-link-search=C:/SLDevice/SDK/lib/x64/Release");
    println!("cargo:rustc-link-lib=SLDeviceLib");

    Ok(())
}
