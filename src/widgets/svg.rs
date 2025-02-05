use lumi2d::types::{CacheableSvg, Event, Object};

use crate::{backend::Backend, byte_source::ByteSource, custom_event::CustomEvent, elements::window::Window, signals::{FutureSignal, FutureState, Signal, SignalRef, SignalTrait}};

use super::{widget_builder::WidgetBuilderTrait, Widget, WidgetTrait};

#[derive(Debug)]
pub struct Svg {
    pub object: Signal<Object>
}

impl WidgetTrait for Svg {
    fn get_objects(&self) -> Option<SignalRef<Object>> {
        Some(self.object.get())
    }
}

#[derive(Debug, Default, Clone)]
pub struct SvgBuilder {
    pub x: Signal<i32>,
    pub y: Signal<i32>,
    pub width: Signal<u32>,
    pub height: Signal<u32>,
    pub color: Signal<u32>,
    pub source: Signal<ByteSource>
}

impl WidgetBuilderTrait for SvgBuilder {
    fn build(&self, _backend: &Backend, window: Option<&Window>) -> Widget {
        let source = FutureSignal::empty();

        let clone = source.clone();
        self.source.relative(move |byte_source| {
            let byte_source = byte_source.clone();
            clone.set(async move {
                CacheableSvg::new(byte_source.get().await.unwrap())
            });
        });

        let window_id = window.map(|w| w.id());
        source.subscribe(move |state| if let FutureState::Completed(_) = state {
            if let Some(win) = window_id.clone() {
                crate::global_send(Event::Custom(CustomEvent::Redraw(win)));
            }
        });

        let combined = (self.x.clone(), self.y.clone(), self.width.clone(), self.height.clone(), self.color.clone(), source.relative(|state| state.clone()));

        let object = combined.relative(move |(x,y, w, h, color, source)| {
            let (x, y, w, h, color) = (**x, **y, **w, **h, **color);

            match source.as_ref() {
                FutureState::Running => {
                    Object::rectangle(x, y, w, h, crate::LOADING_COLOR, None)
                },
                FutureState::Completed(svg) => {
                    Object::svg(x, y, w, h, svg.clone(), color)
                }
            }
        });

        Widget::Svg(Svg { object })
    }
}