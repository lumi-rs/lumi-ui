use lumi2d::types::{CacheableImage, Event, Object};

use crate::{backend::Backend, byte_source::ByteSource, custom_event::CustomEvent, elements::window::Window, signals::{FutureSignal, FutureState, Signal, SignalRef, SignalTrait}};

use super::{widget_builder::WidgetBuilderTrait, Widget, WidgetTrait};

#[derive(Debug)]
pub struct Image {
    pub object: Signal<Object>
}

impl WidgetTrait for Image {
    fn get_objects(&self) -> Option<SignalRef<Object>> {
        Some(self.object.get())
    }
}

#[derive(Debug, Default)]
pub struct ImageBuilder {
    pub x: Signal<i32>,
    pub y: Signal<i32>,
    pub width: Signal<u32>,
    pub height: Signal<u32>,
    pub source: Signal<ByteSource>
}

impl WidgetBuilderTrait for ImageBuilder {
    fn build(self, _backend: &Backend, window: Option<&Window>) -> Widget {
        let decoder = FutureSignal::empty();

        let clone = decoder.clone();
        self.source.relative(move |source| {
            let source = source.clone();
            clone.set(async move {
                let bytes = source.get().await.unwrap();
                let image = CacheableImage::from_encoded(&bytes);

                image
            });
        });

        let window_id = window.map(|w| w.id());
        decoder.subscribe(move |state| if let FutureState::Completed(_) = state {
            if let Some(win) = window_id.clone() {
                crate::global_send(Event::Custom(CustomEvent::Redraw(win)));
            }
        });

        let combined = (self.x, self.y, self.width, self.height, decoder.relative(|state| state.clone()));

        let object = combined.relative(move |(x,y, w, h, image)| {
            let (x, y, w, h) = (**x, **y, **w, **h);

            match image.as_ref() {
                FutureState::Running => {
                    Object::rectangle(x, y, w, h, crate::LOADING_COLOR, None)
                },
                FutureState::Completed(image) => {
                    Object::image(x, y, w, h, image.clone())
                }
            }
        });

        Widget::Image(Image { object })
    }
}