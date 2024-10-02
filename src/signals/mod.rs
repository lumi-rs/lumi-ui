use std::{fmt::{Debug, Display}, ops::Deref, sync::{Arc, RwLock, RwLockReadGuard}};

mod impl_macro;
mod signal;
mod relative;
mod slots;

pub use {signal::*, relative::*};
pub use slots::*;


#[derive(Debug)]
pub enum Signal<T, U = T> {
    Root(Arc<SignalInner<T>>),
    Relative(Arc<RelativeSignal<T, U>>),
}


pub trait SignalTrait<'a, T, U> {
    fn get(&'a self) -> SignalRef<'a, U>;
    fn set(&self, data: T);
    fn subscribe(&self, callback: impl Fn(&U) + 'static);
    fn subscribe_slot(&self, slot: Slot<U>);
    fn notify(&self, callback: impl Fn() + 'static);
    fn notify_slot(&self, slot: NotifSlot);
}


impl<T, U> Default for Signal<T, U> where SignalInner<T>: Default {
    fn default() -> Self {
        Self::Root(Default::default())
    }
}

impl<T, U> Clone for Signal<T, U> {
    fn clone(&self) -> Self {
        match self {
            Signal::Root(inner) => Signal::Root(inner.clone()),
            Signal::Relative(relative) => Signal::Relative(relative.clone())
        }
    }
}


impl<T> Signal<T, T> {
    pub fn new(data: T) -> Self {
        Self::Root(Arc::new(SignalInner {
            data: RwLock::new(data),
            slots: RwLock::new(Vec::new()),
            notif_slots: RwLock::new(Vec::new())
        }))
    }
}

impl<T: 'static, U: 'static> Signal<T, U> {
    pub fn relative(root: Signal<T, T>, map: impl Fn(&T) -> U + 'static) -> Self {
        Self::Relative(RelativeSignal::new(root, map))
    }
}


#[derive(Debug)]
pub enum SignalRef<'a, T> {
    RwLock(RwLockReadGuard<'a, T>),
    Ref(&'a T),
    Owned(T),
}

impl<'a, T> Deref for SignalRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            SignalRef::RwLock(guard) => guard,
            SignalRef::Ref(r) => r,
            SignalRef::Owned(owned) => owned,
        }
    }
}

impl<T> Display for SignalRef<'_, T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}


impl<T: 'static> SignalTrait<'_, T, T> for Signal<T, T> {
    fn get(&self) -> SignalRef<T> {
        match self {
            Signal::Root(root) => root.get(),
            Signal::Relative(relative) => relative.get()
        }
    }

    fn set(&self, data: T) {
        match self {
            Signal::Root(root) => root.set(data),
            Signal::Relative(relative) => relative.set(data)
        }
    }

    fn subscribe(&self, callback: impl Fn(&T) + 'static) {
        match self {
            Signal::Root(root) => root.subscribe(callback),
            Signal::Relative(relative) => relative.subscribe(callback)
        }
    }

    fn subscribe_slot(&self, slot: Slot<T>) {
        match self {
            Signal::Root(root) => root.subscribe_slot(slot),
            Signal::Relative(relative) => relative.subscribe_slot(slot),
        }
    }

    fn notify(&self, callback: impl Fn() + 'static) {
        match self {
            Signal::Root(root) => root.notify(callback),
            Signal::Relative(relative) => relative.notify(callback)
        }
    }

    fn notify_slot(&self, slot: NotifSlot) {
        match self {
            Signal::Root(root) => root.notify_slot(slot),
            Signal::Relative(relative) => relative.notify_slot(slot),
        }
    }
}