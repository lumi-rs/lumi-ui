use lumi_ui::{signals::Signal, use_signals};

fn main() {
    let a = Signal::new(2);
    let b = Signal::new("");
    
    let _ret = use_signals!([a, b] |a, b| {
        println!("{a}, {b}");
    });

    a.set(8);
    b.set("hola");
    a.set(9);
}