fn a() {
    let _guard = eulogy::record_value("answer", 42);
    println!("ready to panic...");
    panic!("This is a panic!");
}

fn main() {
    eulogy::install_panic_hook();
    a();
    println!("return from a");
}
