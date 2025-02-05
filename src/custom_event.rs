use std::fmt::Debug;

use lumi2d::types::{BackendEvent, WindowId};

pub enum CustomEvent {
    BackendEvent(BackendEvent),
    Callback(Box<dyn FnOnce() + Send + 'static>),
    Redraw(WindowId)
}

impl Debug for CustomEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(match self {
            CustomEvent::BackendEvent(_) => "BackendEvent",
            CustomEvent::Callback(_) => "Callback",
            CustomEvent::Redraw(_) => "Redraw",
        }).finish()
    }
}