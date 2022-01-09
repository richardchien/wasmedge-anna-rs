extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

#[no_mangle]
fn add2(a: i32) -> i32 {
    unsafe {
        add(a, 2)
    }
}
