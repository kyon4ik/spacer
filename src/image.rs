use std::fs::File;
use std::io;
use std::path::Path;

use crate::color::Color;

pub struct Image {
    width: u32,
    height: u32,
    stride: u32,
    pixels: Vec<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let stride = width * 3;
        Self {
            width,
            height,
            stride,
            pixels: vec![0; stride as usize * height as usize],
        }
    }

    pub fn from_aspect_ratio(width: u32, aspect_ratio: f64) -> Self {
        assert!(aspect_ratio.is_finite() && aspect_ratio > 0.0);
        let height = (width as f64 / aspect_ratio) as u32;
        Self::new(width, height)
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
        let gamma = color.linear_to_gamma();

        // Gamma gives the valid range [0.0, 1.0]
        let r = (255.0 * gamma.r()) as u8;
        let g = (255.0 * gamma.g()) as u8;
        let b = (255.0 * gamma.b()) as u8;

        let index = y as usize * self.stride as usize + x as usize * 3;
        self.pixels[index] = r;
        self.pixels[index + 1] = g;
        self.pixels[index + 2] = b;
    }

    pub fn split_n(&mut self, n: u32) -> Vec<SubImage<'_>> {
        let mut sub_images = Vec::with_capacity(n as usize);
        let mut remaining_pixels = self.pixels.as_mut_slice();

        let rows_per_stripe = self.height / n;
        let mut remainder = self.height % n;
        let mut current_y_offset = 0;

        for _ in 0..n {
            // Give an extra row to the first few stripes if there's a remainder
            let stripe_height = rows_per_stripe + if remainder > 0 { 1 } else { 0 };
            remainder = remainder.saturating_sub(1);

            if stripe_height == 0 {
                break;
            }

            let byte_count = (stripe_height * self.stride) as usize;
            let (current_slice, rest) = remaining_pixels.split_at_mut(byte_count);
            remaining_pixels = rest;

            sub_images.push(SubImage {
                width: self.width,
                height: stripe_height,
                stride: self.stride,
                y_offset: current_y_offset,
                pixels: current_slice,
            });

            current_y_offset += stripe_height;
        }

        sub_images
    }

    pub fn save_as_ppm<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = File::create(path)?;
        ppm::write(&mut file, &self.pixels, self.width, self.height)
    }
}

pub struct SubImage<'a> {
    width: u32,
    height: u32,
    stride: u32,
    y_offset: u32,
    pixels: &'a mut [u8],
}

impl SubImage<'_> {
    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }

    pub fn get_y_offset(&self) -> u32 {
        self.y_offset
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> &[u8] {
        self.pixels
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
        let gamma = color.linear_to_gamma();

        // Gamma gives the valid range [0.0, 1.0]
        let r = (255.0 * gamma.r()) as u8;
        let g = (255.0 * gamma.g()) as u8;
        let b = (255.0 * gamma.b()) as u8;

        let index = y as usize * self.stride as usize + x as usize * 3;
        self.pixels[index] = r;
        self.pixels[index + 1] = g;
        self.pixels[index + 2] = b;
    }
}

pub mod ppm {
    use std::io;

    const MAGIC: &[u8] = b"P6";
    const MAX_PIXEL_VALUE: u16 = 255;

    pub fn write<W: io::Write>(
        writer: &mut W,
        pixels: &[u8],
        width: u32,
        height: u32,
    ) -> Result<(), io::Error> {
        let len = pixels.len() as u32;
        assert!(
            len.is_multiple_of(3),
            "PPM requires RGB format, but {len} is not divisible by 3"
        );
        assert_eq!(len, width * height * 3, "Size of pixels is incorrect");

        writer.write_all(MAGIC)?;
        write!(writer, "\n{width} {height}\n{MAX_PIXEL_VALUE}\n")?;
        writer.write_all(pixels)
    }
}
