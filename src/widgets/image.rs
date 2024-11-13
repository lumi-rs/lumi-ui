use lumi2d::types::{Object, CacheableImage};

use crate::{backend::Backend, elements::window::Window, signals::{FutureSignal, FutureState, Signal, SignalRef, SignalTrait}};

use super::{widget_builder::WidgetBuilderTrait, Widget, WidgetTrait};

#[derive(Debug)]
pub struct Image {
    pub object: Signal<Object>
}

impl WidgetTrait for Image {
    fn get_objects(&self) -> SignalRef<Object> {
        self.object.get()
    }
}

#[derive(Debug, Default)]
pub struct ImageBuilder {
    pub x: Signal<i32>,
    pub y: Signal<i32>,
    pub width: Signal<u32>,
    pub height: Signal<u32>,
    pub bytes: Signal<Vec<u8>>
}

impl WidgetBuilderTrait for ImageBuilder {
    fn build(self, _backend: &Backend, _window: Option<&Window>) -> Widget {
        let decoder = FutureSignal::empty();

        let clone = decoder.clone();
        self.bytes.relative(move |bytes| {
            let bytes = bytes.clone();
            clone.set(async move {
                CacheableImage::from_encoded(&bytes)
            });
        });

        let combined = (self.x, self.y, self.width, self.height, decoder.relative(|a| a.clone()));

        let object = combined.relative(move |(x,y, w, h, image)| {
            let (x, y, w, h) = (**x, **y, **w, **h);

            match image.as_ref() {
                FutureState::Running => {
                    Object::rectangle(x, y, w, h, 0xAAAAAAAA, None)
                },
                FutureState::Completed(image) => {
                    Object::image(x, y, w, h, image.clone())
                }
            }
        });

        Widget::Image(Image { object })
    }
}