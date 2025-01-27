use enum_dispatch::enum_dispatch;

use crate::{backend::Backend, elements::window::{Window, WindowBuilder}};

use super::*;


#[derive(Debug)]
#[enum_dispatch(WidgetBuilderTrait)]
pub enum WidgetBuilder {
    Window(WindowBuilder),
    Rectangle(rectangle::RectangleBuilder),
    TextBuilder(text::TextBuilder),
    Image(image::ImageBuilder),
    Svg(svg::SvgBuilder),
    Interact(interact::InteractBuilder)
}


#[enum_dispatch]
pub trait WidgetBuilderTrait {
    fn expected_children(&self) -> usize { 1 }
    fn build(self, backend: &Backend, window: Option<&Window>) -> Widget;
}