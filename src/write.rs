use image::{RgbImage, ImageBuffer, Rgb};

pub fn write_image(height: u32, width: u32, filename: &str) {
    let mut buffer: RgbImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let r = x as f64 / (width - 1) as f64;
        let g = y as f64 / (height - 1) as f64;
        let b = 0.25;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }

    match buffer.save(filename) {
        Err(e) => eprintln!("Error writing to file {}", e),
        Ok(()) => (),
    }
}