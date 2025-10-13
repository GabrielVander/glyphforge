#[derive(Debug, PartialEq, Clone)]
pub(crate) struct RgbPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbPixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_slice(i: [u8; 3]) -> Self {
        Self::new(i[0], i[1], i[2])
    }
}

#[cfg(test)]
mod test {
    use crate::domain::entities::rgb_pixel::RgbPixel;

    #[test]
    fn should_create_pixel_from_slice() {
        let rgb_values: Vec<([u8; 3], RgbPixel)> = vec![
            ([0, 0, 0], RgbPixel::new(0, 0, 0)),             // Black
            ([255, 255, 255], RgbPixel::new(255, 255, 255)), // White
            ([255, 0, 0], RgbPixel::new(255, 0, 0)),         // Red
            ([0, 255, 0], RgbPixel::new(0, 255, 0)),         // Green
            ([0, 0, 255], RgbPixel::new(0, 0, 255)),         // Blue
            ([128, 128, 128], RgbPixel::new(128, 128, 128)), // Gray
            ([255, 255, 0], RgbPixel::new(255, 255, 0)),     // Yellow
            ([0, 255, 255], RgbPixel::new(0, 255, 255)),     // Cyan
            ([255, 0, 255], RgbPixel::new(255, 0, 255)),     // Magenta
        ];

        let results: Vec<RgbPixel> = rgb_values
            .iter()
            .map(|i| RgbPixel::from_slice(i.0))
            .collect();

        assert_eq!(
            rgb_values
                .iter()
                .map(|i| i.1.clone())
                .collect::<Vec<RgbPixel>>(),
            results
        );
    }
}
