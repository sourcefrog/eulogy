fn main() {
    let _guard = eulogy::record_value("main", 42, file!());

    eulogy::print_values();
}
