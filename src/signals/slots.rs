use std::sync::Arc;


pub(crate) struct Slot<T> {
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

pub(crate) struct NotifSlot {
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