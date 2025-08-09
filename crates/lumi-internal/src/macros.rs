
#[macro_export]
macro_rules! use_signals {
    ([ $($sig:ident),+ ] $callback:expr) => {{
        use ::clone_macro::clone;
        use ::lumi_ui::signals::SignalTrait;

        let test = clone!([$($sig),*], move || {
            let _ = $callback(
                $(
                    $sig.get()
                ),*
            );
        });

        $({
            $sig.notify(test.clone());
        })*
    }};
}