use crate::backend::Backend;

use super::{widget_builder::WidgetBuilderTrait, window::Window, Widget, WidgetTrait};

#[derive(Debug)]
pub struct Rectangle {

}

impl WidgetTrait for Rectangle {

}

#[derive(Debug, Default)]
pub struct RectangleBuilder {
    
}

impl WidgetBuilderTrait for RectangleBuilder {
    fn build(self, _backend: &Backend, _window: Option<&Window>) -> Widget {
        Widget::Rectangle(Rectangle {  })
    }
}