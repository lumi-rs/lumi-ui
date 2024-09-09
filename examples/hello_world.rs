use lumi_ui::{backend::Backend, elements::{element_builder::ElementBuilder, window::WindowBuilder}, signals::Signal, widgets::{rectangle::RectangleBuilder, widget_builder::WidgetBuilder}};
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
            let window = root.child(
                WidgetBuilder::Window(WindowBuilder {
                    ..Default::default()
                })
            );
            window.child(
                WidgetBuilder::Rectangle(RectangleBuilder {
                    xywh: Signal::new((100, 200, 300, 400))
                })
            ).child(
                WidgetBuilder::Rectangle(RectangleBuilder {
                    xywh: Signal::new((20, 30, 40, 50))
                })
            );
            window.child(
                WidgetBuilder::Rectangle(RectangleBuilder {
                    xywh: Signal::new((400, 300, 200, 100))
                })
            );

            root
        });

    }).expect("Failed to initialize LumiUI!");
}