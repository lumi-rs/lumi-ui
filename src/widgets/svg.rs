use std::sync::Arc;

use lumi2d::types::{CacheableSvg, Object};

use crate::{backend::Backend, elements::window::Window, signals::{Signal, SignalRef, SignalTrait}};

use super::{widget_builder::WidgetBuilderTrait, Widget, WidgetTrait};

#[derive(Debug)]
pub struct Svg {
    pub object: Signal<Object>
}

impl WidgetTrait for Svg {
    fn get_objects(&self) -> SignalRef<Object> {
        self.object.get()
    }
}

#[derive(Debug, Default)]
pub struct SvgBuilder {
    pub x: Signal<i32>,
    pub y: Signal<i32>,
    pub width: Signal<u32>,
    pub height: Signal<u32>,
    pub color: Signal<u32>,
    pub bytes: Signal<Arc<[u8]>>
}

impl WidgetBuilderTrait for SvgBuilder {
    fn build(self, _backend: &Backend, _window: Option<&Window>) -> Widget {
        let combined = (self.x, self.y, self.width, self.height, self.color, self.bytes);

        let object = combined.relative(move |(x,y, w, h, color, bytes)| {
            let (x, y, w, h, color) = (**x, **y, **w, **h, **color);

            Object::svg(x, y, w, h, CacheableSvg::new(bytes.cloned()), color)
        });

        Widget::Svg(Svg { object })
    }
}