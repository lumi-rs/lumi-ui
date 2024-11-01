use std::{fmt::{Debug, Display}, ops::Deref, sync::{Arc, RwLockReadGuard}};

mod combined;
mod root;
mod r#const;
mod relative;
mod slots;

use r#const::ConstSignal;

pub use {slots::*, root::*};


#[derive(Debug)]
pub enum Signal<T> {
    Root(Arc<RootSignal<T>>),
    Const(Arc<ConstSignal<T>>)
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


impl<T> Default for Signal<T> where T: Default {
    fn default() -> Self {
        Self::Root(
            Arc::new(
                RootSignal::new(T::default())
            )
        )
    }
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        match self {
            Signal::Root(inner) => Signal::Root(inner.clone()),
            Signal::Const(inner) => Signal::Const(inner.clone())
            //Signal::Relative(relative) => Signal::Relative(relative.clone())
        }
    }
}


impl<T> Signal<T> {
    pub fn new(data: T) -> Self {
        Self::Root(
            Arc::new(RootSignal::new(data))
        )
    }

    pub fn constant(data: T) -> Self {
        Self::Const(Arc::new(ConstSignal {
            data
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

    fn deref(&self) -> &T {
        match self {
            SignalRef::RwLock(guard) => guard,
            SignalRef::Ref(r) => r,
            SignalRef::Owned(owned) => owned,
        }
    }
}

impl<'a, T> AsRef<T> for SignalRef<'a, T> {
    fn as_ref(&self) -> &T {
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

impl<'a, T: Clone> SignalRef<'a, T> {
    pub fn cloned(&self) -> T {
        match self {
            SignalRef::RwLock(guard) => (*guard).clone(),
            SignalRef::Ref(r) => (*r).clone(),
            SignalRef::Owned(owned) => owned.clone(),
        }
    }
}


impl<T: 'static> SignalTrait<'_, T, T> for Signal<T> {
    fn get(&self) -> SignalRef<T> {
        match self {
            Signal::Root(root) => root.get(),
            Signal::Const(inner) => inner.get(),
            //Signal::Relative(relative) => relative.get()
        }
    }

    fn set(&self, data: T) {
        match self {
            Signal::Root(root) => root.set(data),
            Signal::Const(inner) => inner.set(data),
            //Signal::Relative(relative) => relative.set(data)
        }
    }

    fn subscribe(&self, callback: impl Fn(&T) + 'static) {
        match self {
            Signal::Root(root) => root.subscribe(callback),
            Signal::Const(inner) => inner.subscribe(callback),
            //Signal::Relative(relative) => relative.subscribe(callback)
        }
    }

    fn subscribe_slot(&self, slot: Slot<T>) {
        match self {
            Signal::Root(root) => root.subscribe_slot(slot),
            Signal::Const(inner) => inner.subscribe_slot(slot),
            //Signal::Relative(relative) => relative.subscribe_slot(slot),
        }
    }

    fn notify(&self, callback: impl Fn() + 'static) {
        match self {
            Signal::Root(root) => root.notify(callback),
            Signal::Const(inner) => inner.notify(callback),
            //Signal::Relative(relative) => relative.notify(callback)
        }
    }

    fn notify_slot(&self, slot: NotifSlot) {
        match self {
            Signal::Root(root) => root.notify_slot(slot),
            Signal::Const(inner) => inner.notify_slot(slot),
            //Signal::Relative(relative) => relative.notify_slot(slot),
        }
    }

    fn relative<V: 'static>(&self, map_fn: impl Fn(&T) -> V + 'static) -> Signal<V> {
        match self {
            Signal::Const(inner) => inner.relative(map_fn),
            _ => {
                let signal = Signal::new(map_fn(&self.get()));

                let clone = signal.clone();
                self.subscribe(move |data| {
                    let result = map_fn(data);
                    clone.set(result);
                });
        
                signal
            }
        }

    }
}