use crate::vec3::Color;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};

pub fn write_image(colors: Vec<Vec<Color>>, filename: &str) {
    let width = colors.len() as u32;
    let height = colors[0].len() as u32;
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
        let color = colors[x as usize][y as usize];
        *pixel = Rgb(<[u8; 3]>::from(color));
    }

    bar.finish();

    match buffer.save(filename) {
        Err(e) => eprintln!("Error writing to file {}", e),
        Ok(()) => (),
    }
}
