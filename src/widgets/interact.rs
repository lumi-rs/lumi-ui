use std::ops::Deref;

use lumi2d::types::{Object, Position};

use crate::{backend::Backend, callback::Callback, elements::window::Window, signals::{Signal, SignalRef, SignalTrait, Slot}};

use super::{widget_builder::WidgetBuilderTrait, Widget, WidgetTrait};

#[derive(Debug)]
pub struct Interact {
}

impl WidgetTrait for Interact {
    fn get_objects(&self) -> Option<SignalRef<Object>> {
        None
    }
}

#[derive(Debug, Clone, Default)]
pub struct InteractBuilder {
    pub x: Signal<i32>,
    pub y: Signal<i32>,
    pub width: Signal<u32>,
    pub height: Signal<u32>,
    pub hovered: Signal<bool>,
    pub click_left: Signal<bool>,
    pub click_right: Signal<bool>,
    pub click_middle: Signal<bool>,
    pub clicked: Option<Callback>,
    pub right_clicked: Option<Callback>,
    pub mouse_drag: Option<Slot<Position<f64>>>
}

impl WidgetBuilderTrait for InteractBuilder {
    fn build(&self, _backend: &Backend, window: Option<&Window>) -> Widget {
        let cloned = self.clone();
        let state = &window.unwrap().innerest().state;
        let hovered = self.hovered.clone();
        let click_left = self.click_left.clone();
        let mouse_drag = self.mouse_drag.clone();
        let cursor_pos = state.cursor_pos.clone();
        let combined = (cloned.x, cloned.y, cloned.width, cloned.height);
        
        state.cursor_pos.subscribe(move |pos| {
            // TODO: Optimize this somehow? I feel like this is going to be slow
            let (x, y, w, h) = combined.get().cloned();
            
            let is_within = pos_within(*x, *y, *w, *h, pos);

            if *hovered.get() {
                if !is_within {
                    hovered.set(false);
                } else {
                }
            } else {
                if is_within {
                    hovered.set(true);
                }
            };

            if let Some(cb) = &cloned.mouse_drag {
                if *click_left.get() {
                    cb.invoke(pos);
                }
            }
        });

        let hovered = self.hovered.clone();
        state.click_left.subscribe(move |down| {
            let hover = *hovered.get();
            if *down {
                if hover {
                    cloned.click_left.set(true);
                    if let Some(cb) = &mouse_drag {
                        cb.invoke(cursor_pos.get().deref());
                    }
                }
            } else if *cloned.click_left.get() {
                if hover {
                    if let Some(cb) = &cloned.clicked {
                        cb.run();
                    }
                }
                cloned.click_left.set(false);
            }
        });

        let hovered = self.hovered.clone();
        state.click_right.subscribe(move |down| {
            let hover = *hovered.get();
            if *down {
                if hover {
                    cloned.click_right.set(true);

                    // Most right click actions are done on press instead of on release it seems, so we'll mimic that behaviour
                    if let Some(cb) = &cloned.right_clicked {
                        cb.run();
                    }
                }
            } else if *cloned.click_right.get() {
                cloned.click_right.set(false);
            }
        });

        let hovered = self.hovered.clone();
        state.click_middle.subscribe(move |down| {
            let hover = *hovered.get();
            if *down {
                if hover {
                    cloned.click_middle.set(true);
                }
            } else if *cloned.click_middle.get() {
                cloned.click_middle.set(false);
            }
        });
        

        Widget::Interact(Interact {  })
    }
}

#[inline]
fn pos_within(x: i32, y: i32, width: u32, height: u32, pos: &Position<f64>) -> bool {
    let tx = pos.x as i32;
    let ty = pos.y as i32;
    
    is_within(x, y, width, height, tx, ty)
}

#[inline]
fn is_within(x: i32, y: i32, width: u32, height: u32, tx: i32, ty: i32) -> bool {
    let (w, h) = (width as i32, height as i32);

    tx > x && ty > y && tx < x + w && ty < y + h
}