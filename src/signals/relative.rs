use std::{fmt::Debug, sync::{Arc, RwLock}};

use super::{NotifSlot, Signal, SignalRef, SignalTrait, Slot};


pub struct RelativeSignal<T, U> {
    pub(crate) root: Signal<T, T>,
    pub(crate) map_fn: Box<dyn Fn(&T) -> U>,
    pub(crate) slots: RwLock<Vec<Slot<U>>>,
    pub(crate) notif_slots: RwLock<Vec<NotifSlot>>
}

impl<T: 'static, U: 'static> SignalTrait<'_, T, U> for Arc<RelativeSignal<T, U>> {
    fn get(&self) -> SignalRef<U> {
        SignalRef::Owned(self.map(&self.root.get()))
    }

    fn set(&self, data: T) {
        self.root.set(data);
    }

    fn subscribe(&self, callback: impl Fn(&U) + 'static) {
        self.subscribe_slot(Slot::new(callback));
    }

    fn subscribe_slot(&self, slot: Slot<U>) {
        let mut slots = self.slots.write().unwrap();
        slots.push(slot);
    }

    fn notify(&self, callback: impl Fn() + 'static) {
        self.notify_slot(NotifSlot::new(callback));
    }

    fn notify_slot(&self, slot: NotifSlot) {
        let mut slots = self.notif_slots.write().unwrap();
        slots.push(slot);
    }
}

impl<T: 'static, U: 'static> RelativeSignal<T, U> {
    pub fn new(root: Signal<T, T>, map: impl Fn(&T) -> U + 'static) -> Arc<Self> {
        let relative = Arc::new(Self {
            root: root.clone(),
            map_fn: Box::new(map),
            slots: RwLock::new(Vec::new()),
            notif_slots: RwLock::new(Vec::new())
        });

        let cloned = relative.clone();
        root.subscribe(move |data| {
            cloned.invoke(data);
        });

        relative
    }

    fn invoke(&self, data: &T) {
        let mapped = self.map(data);
        for slot in self.slots.read().unwrap().iter() {
            slot.invoke(&mapped);
        }
    }

    fn map(&self, data: &T) -> U {
        self.map_fn.as_ref()(data)
    }
}

impl<T, U> Debug for RelativeSignal<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RelativeSignal")
        .finish_non_exhaustive() // TODO
    }
}