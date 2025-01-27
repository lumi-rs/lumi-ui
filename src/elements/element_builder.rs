use std::{fmt::Debug, sync::{Arc, RwLock}};

use enum_dispatch::enum_dispatch;

use crate::{backend::Backend, elements::element::Element, widgets::widget_builder::{WidgetBuilder, WidgetBuilderTrait}};

use super::{element::ElementRef, root::RootElementBuilder, widget::WidgetElementBuilder};

#[enum_dispatch(ElementBuilderTrait)]
#[derive(Debug, Clone)]
pub enum ElementBuilder {
    Root(Arc<RootElementBuilder>),
    Widget(Arc<WidgetElementBuilder>)
}


#[enum_dispatch]
pub trait ElementBuilderTrait {
    fn children(&self) -> &RwLock<Vec<ElementBuilder>>;
    fn build(self, backend: &Backend, parent: Option<ElementRef>) -> Element;
    // fn identifier(&self) -> u64;
}


impl ElementBuilder {
    fn new(children: Vec<ElementBuilder>, widget: WidgetBuilder) -> Self {
        Self::Widget(
            Arc::new(WidgetElementBuilder::new(children, widget))
        )
    }

    pub fn root() -> Self {
        Self::Root(Arc::new(RootElementBuilder::new()))
    }

    pub fn child(&self, element: ElementBuilder) -> Self {
        self.children().write().unwrap().push(element.clone());
        element
    }

    pub fn child_widget(&self, widget: WidgetBuilder) -> Self {
        let element = Self::new(
            Vec::with_capacity(widget.expected_children()),
            widget
        );

        self.child(element)
    }
}

