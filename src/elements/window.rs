use lumi2d::{prelude::*, renderer::RResult, types::Window as LumiWindow};

use std::{ops::Deref, sync::{Arc, RwLock, Weak}};

use crate::{backend::Backend, signals::{Signal, SignalTrait}, widgets::{widget_builder::WidgetBuilderTrait, Widget, WidgetTrait}};

use super::element::*;



#[derive(Debug, Clone)]
pub struct Window {
    pub inner: Arc<WindowElement>
}

pub type WindowRef = Weak<WindowElement>;

#[derive(Debug)]
pub struct WindowElement {
    pub(crate) parent: Option<ElementRef>,
    pub(crate) inner: WindowInner, // TODO: Make this Send + Sync, somehow
    pub(crate) children: RwLock<Vec<Element>>
}

impl ElementRefTrait for WindowRef {
    fn upgrade_element(&self) -> Option<Element> {
        self.upgrade().map(|inner| Window { inner }.into())
    }
}

impl ElementTrait for Window {
    fn children(&self) -> &RwLock<Vec<Element>> {
        &self.inner.children
    }
    fn parent(&self) -> &Option<ElementRef> {
        &self.inner.parent
    }
    fn identifier(&self) -> u64 {
        Arc::as_ptr(&self.inner) as u64
    }
    fn render_into(&self, _: &mut Vec<Element>) {
    }
    fn weak(&self) -> ElementRef {
        ElementRef::Window(Arc::downgrade(&self.inner))
    }
}


#[derive(Debug)]
pub struct WindowInner {
    pub(crate) state: WindowState,
    pub(crate) window: LumiWindow,
    pub(crate) renderer: Renderer
}

#[derive(Debug, Clone)]
pub struct WindowState {
    pub dimensions: Signal<Dimensions>,
    pub cursor_pos: Signal<Position<f64>>,
    pub focused: Signal<bool>,
}


#[derive(Debug, Default)]
pub struct WindowBuilder {
    pub details: WindowDetails,
    pub state: WindowState
}

impl WidgetBuilderTrait for WindowBuilder {
    fn build(self, _: &Backend, _: Option<&Window>) -> Widget {
        unreachable!();
    }
}


impl Window {
    pub(crate) fn create_inner(window: LumiWindow, renderer: Renderer, state: WindowState) -> WindowInner {
        WindowInner { window, renderer, state }
    }

    pub(crate) fn create(inner: WindowInner, parent: Option<ElementRef>, children: Vec<Element>) -> Window {
        let element = WindowElement {
            parent,
            inner,
            children: RwLock::new(children)
        };

        Window { inner: Arc::new(element) }
    }

    #[allow(unused)]
    pub(crate) fn weak(&self) -> WindowRef {
        Arc::downgrade(&self.inner)
    }

    pub(crate) fn id(&self) -> WindowId {
        self.inner.inner.id()
    }

    /// Must be called with the window taken out of the backend's list AND the Element tree first!
    pub(crate) fn close(self) {
        Arc::into_inner(self.inner).unwrap().inner.close();
    }

    pub(crate) fn render(&self, data: &RendererData, objects: Vec<&Object>) -> RResult<()>{
        let inner = &self.inner.inner;
        inner.renderer.render(&inner.window, data, objects)
    }

    /// Returns true if the window should be closed.
    pub(crate) fn process_events(&self, backend: &Backend, events: impl Iterator<Item = WindowEvent>) -> bool {
        for event in events {
            match event {
                WindowEvent::CloseRequested => {
                    return true;
                },
                WindowEvent::Redraw => {
                    self.draw_children(&backend.renderer_data());
                },
                WindowEvent::WindowSize(dim) => {
                    self.inner.inner.resized(dim);
                },
                WindowEvent::CursorPos(pos) => {
                    self.inner.inner.state.cursor_pos.set(pos)
                },
                WindowEvent::FocusChange(focus) => {
                    self.inner.inner.state.focused.set(focus)
                }
                _ => {}
            }
        }
        
        self.draw_children(&backend.renderer_data());
        //let cursor = self.inner.inner.state.cursor_pos.get();
        //self.render(vec![&Objects::rectangle(cursor.x as _, cursor.y as _, 10, 10, 0xFFFFFFFF, None)]).unwrap();

        false
    }

    pub fn scale(&self) -> f32 {
        self.inner.inner.window.current_scale()
    }

    fn draw_children(&self, data: &RendererData) {
        let mut elements = Vec::new();

        for child in self.children().read().unwrap().iter() {
            child.render_into(&mut elements)
        }

        let objects = elements
        .iter()
        .filter_map(|element| {
            if let Element::Widget(widget_element) = element {
                Some(widget_element.widget().get_objects())
            } else { None }
        });

        let refs: Vec<_> = objects.collect();

        self.render(data, refs.iter().map(|o| o.deref()).collect()).unwrap();
    }

    pub fn renderer(&self) -> &Renderer {
        &self.inner.inner.renderer
    }
}

impl WindowInner {
    fn close(self) {
        self.window.close()
    }

    fn id(&self) -> WindowId {
        self.window.id()
    }

    fn resized(&self, size: Dimensions) {
        self.state.dimensions.set(size);
        self.renderer.recreate(&self.window);
    }
}


impl Default for WindowState {
    fn default() -> Self {
        Self {
            dimensions: Signal::new(Dimensions::new(0, 0)),
            cursor_pos: Signal::new(Position::new(0.0, 0.0)),
            focused: Signal::new(false)
        }
    }
}