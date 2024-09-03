use std::{fmt::Debug, sync::{Arc, RwLock, Weak}};

use enum_dispatch::enum_dispatch;

use crate::{backend::Backend, widgets::{Widget, WidgetTrait}};

use super::{root::RootElement, widget::WidgetElement, window::{Window, WindowBuilder}};


#[derive(Debug, Clone)]
pub struct Element {
    pub(crate) inner: Arc<ElementInner>
}


#[enum_dispatch(ElementTrait)]
#[derive(Debug)]
pub enum ElementInner {
    Root(RootElement),
    Widget(WidgetElement),
    Window(Window)
}


#[enum_dispatch]
pub trait ElementTrait {
    fn parent(&self) -> &Option<Weak<ElementInner>>;
    fn children(&self) -> &RwLock<Vec<Element>>;
    fn remove(&self);
    fn identifier(&self) -> u64;
}

impl Element {
    pub fn new(inner: ElementInner) -> Self {
        Self { inner: Arc::new(inner) }
    }

    pub(crate) fn new_widget(parent: Option<Weak<ElementInner>>, children: Vec<Element>, widget: Widget) -> Self {
        Self::new(
            ElementInner::new(
                parent,
                children,
                widget
            )
        )
    }

    pub(crate) fn create_window(backend: &Backend, parent: Option<Weak<ElementInner>>, children: Vec<Element>, builder: WindowBuilder) -> Self {
        let inner = backend.create_window_inner(builder.details);
        let window = Window::create(inner, parent, children);

        window.render(Vec::new()).unwrap(); // Draw once, otherwise the window won't be shown yet on some platforms
        backend.windows.borrow_mut().insert(window.id(), window.clone());
        
        Element {
            inner: Arc::new(ElementInner::Window(window))
        }
    }

    pub(crate) fn weak(&self) -> Weak<ElementInner> {
        Arc::downgrade(&self.inner)
    }

    pub fn root() -> Self {
        Self::new(ElementInner::Root(RootElement::new()))
    }

    pub fn child(&self, widget: Widget) -> Self {
        let element = Self::new_widget(
            Some(Arc::downgrade(&self.inner)),
            Vec::with_capacity(widget.expected_children()),
            widget
        );

        self.inner.children().write().unwrap().push(element.clone());

        element
    }
}

impl ElementInner {
    pub(crate) fn new(parent: Option<Weak<ElementInner>>, children: Vec<Element>, widget: Widget) -> Self {
        Self::Widget(
            WidgetElement {
                parent,
                children: RwLock::new(children),
                widget
            }
        )
    }

    pub fn get_window(&self) -> Option<Window> {
        if let ElementInner::Window(window) = &self {
            Some(window.clone()) //window.clone().upgrade().map(|inner| Window { inner })
        } else {
            self.parent().as_ref()?
            .upgrade()?
            .get_window()
        }
    }
}