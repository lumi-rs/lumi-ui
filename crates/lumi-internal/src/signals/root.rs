use std::{cell::RefCell, fmt::Debug};

use super::{NotifSlot, SignalRef, SignalTrait, Slot};


pub struct RootSignal<T> {
    pub(crate) data: RefCell<T>,
    pub(crate) slots: RefCell<Vec<Slot<T>>>,
    pub(crate) notif_slots: RefCell<Vec<NotifSlot>>
}

impl<T> RootSignal<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: RefCell::new(data),
            slots: RefCell::new(Vec::new()),
            notif_slots: RefCell::new(Vec::new())
        }
    }
}

impl<T> SignalTrait<'_, T, T> for RootSignal<T> {
    fn get(&self) -> SignalRef<T> {
        SignalRef::Ref(self.data.borrow())
    }

    fn set(&self, data: T) {
        let mut current = self.data.borrow_mut();
        *current = data;

        for slot in self.slots.borrow().iter() {
            slot.invoke(&current);
        }
        drop(current);
        for notif_slot in self.notif_slots.borrow().iter() {
            notif_slot.invoke()
        }
    }

    fn subscribe(&self, callback: impl Fn(&T) + 'static) {
        self.subscribe_slot(Slot::new(callback));
    }

    fn subscribe_slot(&self, slot: Slot<T>) {
        let mut slots = self.slots.borrow_mut();
        slots.push(slot);
    }

    fn notify(&self, callback: impl Fn() + 'static) {
        self.notify_slot(NotifSlot::new(callback));
    }

    fn notify_slot(&self, slot: NotifSlot) {
        let mut slots = self.notif_slots.borrow_mut();
        slots.push(slot);
    }
}

impl<T: Debug> Debug for RootSignal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SignalInner")
        .field("data", &self.data)
        .field(
            "Callback count", 
            &self.slots.borrow().len().to_string()
        ).field(
            "Notifier count", 
            &self.notif_slots.borrow().len().to_string()
        ).finish()
    }
}
