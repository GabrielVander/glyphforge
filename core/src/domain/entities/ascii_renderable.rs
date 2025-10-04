use crate::domain::entities::ascii_renderer::AsciiRenderer;

pub(crate) trait AsciiRenderable: std::fmt::Debug {
    fn to_ascii(&self, _renderer: &dyn AsciiRenderer) -> String;

    fn add_child(&mut self, _child: Box<dyn AsciiRenderable>);

    fn get_child_mut(&mut self, _index: usize) -> Option<&mut Box<dyn AsciiRenderable>>;
}
