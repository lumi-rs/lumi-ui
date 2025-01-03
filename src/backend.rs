use std::{cell::RefCell, collections::HashMap, sync::{Arc, RwLockReadGuard, Weak}};

use log::info;
use lumi2d::{backend::errors::BackendError, prelude::*};

use crate::{custom_event::CustomEvent, elements::{element::ElementTrait, element_builder::ElementBuilder, window::{Window, WindowInner, WindowState}}};

pub struct Backend {
    pub(crate) backend: Arc<lumi2d::backend::Backend<CustomEvent>>,
    pub(crate) windows: RefCell<HashMap<WindowId, Window>>
}

impl Backend {
    pub fn init(callback: impl FnOnce(Backend) + Copy + Send + 'static) -> Result<(), BackendError> {
        info!("Initializing windowing backend...");
        lumi2d::backend::Backend::create_custom(move |lumi| {
            let backend = Self {
                backend: Arc::new(lumi),
                windows: RefCell::new(HashMap::new())
            };

            crate::GLOBAL_SENDER.set(backend.backend.sender()).unwrap();

            callback(backend);
        })
    }

    pub(crate) fn create_window_inner(&self, details: WindowDetails, state: WindowState) -> WindowInner {
        let lumi_win = self.backend.create_window(details);
        let renderer = lumi_win.create_renderer(&self.backend).unwrap();
        
        Window::create_inner(lumi_win, renderer, state)
    }

    pub fn register_font(&self, alias: &str, font_bytes: &[u8]) {
        self.renderer_data().register_font(font_bytes, alias);
    }

    pub fn register_default_font(&self, alias: &str, font_bytes: &[u8]) {
        self.renderer_data().register_default_font(font_bytes, alias);
    }

    pub fn run_ui(&self, builder: ElementBuilder) {
        let _element = builder.build(self, None);

        self.backend.subscribe_events(|events| {
            let mut grouped: HashMap<WindowId, Vec<WindowEvent>> = HashMap::new();

            let mut append_or_insert = |window_id, event| {
                if let Some(vec) = grouped.get_mut(&window_id) {
                    vec.push(event);
                } else {
                    grouped.insert(window_id, vec![event]);
                }
            };

            for event in events {
                match event {
                    Event::Backend(BackendEvent { event, window_id }) => {
                        append_or_insert(window_id, event);
                    },
                    Event::Custom(custom) => match custom {
                        CustomEvent::BackendEvent(BackendEvent { event, window_id }) => {
                            append_or_insert(window_id, event);
                        },
                        CustomEvent::Callback(fn_once) => fn_once(),
                        CustomEvent::Redraw(window) => append_or_insert(window, WindowEvent::Redraw)
                    },
                }
            }

            for (window, events) in grouped.into_iter() {
                let events = reverse_dedup_enums(events.into_iter());
                
                self.resolve_events(window, events);
            }
        })
    }
    
    fn resolve_events(&self, window: WindowId, events: impl DoubleEndedIterator<Item = WindowEvent>) {
        let result = self.windows.borrow()
        .get(&window)
        .map(|win| 
            win.process_events(self, events.map(|e| e.scale_with(win.scale())))
        );
        // Needs to be handled here so self.windows is no longer borrowed.
        if let Some(true) = result {
            self.take_window(&window).map(|win| {
                win.remove();
                win.close();
            });
            if self.windows.borrow().is_empty() {
                self.backend.exit();
            }
        }
    }

    pub(crate) fn take_window(&self, id: &WindowId) -> Option<Window> {
        self.windows.borrow_mut().remove(id)
    }

    pub fn renderer_data(&self) -> RwLockReadGuard<RendererData> {
        self.backend.renderer_data()
    }

    pub fn weak_inner(&self) -> Weak<lumi2d::types::Backend<CustomEvent>> {
        Arc::downgrade(&self.backend)
    }
}


impl Drop for Backend {
    fn drop(&mut self) {
        info!("Exiting...");
        self.backend.exit();
    }
}

fn reverse_dedup_enums<T>(iter: impl DoubleEndedIterator<Item = T>) -> impl DoubleEndedIterator<Item = T> {
    let mut known = Vec::new();
    
    iter.map(|el| Some(el)).filter_map(move |element| {
        let d = std::mem::discriminant(element.as_ref().unwrap());
        if known.contains(&d) {
            None
        } else {
            known.push(d);
            element
        }
    })
}