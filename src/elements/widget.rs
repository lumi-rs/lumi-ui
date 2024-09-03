use std::sync::{RwLock, Weak};

use crate::widgets::Widget;

use super::element::{Element, ElementInner, ElementTrait};


#[derive(Debug)]
pub struct WidgetElement {
    pub(crate) parent: Option<Weak<ElementInner>>,
    pub(crate) widget: Widget,
    pub(crate) children: RwLock<Vec<Element>>
}

impl ElementTrait for WidgetElement {
    fn children(&self) ->  &RwLock<Vec<Element>> {
        &self.children
    }
    fn parent(&self) ->  &Option<Weak<ElementInner>> {
        &self.parent
    }
    fn remove(&self) {
        
    }
    fn identifier(&self) -> u64 {
        1
    }
}