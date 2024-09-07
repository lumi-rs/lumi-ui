use std::sync::{Arc, RwLock, Weak};

use log::warn;

use super::element::{Element, ElementRef, ElementRefTrait, ElementTrait};


#[derive(Debug, Clone)]
pub struct RootElement {
    pub(crate) children: Arc<RwLock<Vec<Element>>>
}

pub type RootElementRef = Weak<RwLock<Vec<Element>>>;

impl ElementRefTrait for RootElementRef {
    fn upgrade(&self) -> Option<Element> {
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