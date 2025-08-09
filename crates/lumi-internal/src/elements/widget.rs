use std::{fmt::Debug, sync::{Arc, RwLock, Weak}};

use crate::{backend::Backend, widgets::{widget_builder::{WidgetBuilder, WidgetBuilderTrait}, Widget}};

use super::{element::{Element, ElementRef, ElementRefTrait, ElementTrait}, element_builder::{ElementBuilder, ElementBuilderTrait}};


#[derive(Debug, Clone)]
pub struct WidgetElement {
    inner: Arc<WidgetElementInner>
}

pub type WidgetElementRef = Weak<WidgetElementInner>;

pub struct WidgetElementBuilder {
    widget: WidgetBuilder,
    children: RwLock<Vec<ElementBuilder>>
}


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

impl ElementBuilderTrait for Arc<WidgetElementBuilder> {
    fn children(&self) -> &RwLock<Vec<ElementBuilder>> {
        &self.children
    }

    fn build(&self, backend: &Backend, parent: Option<ElementRef>) -> Element {
        let children = self.children.read().unwrap();
        let window = parent.as_ref()
        .and_then(|p| p.upgrade_element())
        .and_then(|p| p.get_window());
        
        // TODO: Make this part of the trait somehow?
        let element = match &self.widget {
            WidgetBuilder::Window(builder) => Element::create_window(
                backend,
                parent,
                Vec::with_capacity(children.len()),
                builder
            ),
            widget => Element::new_widget(
                parent,
                Vec::with_capacity(children.len()),
                widget.build(backend, window.as_ref())
            ),
        };


        // The RwLockWriteGuard needs to be dropped before we can return the element
        {
            let mut new_children = element.children().write().unwrap();

            *new_children = children.iter()
            .map(|child| child.build(backend, Some(element.weak())))
            .collect();
        }

        element
    }
}

impl WidgetElementBuilder {
    pub fn new(children: Vec<ElementBuilder>, widget: WidgetBuilder) -> Self {
        Self {
            children: RwLock::new(children),
            widget
        }
    }
}

impl Debug for WidgetElementBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
        .field("widget", &self.widget)
        .field("children", &self.children.read().unwrap())
        .finish_non_exhaustive()
    }
}
