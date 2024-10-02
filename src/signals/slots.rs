use std::sync::Arc;


pub struct Slot<T> {
    callback: Arc<dyn Fn(&T) + 'static>
}

impl<T> Slot<T> {
    pub fn new(callback: impl Fn(&T) + 'static) -> Self {
        Self {
            callback: Arc::new(callback)
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



pub struct NotifSlot {
    callback: Arc<dyn Fn()>
}

impl NotifSlot {
    pub fn new(callback: impl Fn() + 'static) -> Self {
        Self {
            callback: Arc::new(callback)
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