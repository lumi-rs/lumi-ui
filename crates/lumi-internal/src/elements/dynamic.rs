use std::{fmt::Debug, rc::Rc, sync::{Arc, RwLock, Weak}};


use clone_macro::clone;

use crate::{backend::Backend, signals::{Signal, SignalTrait}};

use super::{element::{Element, ElementRef, ElementRefTrait, ElementTrait}, element_builder::{ElementBuilder, ElementBuilderTrait}};

#[derive(Debug, Clone)]
pub struct DynamicElement {
    inner: Arc<DynamicElementInner>
}

#[derive(Debug)]
pub struct DynamicElementInner {
    pub(crate) identifier: u64,
    pub(crate) parent: Option<ElementRef>,
    pub(crate) children: RwLock<Vec<Element>>
}

pub type DynamicElementRef = Weak<DynamicElementInner>;

impl ElementRefTrait for DynamicElementRef {
    fn upgrade_element(&self) -> Option<Element> {
        self.upgrade().map(|inner| DynamicElement { inner }.into())
    }
}

impl ElementTrait for DynamicElement {
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
        for child in self.children().read().unwrap().iter() {
            child.render_into(objects)
        }
    }

    fn weak(&self) -> ElementRef {
        ElementRef::Dynamic(Arc::downgrade(&self.inner))
    }
}

pub struct DynamicElementBuilder {
    callback: Box<dyn Fn(&Backend, Arc<DynamicElementInner>)>,
    child_container: ChildBuilderContainer
}

impl DynamicElementBuilder {
    pub fn new<T: 'static>(signal: Signal<T>, callback: impl Fn(&T, ElementBuilder) + 'static) -> Self {
        let container = Rc::new(callback);

        Self {
            callback: Box::new(move |backend, inner| {
                let backend = backend.clone();
                
                let rebuild_cb = clone!([container, backend, inner], move |val: &T| {
                    let child_container = ChildBuilderContainer::new();

                    let element_builder: ElementBuilder = child_container.clone().into();

                    container(val, element_builder.clone());

                    let p = Element::Dynamic(DynamicElement { inner: inner.clone() }); 

                    let new_children = child_container.build_children(&backend, Some(p.weak()));
                    
                    let mut children = inner.children.write().unwrap();

                    let old_children = std::mem::replace(
                        &mut *children,
                        new_children
                    );

                    drop(children);

                    for ch in old_children {
                        // ch.remove();
                        ch.destruct(&backend);
                    }
                });

                rebuild_cb(signal.get().as_ref());

                signal.subscribe(rebuild_cb);

            }),
            child_container: ChildBuilderContainer::new()
        }
    }
}

impl ElementBuilderTrait for Arc<DynamicElementBuilder> {
    fn children(&self) -> &RwLock<Vec<ElementBuilder>> {
        &self.child_container.children
    }

    fn build(&self, backend: &Backend, parent: Option<ElementRef>) -> Element {
        let dynamic = Arc::new(DynamicElementInner {
            identifier: fastrand::u64(..),
            parent: parent.clone(),
            children: RwLock::new(Vec::new()),
        });

        (self.callback)(backend, dynamic.clone());

        Element::Dynamic(DynamicElement { inner: dynamic })
    }
}

impl Debug for DynamicElementBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DynamicElementBuilder { callback: ... }")
    }
}

#[derive(Debug, Clone)]

pub struct ChildBuilderContainer {
    children: Arc<RwLock<Vec<ElementBuilder>>>,
}

impl ElementBuilderTrait for ChildBuilderContainer {
    fn children(&self) ->  &RwLock<Vec<ElementBuilder>> {
        &self.children
    }

    fn build(&self, _: &Backend, _: Option<ElementRef>) -> Element {
        unreachable!("Called build(...) on a ChildBuilderContainer!")
    }
}

impl ChildBuilderContainer {
    fn new() -> Self {
        Self {
            children: Arc::new(RwLock::new(Vec::new()))
        }
    }
 
    fn build_children(&self, backend: &Backend, parent: Option<ElementRef>) -> Vec<Element> {
        let built_children = self.children.read().unwrap().iter().map(|child| {
            child.build(backend, parent.clone())
        }).collect();

        built_children
    }
}