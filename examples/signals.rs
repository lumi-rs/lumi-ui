use lumi_ui::{signals::{Signal, SignalTrait}, use_signals};

fn main() {
    let (one, two) = (Signal::new(1), Signal::new("test"));

    (one.clone(), two.clone()).subscribe(|(a, b)| {
        dbg!(a, b);
    });

    one.set(5);
}

fn _old_main() {
    let a = Signal::new(2);
    let relative = Signal::relative(a.clone(), |a| a + 4);
    let b = Signal::new("hola");
    
    let _ret = use_signals!([a, b, relative] |a, b, relative| {
        println!("{a} -> {relative}; {b}")
    });

    a.set(8);
    b.set("world");
    a.set(9);
}