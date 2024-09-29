use lumi2d::Objects;

use crate::{backend::Backend, elements::window::Window, signals::{Signal, SignalTrait}};

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

#[derive(Debug)]
pub struct RectangleBuilder {
    pub xywh: Signal<(u32, u32, u32, u32)>
}

impl WidgetBuilderTrait for RectangleBuilder {
    fn build(self, _backend: &Backend, _window: Option<&Window>) -> Widget {
        let (x, y, w, h) = *self.xywh.get();
        Widget::Rectangle(Rectangle {
            rectangle: lumi2d::Objects::rectangle(x, y, w, h, 0xFFFFFFFF, None)
        })
    }
}