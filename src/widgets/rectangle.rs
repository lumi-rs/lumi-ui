use lumi2d::Objects;

use crate::{backend::Backend, elements::window::Window};

use super::{widget_builder::WidgetBuilderTrait, Widget, WidgetTrait};

#[derive(Debug)]
pub struct Rectangle {
    rectangle: lumi2d::Objects
}

impl WidgetTrait for Rectangle {
    fn get_objects(&self) -> &Objects {
        &self.rectangle
    }
}

#[derive(Debug, Default)]
pub struct RectangleBuilder {
    
}

impl WidgetBuilderTrait for RectangleBuilder {
    fn build(self, _backend: &Backend, _window: Option<&Window>) -> Widget {
        Widget::Rectangle(Rectangle {
            rectangle: lumi2d::Objects::rectangle(100, 100, 100, 200, 0xFFFFFFFF, None)
        })
    }
}