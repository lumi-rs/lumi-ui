use lumi2d::types::{Object, TextOptions};

use crate::{backend::Backend, elements::window::Window, signals::{Signal, SignalRef, SignalTrait}};

use super::{widget_builder::WidgetBuilderTrait, Widget, WidgetTrait};


#[derive(Debug)]
pub struct Text {
    pub paragraph: Signal<Object>
}

impl WidgetTrait for Text {
    fn get_objects(&self) -> Option<SignalRef<Object>> {
        Some(self.paragraph.get())
    }

    fn expected_children(&self) -> usize {
        0
    }
}

#[derive(Debug, Default, Clone)]
pub struct TextBuilder {
    pub x: Signal<i32>,
    pub y: Signal<i32>,
    pub text: Signal<String>,
    pub options: Signal<TextOptions>,
    pub width: Signal<u32>,
    pub max_height: Signal<Option<u32>>
}

impl WidgetBuilderTrait for TextBuilder {
    fn build(&self, backend: &Backend, _window: Option<&Window>) -> Widget {
        let combined = (self.x.clone(), self.y.clone(), self.text.clone(), self.options.clone(), self.width.clone(), self.max_height.clone());
        let weak = backend.weak();

        let paragraph = combined.relative(move |(x, y, text, options, width, max_h)| {
            let backend = weak.upgrade().unwrap();
            let paragraph = backend.backend.data().create_paragraph(text.cloned(), **width, max_h.cloned(), options.cloned());

            Object::paragraph(**x, **y, paragraph)
        });

        Widget::Text(
            Text { paragraph }
        )
    }
}