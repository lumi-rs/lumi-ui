use std::{fmt::Debug, sync::RwLock};

use enum_dispatch::enum_dispatch;

use crate::{backend::Backend, widgets::{Widget, WidgetTrait}};

use super::{root::*, widget::*, window::*};



#[enum_dispatch(ElementTrait)]
#[derive(Debug, Clone)]
pub enum Element {
    Root(RootElement),
    Widget(WidgetElement),
    Window(Window)
}

#[enum_dispatch(ElementRefTrait)]
#[derive(Debug)]
pub enum ElementRef {
    Root(RootElementRef),
    Widget(WidgetElementRef),
    Window(WindowRef)
}

#[enum_dispatch]
pub trait ElementRefTrait {
    fn upgrade(&self) -> Option<Element>;
}

#[enum_dispatch]
pub trait ElementTrait {
    fn parent(&self) -> &Option<ElementRef>;
    fn children(&self) -> &RwLock<Vec<Element>>;
    fn identifier(&self) -> u64;
    fn render_into(&self, objects: &mut Vec<Element>);
    fn weak(&self) -> ElementRef;
    fn remove(&self) {
        if let Some(parent) = self.parent().as_ref().and_then(|p| p.upgrade()) {
            let mut children = parent.children().write().unwrap();
            let index = children.iter().position(|child| {
                self.identifier() == child.identifier()
            });
            children.remove(index.unwrap());
        }
    }
}

impl Element {
    pub(crate) fn new_widget(parent: Option<ElementRef>, children: Vec<Element>, widget: Widget) -> Self {
        Self::new(
            parent,
            children,
            widget
        )
    }

    pub(crate) fn create_window(backend: &Backend, parent: Option<ElementRef>, children: Vec<Element>, builder: WindowBuilder) -> Self {
        let inner = backend.create_window_inner(builder.details, builder.state);
        let window = Window::create(inner, parent, children);

        window.render(Vec::new()).unwrap(); // Draw once, otherwise the window won't be shown yet on some platforms
        backend.windows.borrow_mut().insert(window.id(), window.clone());
        
        Element::Window(window)
    }

    pub fn root() -> Self {
        Element::Root(RootElement::new())
    }

    pub fn child(&self, widget: Widget) -> Self {
        let element = Self::new_widget(
            Some(self.weak()),
            Vec::with_capacity(widget.expected_children()),
            widget
        );

        self.children().write().unwrap().push(element.clone());

        element
    }

    pub(crate) fn new(parent: Option<ElementRef>, children: Vec<Element>, widget: Widget) -> Self {
        Self::Widget(
            WidgetElement::new(
                parent,
                widget,
                RwLock::new(children)
            )
        )
    }

    pub fn get_window(&self) -> Option<Window> {
        if let Element::Window(window) = &self {
            Some(window.clone()) //window.clone().upgrade().map(|inner| Window { inner })
        } else {
            self.parent().as_ref()?
            .upgrade()?
            .get_window()
        }
    }
}