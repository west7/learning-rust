fn main() {
    unsafe {
        let p: *const i32 = std::ptr::null();
        println!("{}", *p); // Desreferenciamento de ponteiro nulo
    }
}
