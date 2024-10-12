use std::{fmt::{Debug, Display}, ops::Deref, sync::{Arc, RwLock, RwLockReadGuard}};

mod combined;
mod signal;
mod relative;
mod slots;

pub use {slots::*, signal::*};


#[derive(Debug)]
pub enum Signal<T> {
    Root(Arc<SignalInner<T>>),
    //Relative(Arc<RelativeSignal<T, U>>),
}


pub trait SignalTrait<'a, T, U> {
    fn get(&'a self) -> SignalRef<'a, U>;
    fn set(&self, data: T);
    fn subscribe(&self, callback: impl Fn(&U) + 'static);
    fn subscribe_slot(&self, slot: Slot<U>);
    fn notify(&self, callback: impl Fn() + 'static);
    fn notify_slot(&self, slot: NotifSlot);
    #[allow(unused)]
    fn relative<V: 'static>(&'a self, map_fn: impl Fn(&U) -> V + 'static) -> Signal<V> { unimplemented!() }
}


impl<T> Default for Signal<T> where SignalInner<T>: Default {
    fn default() -> Self {
        Self::Root(Default::default())
    }
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        match self {
            Signal::Root(inner) => Signal::Root(inner.clone()),
            //Signal::Relative(relative) => Signal::Relative(relative.clone())
        }
    }
}


impl<T> Signal<T> {
    pub fn new(data: T) -> Self {
        Self::Root(Arc::new(SignalInner {
            data: RwLock::new(data),
            slots: RwLock::new(Vec::new()),
            notif_slots: RwLock::new(Vec::new())
        }))
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


impl<T: 'static> SignalTrait<'_, T, T> for Signal<T> {
    fn get(&self) -> SignalRef<T> {
        match self {
            Signal::Root(root) => root.get(),
            //Signal::Relative(relative) => relative.get()
        }
    }

    fn set(&self, data: T) {
        match self {
            Signal::Root(root) => root.set(data),
            //Signal::Relative(relative) => relative.set(data)
        }
    }

    fn subscribe(&self, callback: impl Fn(&T) + 'static) {
        match self {
            Signal::Root(root) => root.subscribe(callback),
            //Signal::Relative(relative) => relative.subscribe(callback)
        }
    }

    fn subscribe_slot(&self, slot: Slot<T>) {
        match self {
            Signal::Root(root) => root.subscribe_slot(slot),
            //Signal::Relative(relative) => relative.subscribe_slot(slot),
        }
    }

    fn notify(&self, callback: impl Fn() + 'static) {
        match self {
            Signal::Root(root) => root.notify(callback),
            //Signal::Relative(relative) => relative.notify(callback)
        }
    }

    fn notify_slot(&self, slot: NotifSlot) {
        match self {
            Signal::Root(root) => root.notify_slot(slot),
            //Signal::Relative(relative) => relative.notify_slot(slot),
        }
    }

    fn relative<V: 'static>(&self, map_fn: impl Fn(&T) -> V + 'static) -> Signal<V> {
        let signal = Signal::new(map_fn(&self.get()));

        let clone = signal.clone();
        self.subscribe(move |data| {
            let result = map_fn(data);
            clone.set(result);
        });

        signal
    }
}