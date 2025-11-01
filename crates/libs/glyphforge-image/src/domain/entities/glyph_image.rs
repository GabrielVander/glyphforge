trait GlyphImage {
    fn get_image_data(&self) -> &[u8];

    fn from_image_data(data: Vec<u8>, width: u32, height: u32) -> Self;

    fn get_width(&self) -> u32;

    fn get_height(&self) -> u32;
}
