use lumi2d::types::{Rounding, Object};

use crate::{backend::Backend, elements::window::Window, signals::{Signal, SignalRef, SignalTrait}};

use super::{widget_builder::WidgetBuilderTrait, Widget, WidgetTrait};

#[derive(Debug)]
pub struct Rectangle {
    pub rectangle: Signal<Object>
}

impl WidgetTrait for Rectangle {
    fn get_objects(&self) -> SignalRef<Object> {
        self.rectangle.get()
    }
}

#[derive(Debug, Default)]
pub struct RectangleBuilder {
    pub x: Signal<i32>,
    pub y: Signal<i32>,
    pub width: Signal<u32>,
    pub height: Signal<u32>,
    pub color: Signal<u32>,
    pub rounding: Signal<Option<Rounding>>
}

impl WidgetBuilderTrait for RectangleBuilder {
    fn build(self, _backend: &Backend, _window: Option<&Window>) -> Widget {
        let combined = (self.x, self.y, self.width, self.height, self.color, self.rounding);

        let rectangle = combined.relative(|(x,y, w, h, c, r)| {
            Object::rectangle(**x, **y, **w, **h, **c, r.cloned())
        });

        Widget::Rectangle(Rectangle { rectangle })
    }
}