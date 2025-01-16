use std::rc::Rc;

use super::{NotifSlot, Signal, SignalRef, SignalTrait, Slot};

#[derive(Debug)]
pub struct ConstSignal<T> {
    pub(crate) data: T
}

impl<T> ConstSignal<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> SignalTrait<'_, T, T> for ConstSignal<T> {
    fn get(&self) -> SignalRef<T> {
        SignalRef::Reference(&self.data)
    }

    fn set(&self, _data: T) {
        panic!("Attempted to write to a const Signal!\nUse a root Signal instead.");
    }

    fn subscribe(&self, _callback: impl Fn(&T) + 'static) {}

    fn subscribe_slot(&self, _slot: Slot<T>) {}

    fn notify(&self, _callback: impl Fn() + 'static) {}

    fn notify_slot(&self, _slot: NotifSlot) {}

    fn relative<V: 'static>(&self, map_fn: impl Fn(&T) -> V + 'static) -> Signal<V> {
        let data = map_fn(&self.get());
        
        Signal::Const(
            Rc::new(
                ConstSignal { data }
            )
        )
    }
}