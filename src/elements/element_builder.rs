use std::{fmt::Debug, sync::{Arc, RwLock, Weak}};

use crate::{backend::Backend, elements::element::{Element, ElementInner}, widgets::{root::RootBuilder, widget_builder::{WidgetBuilder, WidgetBuilderTrait}}};

use super::element::ElementTrait;

#[derive(Debug, Clone)]
pub struct ElementBuilder {
    inner: Arc<ElementBuilderInner>
}

struct ElementBuilderInner {
    widget: WidgetBuilder,
    children: RwLock<Vec<ElementBuilder>>
}


impl ElementBuilder {
    fn new(children: Vec<ElementBuilder>, widget: WidgetBuilder) -> Self {
        Self {
            inner: Arc::new(ElementBuilderInner {
                children: RwLock::new(children),
                widget
            })
        }
    }

    pub fn root() -> Self {
        Self::new(
            Vec::with_capacity(1),
            WidgetBuilder::Root(RootBuilder())
        )
    }

    pub fn child(&self, widget: WidgetBuilder) -> Self {
        let element = Self::new(
            Vec::with_capacity(widget.expected_children()),
            widget
        );

        self.inner.children.write().unwrap().push(element.clone());
        element
    }

    pub(crate) fn build(self, backend: &Backend, parent: Option<Weak<ElementInner>>) -> Element {
        let inner = Arc::into_inner(self.inner).unwrap();
        let children = inner.children.into_inner().unwrap();
        let window = parent.as_ref()
        .and_then(|p| p.upgrade())
        .and_then(|p| p.get_window());
        
        let element = match inner.widget {
            WidgetBuilder::Root(_) => Element::root(),
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
            let mut new_children = element.inner.children().write().unwrap();

            *new_children = children.into_iter()
            .map(|child| child.build(backend, Some(element.weak())))
            .collect();
        }

        element
    }
}

impl Debug for ElementBuilderInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
        .field("widget", &self.widget)
        .field("children", &self.children.read().unwrap())
        .finish_non_exhaustive()
    }
}
