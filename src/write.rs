use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};

pub fn write_image(height: u32, width: u32, filename: &str) {
    let mut buffer: RgbImage = ImageBuffer::new(width, height);

    let bar = ProgressBar::new(height as u64);
    bar.set_style(
        ProgressStyle::default_bar().template(
            "[{elapsed} elapsed] {wide_bar} {percent}% [{eta} remaining] writing to file",
        ),
    );

    let mut prev: u32 = 0;
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        if y != prev {
            prev = y;
            bar.inc(1);
        }

        let r = x as f64 / (width - 1) as f64;
        let g = y as f64 / (height - 1) as f64;
        let b = 0.25;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }

    bar.finish();

    match buffer.save(filename) {
        Err(e) => eprintln!("Error writing to file {}", e),
        Ok(()) => (),
    }
}
