use std::ops::Add;

use lumi2d::renderer::{objects::Rounding, text::TextOptions};
use lumi_ui::{backend::Backend, byte_source::ByteSource, elements::{element_builder::ElementBuilder, window::{WindowBuilder, WindowState}}, signals::{Signal, SignalTrait}, widgets::{image::ImageBuilder, interact::InteractBuilder, rectangle::RectangleBuilder, svg::SvgBuilder, text::TextBuilder, widget_builder::WidgetBuilder}};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
    .with_level(log::LevelFilter::Debug)
    .env()
    .init()
    .unwrap_or_else(|err| eprintln!("Failed to initialize logger: {err}"));

    Backend::init(|backend| {
        backend.register_default_font("Inter", include_bytes!("Inter-Tight.ttf"));

        backend.run_ui({
            let root = ElementBuilder::root();
            let window_state = WindowState::default();
            let window = root.child(
                WidgetBuilder::Window(WindowBuilder {
                    state: window_state.clone(),
                    ..Default::default()
                })
            );

            let rect1 = RectangleBuilder {
                x: Signal::constant(100),
                y: Signal::constant(100),
                width: window_state.dimensions.relative(|dims| dims.width.saturating_sub(200)),
                height: window_state.dimensions.relative(|dims| dims.height.saturating_sub(200)),
                color: Signal::constant(0xDDDDDDDD),
                rounding: Signal::constant(Some(Rounding::new_uniform(10)))
            };
            let rect2 = RectangleBuilder {
                x: rect1.x.relative(|x| x + 50),
                y: rect1.y.relative(|y| y + 50),
                width: rect1.width.clone(),
                height: rect1.height.relative(|h| h.saturating_sub(100)),
                color: Signal::constant(0xFF11EEAA),
                rounding: Signal::constant(None)
            };
            let text1 = TextBuilder {
                x: rect2.x.clone(),
                y: rect2.y.clone(),
                width: rect2.width.clone(),
                text: Signal::constant("Hello, world!".to_string()),
                max_height: rect2.height.relative(|h| Some(*h)),
                options: Signal::constant(TextOptions {
                    size: 42.0,
                    italic: true,
                    underline: true,
                    ..Default::default()
                })
            };
            let image1 = ImageBuilder {
                x: rect2.x.clone(),
                y: rect2.y.relative(|y| y + 80),
                width: rect2.width.clone(),
                height: rect2.height.relative(|h| h.saturating_sub(80)),
                source: Signal::constant(ByteSource::bytes(include_bytes!("nori.gif")))
            };
            let svg1 = SvgBuilder {
                x: (rect2.x.clone(), rect2.width.clone()).relative(|(x, w)| (**x).add(**w as i32).saturating_sub(70)),
                y: rect2.y.relative(|y| y + 10),
                width: Signal::constant(60),
                height: Signal::constant(60),
                color: Signal::constant(0xEEEEEEFF),
                source: Signal::constant(ByteSource::bytes(include_bytes!("home.svg")))
            };
            let interact1 = InteractBuilder {
                x: Signal::constant(10),
                y: Signal::constant(10),
                width: Signal::constant(100),
                height: Signal::constant(100),
                ..Default::default()
            };
            let rect3 = RectangleBuilder {
                x: interact1.x.clone(),
                y: interact1.y.clone(),
                width: interact1.width.clone(),
                height: interact1.height.clone(),
                color: (interact1.hovered.clone(), interact1.click_left.clone()).relative(|(hovered, clicked)| if **clicked {
                    0xFFAAAAFF
                } else if **hovered {
                    0xFFFFFFFF
                } else {
                    0xAAAAAAFF
                }),
                rounding: Signal::constant(None),
            };
            
            window.child(
                rect1.into()
            ).child(
                rect2.into()
            ).child(
                text1.into()
            ).child(
                image1.into()
            ).child(
                svg1.into()
            ).child(
                interact1.into()
            ).child(
                rect3.into()
            );

            root
        });

    }).expect("Failed to initialize LumiUI!");
}