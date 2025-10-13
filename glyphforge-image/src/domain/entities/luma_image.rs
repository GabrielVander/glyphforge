use itertools::Itertools;

use crate::domain::entities::{luma_pixel::LumaPixel, rgb_pixel::RgbPixel};

#[derive(Debug)]
pub struct LumaImage {
    width: usize,
    height: usize,
    pixels: Vec<LumaPixel>,
    channel_size: usize,
    channel_buffer: Vec<u8>,
}

impl LumaImage {
    pub fn new(width: usize, height: usize) -> Self {
        const CHANNEL_SIZE: usize = 3;

        Self {
            width,
            height,
            pixels: Vec::with_capacity(width * height),
            channel_size: CHANNEL_SIZE,
            channel_buffer: Vec::with_capacity(CHANNEL_SIZE),
        }
    }

    pub fn is_full(&self) -> bool {
        self.pixels.len() >= self.width * self.height
    }

    pub fn add_raw_byte(&mut self, byte: u8) {
        self.channel_buffer.push(byte);

        if self.channel_buffer.len() == self.channel_size {
            self.pixels.push(LumaPixel::from_rgb(RgbPixel::from_slice(
                self.channel_buffer
                    .iter()
                    .copied()
                    .collect_array::<3>()
                    .unwrap(),
            )));

            self.channel_buffer.clear();
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use std::collections::VecDeque;

    use crate::domain::entities::{luma_image::LumaImage, luma_pixel::LumaPixel};

    #[test]
    fn should_create_from_raw_rgb_iteratively() {
        let mut raw_rgb: VecDeque<u8> = VecDeque::from([
            255, 0, 0, // Red
            0, 255, 0, // Green
            0, 0, 255, // Blue
        ]);

        let expected_pixels: Vec<LumaPixel> = vec![
            54_u8,  // Luma for Red
            182_u8, // Luma for Green
            18_u8,  // Luma for Blue
        ]
        .into_iter()
        .map(LumaPixel::new)
        .collect();

        let mut image: LumaImage = LumaImage::new(3, 1);

        while !image.is_full() {
            if let Some(byte) = raw_rgb.pop_front() {
                image.add_raw_byte(byte)
            } else {
                break;
            }
        }

        assert_eq!(image.pixels, expected_pixels);
    }
}
