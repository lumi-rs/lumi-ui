use std::{cell::RefCell, collections::HashMap};

use log::info;
use lumi2d::backend::{errors::BackendError, events::WindowEvent, windows::{BackendEvent, WindowDetails, WindowId, WindowTrait}, BackendTrait};

use crate::{element_builder::ElementBuilder, widgets::window::Window};

pub struct Backend {
    pub(crate) backend: lumi2d::backend::Backend,
    pub(crate) windows: RefCell<HashMap<WindowId, Window>>
}

impl Backend {
    pub fn init(callback: impl FnOnce(Backend) + Copy + Send + 'static) -> Result<(), BackendError> {
        info!("Initializing windowing backend...");
        lumi2d::backend::Backend::create(move |lumi| {
            let backend = Self {
                backend: lumi,
                windows: RefCell::new(HashMap::new())
            };
            callback(backend);
        })
    }

    pub(crate) fn create_window(&self, details: WindowDetails) -> Window {
        let lumi_win = self.backend.create_window(details);
        let renderer = lumi_win.create_renderer().unwrap();
        
        let id = lumi_win.id();
        let window = Window::new(lumi_win, renderer);
        self.windows.borrow_mut().insert(id, window.clone());
        window
    }

    pub fn run_ui(&self, builder: ElementBuilder) {
        let _element = builder.build(self, None);

        self.backend.subscribe_events(|events| {
            let mut grouped: HashMap<WindowId, Vec<WindowEvent>> = HashMap::new();

            for BackendEvent { event, window_id } in events {
                if let Some(vec) = grouped.get_mut(&window_id) {
                    vec.push(event);
                } else {
                    grouped.insert(window_id, vec![event]);
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
            self.take_window(&window).map(|win| win.close());
            if self.windows.borrow().is_empty() {
                self.backend.exit();
            }
        }
    }

    pub(crate) fn take_window(&self, id: &WindowId) -> Option<Window> {
        self.windows.borrow_mut().remove(id)
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
    
    iter.rev().map(|el| Some(el)).filter_map(move |element| {
        let d = std::mem::discriminant(element.as_ref().unwrap());
        if known.contains(&d) {
            None
        } else {
            known.push(d);
            element
        }
    })
}