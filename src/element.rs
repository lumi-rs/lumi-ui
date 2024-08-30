use std::{fmt::Debug, sync::{Arc, RwLock, Weak}};

use crate::widgets::{window::Window, Widget, WidgetTrait};


#[derive(Debug, Clone)]
pub struct Element {
    pub(crate) inner: Arc<ElementInner>
}

pub(crate) struct ElementInner {
    pub(crate) parent: Option<Weak<ElementInner>>,
    pub(crate) widget: Widget,
    pub(crate) children: RwLock<Vec<Element>>
}

impl Element {
    pub(crate) fn new(parent: Option<Weak<ElementInner>>, children: Vec<Element>, widget: Widget) -> Self {
        Self {
            inner: Arc::new(ElementInner::new(
                parent,
                children,
                widget
            ))
        }
    }

    pub(crate) fn weak(&self) -> Weak<ElementInner> {
        Arc::downgrade(&self.inner)
    }

    pub fn root(widget: Widget) -> Self {
        Self::new(
            None,
            Vec::with_capacity(widget.expected_children()),
            widget
        )
    }

    pub fn child(&self, widget: Widget) -> Self {
        let element = Self::new(
            Some(Arc::downgrade(&self.inner)),
            Vec::with_capacity(widget.expected_children()),
            widget
        );

        self.inner.children.write().unwrap().push(element.clone());

        element
    }
}

impl ElementInner {
    pub(crate) fn new(parent: Option<Weak<ElementInner>>, children: Vec<Element>, widget: Widget) -> Self {
        Self {
            parent,
            children: RwLock::new(children),
            widget
        }
    }

    pub fn get_window(&self) -> Option<Window> {
        if let Widget::Window(window) = &self.widget {
            window.upgrade().map(|inner| Window { inner })
        } else {
            self.parent.as_ref()?
            .upgrade()?
            .get_window()
        }
    }
}

impl Debug for ElementInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
        .field("widget", &self.widget)
        .field("children", &self.children.read().unwrap())
        .finish_non_exhaustive()
    }
}
