use lumi_ui::{backend::Backend, elements::{element_builder::ElementBuilder, window::WindowBuilder}, widgets::{rectangle::RectangleBuilder, widget_builder::WidgetBuilder}};
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
            root.child(
                WidgetBuilder::Window(WindowBuilder {
                    ..Default::default()
                })
            );
            root.child(
                WidgetBuilder::Window(WindowBuilder {
                    ..Default::default()
                })
            );
            root.child(
                WidgetBuilder::Rectangle(RectangleBuilder {})
            ).child(
                WidgetBuilder::Rectangle(RectangleBuilder {})
            );
            root.child(
                WidgetBuilder::Rectangle(RectangleBuilder {})
            );

            root
        });

    }).expect("Failed to initialize LumiUI!");
}