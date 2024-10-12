use lumi_ui::{signals::{Signal, SignalTrait, Slot}, use_signals};

fn main() {
    let (one, two) = (Signal::new(1), Signal::new("test"));

    let combined = (one.clone(), two.clone());

    println!("testing subscribe...");
    combined.subscribe(|(a, b)| {
        println!("sub: a = {a}, b = {b}");
    });

    println!("testing subscribe slot...");
    combined.subscribe_slot(Slot::new(|(a, b)| {
        println!("slot: a = {a}, b = {b}");
    }));

    println!("Getting value...");
    let a = combined.get();
    println!("Got: {a:?}");
    drop(a);

    println!("Testing relative...");
    combined.relative(|a| println!("from relative: {a:?}"));

    one.set(5);
    one.set(7);
    two.set("test complete");
}

fn _old_main() {
    let a = Signal::new(2);
    let relative = Signal::relative(&a.clone(), |a| a.to_string());
    let b = Signal::new("hola");
    
    use_signals!([a, b, relative] |a, b, relative| {
        println!("{a} -> {relative}; {b}")
    });

    a.set(8);
    b.set("world");
    a.set(9);
}