use std::{fmt::Debug, rc::Rc};


pub struct Slot<T> {
    callback: Rc<dyn Fn(&T) + 'static>
}

impl<T> Slot<T> {
    pub fn new(callback: impl Fn(&T) + 'static) -> Self {
        Self {
            callback: Rc::new(callback)
        }
    }

    pub fn invoke(&self, with: &T) {
        self.callback.as_ref()(with)
    }
}

impl<T> Clone for Slot<T> {
    fn clone(&self) -> Self {
        Self { callback: self.callback.clone() }
    }
}

impl<T> Debug for Slot<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Slot { ... }")
    }
}



pub struct NotifSlot {
    callback: Rc<dyn Fn()>
}

impl NotifSlot {
    pub fn new(callback: impl Fn() + 'static) -> Self {
        Self {
            callback: Rc::new(callback)
        }
    }

    pub fn invoke(&self) {
        self.callback.as_ref()()
    }
}

impl Clone for NotifSlot {
    fn clone(&self) -> Self {
        Self { callback: self.callback.clone() }
    }
}

impl Debug for NotifSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NotifSlot { ... }")
    }
}