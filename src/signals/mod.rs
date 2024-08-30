use std::{fmt::Debug, sync::{Arc, RwLock, RwLockReadGuard}};

#[derive(Debug, Clone)]
pub struct Signal<T> {
    inner: Arc<SignalInner<T>>
}

pub struct SignalInner<T> {
    data: RwLock<T>,
    slots: RwLock<Vec<Slot<T>>>,
    notif_slots: RwLock<Vec<NotifSlot>>
}

impl<T> Signal<T> {
    pub fn new(data: T) -> Self {
        Self {
            inner: Arc::new(SignalInner {
                data: RwLock::new(data),
                slots: RwLock::new(Vec::new()),
                notif_slots: RwLock::new(Vec::new())
            })
        }
    }

    pub fn get(&self) -> RwLockReadGuard<'_, T> {
        self.inner.data.read().unwrap()
    }

    pub fn set(&self, data: T) {
        let mut current = self.inner.data.write().unwrap();
        *current = data;

        for slot in self.inner.slots.read().unwrap().iter() {
            slot.invoke(&current);
        }
        drop(current);
        for notif_slot in self.inner.notif_slots.read().unwrap().iter() {
            notif_slot.invoke()
        }
    }

    pub fn subscribe(&self, callback: impl Fn(&T) + 'static) {
        let mut slots = self.inner.slots.write().unwrap();
        slots.push(Slot::new(callback));
    }

    pub fn notify(&self, callback: impl Fn() + 'static) {
        let mut slots = self.inner.notif_slots.write().unwrap();
        slots.push(NotifSlot::new(callback));
    }
}

impl<T: Debug> Debug for SignalInner<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SignalInner")
        .field("data", &self.data)
        .field(
            "Callback count", 
            &self.slots.read()
            .map_or(
                "Error".to_string(),
                |s| s.len().to_string()
            )
        ).field(
            "Notifier count", 
            &self.notif_slots.read()
            .map_or(
                "Error".to_string(),
                |s| s.len().to_string()
            )
        ).finish()
    }
}


pub struct Slot<T> {
    callback: Arc<dyn Fn(&T)>
}

impl<T> Slot<T> {
    fn new(callback: impl Fn(&T) + 'static) -> Self {
        Self {
            callback: Arc::new(callback)
        }
    }

    pub fn invoke(&self, with: &T) {
        self.callback.as_ref()(with)
    }
}

pub struct NotifSlot {
    callback: Arc<dyn Fn()>
}

impl NotifSlot {
    fn new(callback: impl Fn() + 'static) -> Self {
        Self {
            callback: Arc::new(callback)
        }
    }

    pub fn invoke(&self) {
        self.callback.as_ref()()
    }
}