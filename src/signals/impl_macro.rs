macro_rules! impl_signal {
    ( $(($typ:ident, $ident:ident, $alt:ident)),+ ) => {
        #[allow(unused)]
        use crate::{signals::*, *};

        impl<'a: 'b, 'b, $($typ: 'static),+>
            SignalTrait<'a, ($($typ),+), ($(SignalRef<'b, $typ>),+)>
        for
            ($(Signal<$typ>),+)
        {
            fn get(&'a self) -> SignalRef<'a, ($(SignalRef<'b, $typ>),+)> {
                let ($($ident),+) = self;
                
                SignalRef::Owned(($($ident.get()),+))
            }

            fn set(&self, data: ($($typ),+)) {
                let ($($ident),+) = self;
                let ($($alt),+) = data;

                $(
                    $ident.set($alt);
                )+
            }

            fn subscribe(&self, callback: impl Fn(&($(SignalRef<'b, $typ>),+)) + 'static) {
                self.subscribe_slot(Slot::new(callback))
            }

            fn subscribe_slot(&self, slot: Slot<($(SignalRef<'b, $typ>),+)>) {
                let ($($ident),+) = self;
                $(let $alt = $ident.clone();)+

                // Yes, this is bad. But the compiler hates me, and I hate lifetimes, and this seems to work, so...
                let cb: Slot<($(SignalRef<$typ>),+)> = unsafe { std::mem::transmute(slot) };

                let func = move || {
                    let cb: Slot<($(SignalRef<$typ>),+)> = unsafe { std::mem::transmute(cb.clone()) };

                    let args = ($(
                        $alt.get()
                    ),+);

                    cb.invoke(&args);
                };

                let slot = NotifSlot::new(func);
        
                $({
                    $ident.notify_slot(slot.clone());
                })+
            }
        
            fn notify(&self, callback: impl Fn() + 'static) {
                let slot = NotifSlot::new(callback);
                self.notify_slot(slot);
            }

            fn notify_slot(&self, slot: NotifSlot) {
                let ($($ident),+) = self;

                $(
                    $ident.notify_slot(slot.clone());
                )+
            }
        }
    };
}


impl_signal!((T, a, b), (U, c, d));
impl_signal!((T, a, b), (U, c, d), (V, e, f));
impl_signal!((T, a, b), (U, c, d), (V, e, f), (W, g, h));
impl_signal!((T, a, b), (U, c, d), (V, e, f), (W, g, h), (X, i, j));
impl_signal!((T, a, b), (U, c, d), (V, e, f), (W, g, h), (X, i, j), (Y, k, l));
