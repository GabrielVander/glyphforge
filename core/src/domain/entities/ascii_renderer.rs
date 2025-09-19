pub(crate) trait AsciiRenderer {
    fn render_luma(&self, y: u8) -> char;
}
