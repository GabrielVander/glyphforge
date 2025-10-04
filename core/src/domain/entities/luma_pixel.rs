#[derive(Debug)]
struct LumaPixel {
    y: u8,
}

impl LumaPixel {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let luma_value: u8 = ((2126 * r as u32 + 7152 * g as u32 + 722 * b as u32) / 10000) as u8;
        Self { y: luma_value }
    }
}

impl AsciiRenderable for LumaPixel {
    fn to_ascii(&self, renderer: &dyn AsciiRenderer) -> String {
        renderer.render_luma(self.y).to_string()
    }

    fn add_child(&mut self, _child: Box<dyn AsciiRenderable>) {
        todo!()
    }

    fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn AsciiRenderable>> {
        todo!()
    }
}
