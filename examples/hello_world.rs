use lumi2d::renderer::objects::Rounding;
use lumi_ui::{backend::Backend, elements::{element_builder::ElementBuilder, window::{WindowBuilder, WindowState}}, signals::{Signal, SignalTrait}, widgets::{rectangle::RectangleBuilder, widget_builder::WidgetBuilder}};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
    .with_level(log::LevelFilter::Debug)
    .env()
    .init()
    .unwrap_or_else(|err| eprintln!("Failed to initialize logger: {err}"));

    Backend::init(|backend| {
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
                x: Signal::new(100),
                y: Signal::new(100),
                width: window_state.dimensions.relative(|dims| dims.width.saturating_sub(200)),
                height: window_state.dimensions.relative(|dims| dims.height.saturating_sub(200)),
                color: Signal::new(0xFFFFFFFF),
                rounding: Signal::new(Some(Rounding::new_uniform(10)))
            };
            let rect2 = RectangleBuilder {
                x: rect1.x.relative(|x| x+50),
                y: rect1.y.relative(|y| y+50),
                width: rect1.width.clone(),
                height: rect1.height.relative(|h| h.saturating_sub(100)),
                color: Signal::new(0xFF11EEAA),
                rounding: Signal::new(None)
            };
            window.child(
                WidgetBuilder::Rectangle(rect1)
            ).child(
                WidgetBuilder::Rectangle(rect2)
            );

            root
        });

    }).expect("Failed to initialize LumiUI!");
}