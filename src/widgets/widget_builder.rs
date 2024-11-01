use enum_dispatch::enum_dispatch;

use crate::{backend::Backend, elements::window::{Window, WindowBuilder}};

use super::{rectangle::RectangleBuilder, root::RootBuilder, text::TextBuilder, Widget};


#[derive(Debug)]
#[enum_dispatch(WidgetBuilderTrait)]
pub enum WidgetBuilder {
    Root(RootBuilder),
    Window(WindowBuilder),
    Rectangle(RectangleBuilder),
    TextBuilder(TextBuilder)
}


#[enum_dispatch]
pub trait WidgetBuilderTrait {
    fn expected_children(&self) -> usize { 1 }
    fn build(self, backend: &Backend, window: Option<&Window>) -> Widget;
}