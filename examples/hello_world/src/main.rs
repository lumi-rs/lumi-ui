use std::{ops::Add, sync::Arc, time::Duration};

use lumi_ui::lumi2d::renderer::{objects::Rounding, text::TextOptions};
use lumi_ui::lumi2d::types::Position;
use lumi_ui::{backend::Backend, byte_source::ByteSource, callback::Callback, elements::{dynamic::DynamicElementBuilder, element_builder::ElementBuilder, window::{WindowBuilder, WindowState}}, signals::{Signal, SignalTrait, Slot}, widgets::{image::ImageBuilder, interact::InteractBuilder, rectangle::RectangleBuilder, svg::SvgBuilder, text::TextBuilder, widget_builder::WidgetBuilder}};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
    .with_level(log::LevelFilter::Debug)
    .with_module_level("naga", log::LevelFilter::Warn) // Slows down window creation otherwise
    .with_module_level("wgpu_core", log::LevelFilter::Warn)
    .with_module_level("wgpu_hal", log::LevelFilter::Warn)
    .env()
    .init()
    .unwrap_or_else(|err| eprintln!("Failed to initialize logger: {err}"));

    Backend::init(|backend| {
        backend.register_default_font("Inter", include_bytes!("../../Inter-Tight.ttf"));

        backend.run_ui({
            let root = ElementBuilder::root();
            let window_state = WindowState::default();
            let window = root.child_widget(
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
                source: Signal::constant(ByteSource::bytes(include_bytes!("../../nori.gif")))
            };
            let svg1 = SvgBuilder {
                x: (rect2.x.clone(), rect2.width.clone()).relative(|(x, w)| (**x).add(**w as i32).saturating_sub(70)),
                y: rect2.y.relative(|y| y + 10),
                width: Signal::constant(60),
                height: Signal::constant(60),
                color: Signal::constant(0xEEEEEEFF),
                source: Signal::constant(ByteSource::bytes(include_bytes!("../../home.svg")))
            };
            let interact1 = InteractBuilder {
                x: Signal::constant(10),
                y: Signal::constant(10),
                width: Signal::constant(100),
                height: Signal::constant(100),
                clicked: Some(Callback::new(|| {
                    println!("Released left click!");
                })),
                right_clicked: Some(Callback::new(|| {
                    println!("Right clicked!");
                })),
                ..Default::default()
            };

            let rect3 = RectangleBuilder {
                x: interact1.x.clone(),
                y: interact1.y.clone(),
                width: interact1.width.clone(),
                height: interact1.height.clone(),
                color: (interact1.hovered.clone(), interact1.click_left.clone())
                .relative(|(hovered, clicked)| if **clicked {
                    0xFFAAAAFF
                } else if **hovered {
                    0xFFFFFFFF
                } else {
                    0xAAAAAAFF
                }),
                rounding: Signal::constant(None),
            };

            let switch_state = Signal::new(false);

            let state_clone = switch_state.clone();
            let switch = InteractBuilder {
                x: Signal::constant(20),
                y: Signal::constant(300),
                width: Signal::constant(50),
                height: Signal::constant(50),
                clicked: Some(Callback::new(move || {
                    let current = *state_clone.get();
                    state_clone.set(!current);
                })),
                ..Default::default()
            };
            let switch_bg = RectangleBuilder {
                x: Signal::constant(20),
                y: Signal::constant(300),
                width: Signal::constant(50),
                height: Signal::constant(50),
                color: switch_state.relative(|state| {
                    if *state {
                        0x00FF00FF
                    } else {
                        0xFF0000FF
                    }
                }),
                ..Default::default()
            };

            let dynamic_element = Arc::new(DynamicElementBuilder::new(
                switch_state,
                move |state, parent| {
                    if *state {
                        let window = WindowBuilder {
                            ..Default::default()
                        };

                        let text = TextBuilder {
                            x: window.state.dimensions.relative(|dim| dim.width as i32 - 100),
                            y: Signal::constant(20),
                            width: Signal::constant(50),
                            text: Signal::constant("Hehe window".to_string()),
                            ..Default::default()
                        };

                        parent.child_widget(window.into()).child_widget(text.into());
                    } else {
                        let rect = TextBuilder {
                            x: Signal::constant(20),
                            y: Signal::constant(350),
                            width: Signal::constant(150),
                            text: Signal::constant("Toggle window!".to_string()),
                            ..Default::default()
                        };

                        parent.child_widget(rect.into());
                    }
                }
            ));

            
            window.child_widget(
                rect1.into()
            ).child_widget(
                rect2.into()
            ).child_widget(
                text1.into()
            ).child_widget(
                image1.into()
            ).child_widget(
                svg1.into()
            ).child_widget(
                interact1.into()
            ).child_widget(
                rect3.into()
            ).child_widget(
                switch.into()
            ).child_widget(
                switch_bg.into()
            ).child(dynamic_element.into());

            slider(window.clone());

            root
        });
    }).expect("Failed to initialize LumiUI!");
}

fn slider(parent: ElementBuilder) {
    let (x, y, width, height) = (150, 20, 200, 30);
    let point_x = Signal::new(x - height as i32 / 2);
    let progress = Signal::new(0.0);

    let rect = RectangleBuilder {
        x: Signal::constant(x),
        y: Signal::constant(y + height as i32 / 4),
        width: Signal::constant(width),
        height: Signal::constant(height/2),
        color: Signal::constant(0xAAAAAAFF),
        rounding: Signal::constant(Some(Rounding::new_uniform(10))),
        ..Default::default()
    };

    let overlay = RectangleBuilder {
        color: Signal::constant(0xFF33FFFF),
        width: progress.relative(move |prog| {
            (width as f64 * prog) as u32
        }).animate(Duration::from_secs(3), lumi_ui::animations::easings::EasingFunction::Linear),
        ..rect.clone()
    };

    let point = RectangleBuilder {
        x: point_x.clone(),
        y: Signal::constant(y),
        width: Signal::constant(height),
        height: Signal::constant(height),
        color: Signal::constant(0xFF33FFFF),
        rounding: Signal::constant(Some(Rounding::new_uniform(100)))
    };

    let text = TextBuilder {
        x: Signal::constant(x + width as i32 + 20),
        y: Signal::constant(y),
        text: progress.relative(|prog| {
            format!("{:.1}%", prog * 100.0)
        }),
        options: Signal::constant(TextOptions {
            size: 20.0,
            ..Default::default()
        }),
        width: Signal::constant(80),
        ..Default::default()
    };

    let interact = InteractBuilder {
        x: Signal::constant(x),
        y: Signal::constant(y),
        width: Signal::constant(width),
        height: Signal::constant(height),
        mouse_drag: Some(Slot::new(move |pos: &Position<f64>| {
            let new_x = (pos.x as i32).clamp(x, x + width as i32);
            point_x.set(new_x - height as i32 / 2);

            let offset = new_x - x;
            progress.set(offset as f64 / width as f64);
        })),
        ..Default::default()
    };


    parent
    .child_widget(rect.into())
    .child_widget(overlay.into())
    .child_widget(interact.into())
    .child_widget(point.into())
    .child_widget(text.into());
}