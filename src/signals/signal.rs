use std::{fmt::Debug, sync::{Arc, RwLock}};

use super::{NotifSlot, SignalRef, SignalTrait, Slot};


pub struct SignalInner<T> {
    pub(crate) data: RwLock<T>,
    pub(crate) slots: RwLock<Vec<Slot<T>>>,
    pub(crate) notif_slots: RwLock<Vec<NotifSlot>>
}

impl<T> SignalTrait<T, T> for Arc<SignalInner<T>> {
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
        let mut slots = self.slots.write().unwrap();
        slots.push(Slot::new(callback));
    }

    fn notify(&self, callback: impl Fn() + 'static) {
        let mut slots = self.notif_slots.write().unwrap();
        slots.push(NotifSlot::new(callback));
    }
}

impl<T: Debug> Debug for SignalInner<T> {
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
