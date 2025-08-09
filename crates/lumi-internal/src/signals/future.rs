use std::{future::Future, marker::PhantomData, sync::RwLock};

use lumi2d::types::Event;

use crate::custom_event::CustomEvent;

use super::*;


pub struct FutureSignal<T, U> {
    pub(crate) data: Arc<RwLock<FutureState<T>>>,
    pub(crate) slots: Arc<RwLock<Vec<Slot<FutureState<T>>>>>,
    pub(crate) notif_slots: Arc<RwLock<Vec<NotifSlot>>>,
    pub(crate) _phantom: PhantomData<U>
}

impl<T: Send + Sync + 'static, U: Future<Output = T> + Send + 'static> FutureSignal<T, U> {
    pub fn new(data: U) -> Self {
        let signal = Self::empty();

        signal.set(data);

        signal
    }

    pub fn empty() -> Self {
        Self {
            data: Arc::new(RwLock::new(FutureState::Running)),
            slots: Arc::new(RwLock::new(Vec::new())),
            notif_slots: Arc::new(RwLock::new(Vec::new())),
            _phantom: PhantomData
        }
    }
    
    fn set_running_state(&self) {
        *self.data.write().unwrap() = FutureState::Running;
    }

    fn invoke(&self) {
        let current = self.data.read().unwrap();
    
        for slot in self.slots.read().unwrap().iter() {
            slot.invoke(&current);
        }
        drop(current);
        for notif_slot in self.notif_slots.read().unwrap().iter() {
            notif_slot.invoke()
        }
    }
}

impl<U: Send + Sync + 'static, T: Future<Output = U> + Send + 'static> SignalTrait<'_, T, FutureState<U>> for FutureSignal<U, T> {
    fn get(&self) -> SignalRef<FutureState<U>> {
        SignalRef::RwLock(self.data.read().unwrap())
    }

    fn set(&self, data: T) {
        self.set_running_state();

        let raw = Box::into_raw(Box::new(self.clone()));
        let pointer = raw as usize;

        let clone = self.clone();
        crate::THREAD_POOL.spawn_ok(async move {
            *clone.data.write().unwrap() = FutureState::Completed(data.await);
            
            let sender = crate::GLOBAL_SENDER.get().unwrap();
            sender.send(Event::Custom(CustomEvent::Callback(
                Box::new(move || {
                    let cast = pointer as *mut FutureSignal<U, T>;
                    let boxed = unsafe { Box::from_raw(cast) };
                    boxed.invoke();
                })
            ))).unwrap();
        });

        //crate::LOCAL_POOL.with(move |pool| {
        //    pool.spawner().spawn_local(handle.then(
        //        move |data| async move {
        //            *write_handle.write().unwrap() = FutureState::Completed(data);
        //            println!("done");
        //        }
        //    )).unwrap();
        //});
    }

    fn subscribe(&self, callback: impl Fn(&FutureState<U>) + 'static) {
        self.subscribe_slot(Slot::new(callback));
    }

    fn subscribe_slot(&self, slot: Slot<FutureState<U>>) {
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

    fn relative<V: 'static>(&self, map_fn: impl Fn(&FutureState<U>) -> V + 'static) -> Signal<V> {
        let signal = Signal::new(map_fn(&self.get()));

        let clone = signal.clone();
        self.subscribe(move |data| {
            let result = map_fn(data);
            clone.set(result);
        });

        signal
    }
}

#[derive(Debug)]
pub enum FutureState<T> {
    Running,
    Completed(T)
}

impl<T: Clone> Clone for FutureState<T> {
    fn clone(&self) -> Self {
        match self {
            FutureState::Running => FutureState::Running,
            FutureState::Completed(inner) => FutureState::Completed(inner.clone()),
        }
    }
}


impl<T: Debug, U> Debug for FutureSignal<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FutureSignal")
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

impl<T, U> Clone for FutureSignal<T, U> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            slots: self.slots.clone(),
            notif_slots: self.notif_slots.clone(),
            _phantom: PhantomData
        }
    }
}