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
    drop(a);

    println!("Testing relative... nyi");
    

    one.set(5);
    one.set(7);
    two.set("test complete");
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