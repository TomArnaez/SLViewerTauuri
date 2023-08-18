use image::{ImageBuffer, Luma};

pub fn histogram_equilization(
    image: &ImageBuffer<Luma<u16>, Vec<u16>>,
) -> ImageBuffer<Luma<u16>, Vec<u16>> {
    let (width, height) = image.dimensions();
    let mut histogram = [0u32; 65536];
    let mut lut = [0u32; 65536];

    for pixel in image.pixels() {
        let intensity = pixel[0] as usize;
        histogram[intensity] += 1;
    }

    let mut i = 0;
    while histogram[i] == 0 {
        i += 1;
    }

    let divisor = image.width() * image.height() - histogram[i];
    let scale = if divisor != 0 {
        (histogram.len() - 1) as f32 / divisor as f32
    } else {
        1.0
    };

    let mut sum = 0;
    for i in 0..65535 {
        sum += histogram[i];
        lut[i] = (sum as f32 * scale) as u32;
    }

    let mut equalized_image = ImageBuffer::<Luma<u16>, Vec<u16>>::new(width, height);
    for (x, y, pixel) in equalized_image.enumerate_pixels_mut() {
        let intensity = image.get_pixel(x, y)[0] as usize;
        let lutval = lut[intensity];
        *pixel = Luma([lutval as u16]);
    }

    equalized_image
}

pub fn histogram_equalization_roi(
    image: &ImageBuffer<Luma<u16>, Vec<u16>>,
    roi_x: u32,
    roi_y: u32,
    roi_width: u32,
    roi_height: u32,
) -> ImageBuffer<Luma<u16>, Vec<u16>> {
    let (width, height) = image.dimensions();
    let mut histogram = [0u32; 65536];
    let mut lut = [0u32; 65536];

    for y in roi_y..(roi_y + roi_height) {
        for x in roi_x..(roi_x + roi_width) {
            let intensity = image.get_pixel(x, y)[0] as usize;
            histogram[intensity] += 1;
        }
    }

    let divisor = roi_width * roi_height - histogram[0];
    if divisor == 0 {
        return image.clone();
    }

    let scale = (65535) as f32 / divisor as f32;

    let mut sum = 0;
    for i in 0..=65535 {
        sum += histogram[i];
        lut[i] = (sum as f32 * scale) as u32;
    }

    let mut equalized_image = ImageBuffer::<Luma<u16>, Vec<u16>>::new(width, height);
    for (x, y, pixel) in equalized_image.enumerate_pixels_mut() {
        let intensity = image.get_pixel(x, y)[0] as usize;
        let lutval = lut[intensity];
        *pixel = Luma([lutval as u16]);
    }

    equalized_image
}

pub fn adjust_brightness(
    image: &ImageBuffer<Luma<u16>, Vec<u16>>,
    delta: u16,
) -> ImageBuffer<Luma<u16>, Vec<u16>> {
    let mut adjusted_image = ImageBuffer::<Luma<u16>, Vec<u16>>::new(image.width(), image.height());
    for (x, y, pixel) in adjusted_image.enumerate_pixels_mut() {
        let old_pixel_intensity = image.get_pixel(x, y)[0] as u16;
        let new_intensity = old_pixel_intensity.saturating_add(delta); // Ensure the new value doesn't exceed u16 range
        *pixel = Luma([new_intensity]);
    }

    adjusted_image
}

pub fn adjust_contrast(
    image: &ImageBuffer<Luma<u16>, Vec<u16>>,
    contrast: f32,
) -> ImageBuffer<Luma<u16>, Vec<u16>> {
    let mut adjusted_image = ImageBuffer::<Luma<u16>, Vec<u16>>::new(image.width(), image.height());

    for (x, y, pixel) in adjusted_image.enumerate_pixels_mut() {
        let original_intensity = image.get_pixel(x, y)[0] as f32;
        let new_intensity = 32768.0 + (original_intensity - 32768.0) * contrast;
        let clamped_intensity = new_intensity.max(0.0).min(65535.0);
        *pixel = Luma([clamped_intensity as u16]);
    }

    adjusted_image
}

pub fn invert_colors_grayscale(
    image: &ImageBuffer<Luma<u16>, Vec<u16>>,
) -> ImageBuffer<Luma<u16>, Vec<u16>> {
    let (width, height) = image.dimensions();
    let mut inverted_image = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let inverted_pixel_value = u16::max_value() - pixel[0];
            inverted_image.put_pixel(x, y, Luma([inverted_pixel_value]));
        }
    }

    inverted_image
}
