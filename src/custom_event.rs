use lumi2d::types::{BackendEvent, WindowId};

pub enum CustomEvent {
    BackendEvent(BackendEvent),
    Callback(Box<dyn FnOnce() + Send + 'static>),
    Redraw(WindowId)
}