use lumi2d::{renderer::images::CacheableImage, Object};

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
    fn build(self, backend: &Backend, _window: Option<&Window>) -> Widget {
        let signal = FutureSignal::empty();
        let combined = (self.x, self.y, self.width, self.height, self.bytes, signal.relative(|a| a.clone()));
        let _weak = backend.weak_inner();

        let clone = signal.clone();
        let object = combined.relative(move |
            (x,y, w, h, bytes, image)
        | {
            let (x, y, w, h, bytes) = (**x, **y, **w, **h, bytes.cloned());
            
            clone.set(async move {
                println!("a");
                CacheableImage::from_encoded(&bytes)
            });

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