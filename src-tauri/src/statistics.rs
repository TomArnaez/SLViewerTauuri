use image::{GenericImage, ImageBuffer, Luma};
use imageproc::definitions::Image;

pub fn get_snr(
    images: &Vec<ImageBuffer<Luma<u16>, Vec<u16>>>,
    window_size: u32,
    frame: u32,
) -> f64 {
    let mut min_mean: f64 = u32::MAX as f64;
    let mut max_mean: f64 = 0.0;
    let mut snr: f64 = -1.0;

    let dark_offset: u32 = 300;

    let mut bg_x: u32 = 0;
    let mut bg_y: u32 = 0;
    let mut fg_x: u32 = 0;
    let mut fg_y: u32 = 0;

    let width: u32 = images[0].width();
    let height: u32 = images[0].height();
    let image_count: u32 = images.len() as u32;

    if (frame >= image_count || window_size >= width || window_size > height) {
        return snr;
    }

    for x in 0..width + 1 {
        for y in 1..height + 1 {
            let window: ImageBuffer<Luma<u16>, Vec<u16>> = images[0]
                .sub_image(x, y, window_size, window_size)
                .to_image();

            let (window_mean, window_std_dev) = calculate_mean_and_std(&window);

            if (window_mean < min_mean) {
                min_mean = window_mean
            } else if (window_mean > max_mean) {
                max_mean = window_mean;
                fg_x = x;
                fg_y = y;
            }
        }
    }

    (max_mean - min_mean) / min_mean
}

fn calculate_mean_and_std(buffer: &ImageBuffer<Luma<u16>, Vec<u16>>) -> (f64, f64) {
    let width = buffer.width();
    let height = buffer.height();
    let mut sum = 0.0;
    let mut squared_diff_sum = 0.0;
    let total_pixels = (width * height) as f64;

    for y in 0..height {
        for x in 0..width {
            let pixel = buffer.get_pixel(x, y);
            let grayscale = (pixel[0] as f64 + pixel[1] as f64 + pixel[2] as f64) / 3.0;
            sum += grayscale;
        }
    }

    let mean = sum / total_pixels;

    for y in 0..height {
        for x in 0..width {
            let pixel = buffer.get_pixel(x, y);
            let grayscale = (pixel[0] as f64 + pixel[1] as f64 + pixel[2] as f64) / 3.0;
            let diff = grayscale - mean;
            squared_diff_sum += diff * diff;
        }
    }

    let standard_deviation = (squared_diff_sum / total_pixels).sqrt();

    (mean, standard_deviation)
}
