use std::{cell::{Cell, Ref}, fmt::{Debug, Display}, ops::Deref, rc::Rc, sync::{Arc, RwLockReadGuard}, time::{Duration, Instant}};

mod combined;
mod root;
mod r#const;
mod future;
mod relative;
mod slots;

use r#const::ConstSignal;
use num_traits::AsPrimitive;

use crate::{animations::easings::EasingFunction, frame_notifier::FrameListener};

pub use {slots::*, root::*, future::*};


#[derive(Debug)]
pub enum Signal<T> {
    Root(Rc<RootSignal<T>>),
    Const(Rc<ConstSignal<T>>),
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
            Rc::new(
                RootSignal::new(T::default())
            )
        )
    }
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Root(inner) => Self::Root(inner.clone()),
            Self::Const(inner) => Self::Const(inner.clone())
        }
    }
}


impl<T: 'static> Signal<T> {
    pub fn new(data: T) -> Self {
        Self::Root(
            Rc::new(RootSignal::new(data))
        )
    }

    pub fn constant(data: T) -> Self {
        Self::Const(
            Rc::new(ConstSignal {
                data
            })
        )
    }
}

impl<T: Clone + 'static + AsPrimitive<f32>> Signal<T> where f32: AsPrimitive<T> {
    pub fn animate(&self, duration: Duration, easing: EasingFunction) -> Self {
        let _clone = self.clone();
        let previous = Cell::new(self.get().cloned());
        let new_signal = Signal::new(self.get().cloned());
        let cloned_new = new_signal.clone();
        let easing = Rc::new(easing);

        self.subscribe(move |new| {
            let start = previous.replace(new.clone());
            let end = new.clone();
            let clone = cloned_new.clone();
            let easing = easing.clone();
            let start_time = Instant::now();

            dbg!(start.as_(), end.as_());

            let listener = FrameListener::new(move |time| {
                let progress = (time - start_time).div_duration_f32(duration).min(1.0);
                // log::debug!("Current progress: {:?}", progress);

                clone.set(easing.calculate(&start, &end, progress));
            }, Instant::now() + duration);

            crate::LOCAL_FRAME_NOTIFIER.with(|notifier| {
                notifier.add(listener);
            });
        });

        // self.relative(|input| easing.calculate(input, progress));
        
        new_signal
    }
}


#[derive(Debug)]
pub enum SignalRef<'a, T> {
    Owned(T),
    Reference(&'a T),
    Ref(Ref<'a, T>),
    RwLock(RwLockReadGuard<'a, T>)
}

impl<'a, T> Deref for SignalRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            Self::Owned(owned) => owned,
            Self::Reference(r) => r,
            Self::Ref(r) => r,
            Self::RwLock(guard) => guard
        }
    }
}

impl<'a, T> AsRef<T> for SignalRef<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Owned(owned) => owned,
            Self::Reference(r) => r,
            Self::Ref(r) => r,
            Self::RwLock(guard) => guard
        }
    }
}

impl<T> Display for SignalRef<'_, T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

impl<'a, T: Clone> Clone for SignalRef<'a, T> {
    fn clone(&self) -> Self {
        Self::Owned(self.cloned())
    }
}

impl<'a, T: Clone> SignalRef<'a, T> {
    pub fn cloned(&self) -> T {
        match self {
            Self::Owned(owned) => owned.clone(),
            Self::Reference(r) => (*r).clone(),
            Self::Ref(r) => (*r).clone(),
            Self::RwLock(guard) => (*guard).clone()
        }
    }
}


impl<T: 'static> SignalTrait<'_, T, T> for Signal<T> {
    fn get(&self) -> SignalRef<T> {
        match self {
            Signal::Root(root) => root.get(),
            Signal::Const(inner) => inner.get()
        }
    }

    fn set(&self, data: T) {
        match self {
            Signal::Root(root) => root.set(data),
            Signal::Const(inner) => inner.set(data)
        }
    }

    fn subscribe(&self, callback: impl Fn(&T) + 'static) {
        match self {
            Signal::Root(root) => root.subscribe(callback),
            Signal::Const(inner) => inner.subscribe(callback)
        }
    }

    fn subscribe_slot(&self, slot: Slot<T>) {
        match self {
            Signal::Root(root) => root.subscribe_slot(slot),
            Signal::Const(inner) => inner.subscribe_slot(slot)
        }
    }

    fn notify(&self, callback: impl Fn() + 'static) {
        match self {
            Signal::Root(root) => root.notify(callback),
            Signal::Const(inner) => inner.notify(callback)
        }
    }

    fn notify_slot(&self, slot: NotifSlot) {
        match self {
            Signal::Root(root) => root.notify_slot(slot),
            Signal::Const(inner) => inner.notify_slot(slot)
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