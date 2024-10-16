use std::{fmt::Debug, sync::RwLock};

use super::{NotifSlot, SignalRef, SignalTrait, Slot};


pub struct RootSignal<T> {
    pub(crate) data: RwLock<T>,
    pub(crate) slots: RwLock<Vec<Slot<T>>>,
    pub(crate) notif_slots: RwLock<Vec<NotifSlot>>
}

impl<T> RootSignal<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: RwLock::new(data),
            slots: RwLock::new(Vec::new()),
            notif_slots: RwLock::new(Vec::new())
        }
    }
}

impl<T> SignalTrait<'_, T, T> for RootSignal<T> {
    fn get(&self) -> SignalRef<T> {
        SignalRef::RwLock(self.data.read().unwrap())
    }

    fn set(&self, data: T) {
        let mut current = self.data.write().unwrap();
        *current = data;

        for slot in self.slots.read().unwrap().iter() {
            slot.invoke(&current);
        }
        drop(current);
        for notif_slot in self.notif_slots.read().unwrap().iter() {
            notif_slot.invoke()
        }
    }

    fn subscribe(&self, callback: impl Fn(&T) + 'static) {
        self.subscribe_slot(Slot::new(callback));
    }

    fn subscribe_slot(&self, slot: Slot<T>) {
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

impl<T: Debug> Debug for RootSignal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
