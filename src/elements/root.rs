use std::sync::{Arc, RwLock, Weak};

use log::warn;

use crate::backend::Backend;

use super::{element::{Element, ElementRef, ElementRefTrait, ElementTrait}, element_builder::{ElementBuilder, ElementBuilderTrait}};


#[derive(Debug, Clone)]
pub struct RootElement {
    children: Arc<RwLock<Vec<Element>>>
}

pub type RootElementRef = Weak<RwLock<Vec<Element>>>;

#[derive(Debug)]
pub struct RootElementBuilder {
    children: RwLock<Vec<ElementBuilder>>
}

impl ElementBuilderTrait for Arc<RootElementBuilder> {
    fn children(&self) ->  &RwLock<Vec<ElementBuilder>> {
        &self.children
    }

    fn build(&self, backend: &Backend, _parent: Option<ElementRef>) -> Element {
        let children = self.children.read().unwrap();

        let temp_children = Arc::new(RwLock::new(Vec::with_capacity(children.len())));
        let element = Element::Root(RootElement { children: temp_children.clone() });

        let new_children = children.iter().map(|builder| { 
            builder.build(backend, Some(element.weak().clone()))
        });
        
        *temp_children.write().unwrap() = new_children.collect();

        element
    }
}

impl RootElementBuilder {
    pub fn new() -> Self {
        Self {
            children: RwLock::new(Vec::with_capacity(1))
        }
    }
}

impl ElementRefTrait for RootElementRef {
    fn upgrade_element(&self) -> Option<Element> {
        self.upgrade().map(|children| RootElement { children }.into())
    }
}

impl ElementTrait for RootElement {
    fn children(&self) ->  &RwLock<Vec<Element>> {
        &self.children
    }
    fn parent(&self) ->  &Option<ElementRef> {
        &None
    }
    fn identifier(&self) -> u64 {
        0
    }
    fn render_into(&self, _: &mut Vec<Element>) {
        warn!("Tried to render the root element!")
    }
    fn weak(&self) -> ElementRef {
        ElementRef::Root(Arc::downgrade(&self.children))
    }
    fn remove(&self) {
        warn!("Tried to remove the root element!")
    }
}

impl RootElement {
    pub fn new() -> Self {
        Self {
            children: Arc::new(RwLock::new(Vec::with_capacity(1)))
        }
    }
}