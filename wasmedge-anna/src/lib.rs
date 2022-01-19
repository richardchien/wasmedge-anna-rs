extern "C" {
    fn __wasmedge_anna_add(a: i32, b: i32) -> i32;
}

pub fn add(a: i32, b: i32) -> i32 {
    unsafe { __wasmedge_anna_add(a, b) }
}

#[no_mangle]
fn add2(a: i32) -> i32 {
    unsafe { __wasmedge_anna_add(a, 2) }
}
