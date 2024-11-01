use std::sync::{Arc, RwLock, Weak};

use crate::widgets::Widget;

use super::element::{Element, ElementRef, ElementRefTrait, ElementTrait};


#[derive(Debug, Clone)]
pub struct WidgetElement {
    inner: Arc<WidgetElementInner>
}

pub type WidgetElementRef = Weak<WidgetElementInner>;

#[derive(Debug)]
pub struct WidgetElementInner {
    pub(crate) parent: Option<ElementRef>,
    pub(crate) widget: Widget,
    pub(crate) children: RwLock<Vec<Element>>,
    pub(crate) identifier: u64
}

impl WidgetElement {
    pub fn new(parent: Option<ElementRef>, widget: Widget, children: RwLock<Vec<Element>>) -> Self {
        Self {
            inner: Arc::new(WidgetElementInner {
                parent,
                widget,
                children,
                identifier: fastrand::u64(..)
            })
        }
    }

    pub fn widget(&self) -> &Widget {
        &self.inner.widget
    }
}

impl ElementRefTrait for WidgetElementRef {
    fn upgrade_element(&self) -> Option<Element> {
        self.upgrade().map(|inner| WidgetElement { inner }.into())
    }
}

impl ElementTrait for WidgetElement {
    fn children(&self) -> &RwLock<Vec<Element>> {
        &self.inner.children
    }
    fn parent(&self) -> &Option<ElementRef> {
        &self.inner.parent
    }
    fn identifier(&self) -> u64 {
        self.inner.identifier
    }
    fn render_into(&self, objects: &mut Vec<Element>) {
        objects.push(self.clone().into());

        for child in self.children().read().unwrap().iter() {
            child.render_into(objects)
        }
    }
    fn weak(&self) -> ElementRef {
        ElementRef::Widget(Arc::downgrade(&self.inner))
    }
}