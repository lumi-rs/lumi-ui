use std::sync::{Arc, Weak};

use lumi2d::{backend::{events::WindowEvent, windows::{Window as LumiWindow, WindowDetails, WindowId, WindowTrait}}, renderer::{Renderer, RendererTrait}, structs::{Dimensions, Position}, Objects};

use crate::{backend::Backend, signals::Signal};
use super::{widget_builder::WidgetBuilderTrait, Widget, WidgetTrait};


pub type WindowRef = Weak<WindowInner>;


#[derive(Debug, Clone)]
pub struct Window {
    pub inner: Arc<WindowInner> // TODO: Make this Send + Sync, somehow
}

#[derive(Debug)]
pub struct WindowInner {
    pub(crate) state: WindowState,
    pub(crate) window: LumiWindow,
    pub(crate) renderer: Renderer
}

#[derive(Debug, Clone)]
pub struct WindowState {
    dimensions: Signal<Dimensions>,
    cursor_pos: Signal<Position<f64>>,
    focused: Signal<bool>,
}

impl WidgetTrait for WindowRef {

}

#[derive(Debug, Default)]
pub struct WindowBuilder {
    pub details: WindowDetails
}

impl WidgetBuilderTrait for WindowBuilder {
    fn build(self, backend: &Backend, _window: Option<&Window>) -> Widget {
        let window = backend.create_window(self.details);
        window.render(Vec::new()); // Draw once, otherwise the window won't be shown yet on some platforms

        Widget::Window(window.weak())
    }
}


impl Window {
    pub(crate) fn new(window: LumiWindow, renderer: Renderer) -> Self {
        let state = WindowState {
            dimensions: Signal::new(window.dimensions()),
            cursor_pos: Signal::new(Position::new(0.0, 0.0)),
            focused: Signal::new(false)
        };

        Self {
            inner: Arc::new(WindowInner { window, renderer, state })
        }
    }

    pub(crate) fn weak(&self) -> WindowRef {
        Arc::downgrade(&self.inner)
    }

    /// Must be called with the window taken out of the backend's list first!
    pub(crate) fn close(self) {
        Arc::into_inner(self.inner).unwrap().close();
    }

    pub(crate) fn render(&self, objects: Vec<&Objects>) {
        let inner = &self.inner;
        inner.renderer.render(&inner.window, objects).unwrap();
    }

    /// Returns true if the window should be closed.
    pub(crate) fn process_events(&self, _backend: &Backend, events: impl DoubleEndedIterator<Item = WindowEvent>) -> bool {
        for event in events {
            match event {
                WindowEvent::CloseRequested => {
                    return true;
                },
                WindowEvent::WindowSize(dim) => {
                    self.inner.resized(dim);
                },
                WindowEvent::CursorPos(pos) => {
                    self.inner.state.cursor_pos.set(pos)
                },
                WindowEvent::FocusChange(focus) => {
                    self.inner.state.focused.set(focus)
                }
                _ => {}
            }
        }

        let cursor = self.inner.state.cursor_pos.get();
        self.render(vec![&Objects::rectangle(cursor.x as _, cursor.y as _, 10, 10, 0xFFFFFFFF, None)]);

        false
    }

    pub fn scale(&self) -> f32 {
        self.inner.window.current_scale()
    }
}

impl WindowInner {
    fn close(self) {
        self.window.close()
    }

    #[allow(unused)]
    fn id(&self) -> WindowId {
        self.window.id()
    }

    fn resized(&self, size: Dimensions) {
        self.state.dimensions.set(size);
        self.renderer.recreate(&self.window);
    }
}