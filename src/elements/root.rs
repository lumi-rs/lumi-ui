use std::sync::{RwLock, Weak};

use log::warn;

use super::element::{Element, ElementInner, ElementTrait};


#[derive(Debug)]
pub struct RootElement {
    pub(crate) children: RwLock<Vec<Element>>
}

impl ElementTrait for RootElement {
    fn children(&self) ->  &RwLock<Vec<Element>> {
        &self.children
    }
    fn parent(&self) ->  &Option<Weak<ElementInner>> {
        &None
    }
    fn remove(&self) {
        warn!("Tried to remove root element!")
    }
    fn identifier(&self) -> u64 {
        1
    }
}

impl RootElement {
    pub fn new() -> Self {
        Self {
            children: RwLock::new(Vec::with_capacity(1))
        }
    }
}