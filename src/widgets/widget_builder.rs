use enum_dispatch::enum_dispatch;

use crate::backend::Backend;

use super::{rectangle::RectangleBuilder, window::{Window, WindowBuilder}, Widget};


#[derive(Debug)]
#[enum_dispatch(WidgetBuilderTrait)]
pub enum WidgetBuilder {
    Window(WindowBuilder),
    Rectangle(RectangleBuilder)
}


#[enum_dispatch]
pub trait WidgetBuilderTrait {
    fn expected_children(&self) -> usize { 1 }
    fn build(self, backend: &Backend, window: Option<&Window>) -> Widget;
}