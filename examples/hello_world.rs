use lumi2d::renderer::{objects::Rounding, text::TextOptions};
use lumi_ui::{backend::Backend, elements::{element_builder::ElementBuilder, window::{WindowBuilder, WindowState}}, signals::{Signal, SignalTrait}, widgets::{image::ImageBuilder, rectangle::RectangleBuilder, text::TextBuilder, widget_builder::WidgetBuilder}};
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
                bytes: Signal::constant(include_bytes!("./nori.gif").to_vec())
            };
            
            window.child(
                rect1.into()
            ).child(
                rect2.into()
            ).child(
                text1.into()
            ).child(
                image1.into()
            );

            root
        });

    }).expect("Failed to initialize LumiUI!");
}