use std::{cell::{RefCell, RefMut}, time::Instant};

pub struct FrameNotifier {
    pub listeners: RefCell<Vec<FrameListener>>
}

pub struct FrameListener {
    callback: Box<dyn FnMut(Instant)>,
    expires: Instant
}


impl FrameNotifier {
    pub const fn new() -> Self {
        Self {
            listeners: RefCell::new(Vec::new())
        }
    }

    pub fn get_mut(&self) -> RefMut<Vec<FrameListener>> {
        self.listeners.borrow_mut()
    }

    pub fn add(&self, listener: FrameListener) {
        self.get_mut().push(listener);
    }

    // returns true if there are still active listeners
    pub fn run(&self, time: Instant) -> bool {
        let mut lock = self.get_mut();
      
        for listener in lock.iter_mut() {
            (listener.callback)(time)
        }

        lock.retain(|listener| {
            listener.expires > time
        });

        lock.len() > 0
    }
}


impl FrameListener {
    pub fn new(callback: impl Fn(Instant) + 'static, expires: Instant) -> Self {
        Self {
            callback: Box::new(callback),
            expires
        }
    }
}