fn a() {
    // let _guard = eulogy::record_value("answer", 42);
    eulogy::record!("outer", 19);
    for i in 0.. {
        eulogy::record!("i", i);
        if i == 4 {
            panic!("That's enough!");
        }
    }
}

fn main() {
    eulogy::install_panic_hook();
    a();
    println!("return from a");
}
