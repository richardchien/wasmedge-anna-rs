extern "C" {
    fn __wasmedge_anna_add(a: i32, b: i32) -> i32;
    fn __wasmedge_anna_put(
        key_size: usize,
        key_ptr: *const u8,
        val_size: usize,
        val_ptr: *const u8,
    ) -> bool;
    fn __wasmedge_anna_get_stage_1(key_size: usize, key_ptr: *const u8) -> usize;
    fn __wasmedge_anna_get_stage_2(key_size: usize, key_ptr: *const u8, val_ptr: *mut u8) -> bool;
}

pub fn add(a: i32, b: i32) -> i32 {
    unsafe { __wasmedge_anna_add(a, b) }
}

pub fn put(key: impl AsRef<str>, value: impl AsRef<[u8]>) -> bool {
    let key = key.as_ref();
    let val = value.as_ref();
    unsafe { __wasmedge_anna_put(key.len(), key.as_ptr(), val.len(), val.as_ptr()) }
}

pub fn get(key: impl AsRef<str>) -> Option<Vec<u8>> {
    let key = key.as_ref();
    let val_size = unsafe { __wasmedge_anna_get_stage_1(key.len(), key.as_ptr()) };
    if val_size == 0 {
        return None;
    }
    let mut val = vec![0; val_size];
    let ok = unsafe { __wasmedge_anna_get_stage_2(key.len(), key.as_ptr(), val.as_mut_ptr()) };
    if ok {
        Some(val)
    } else {
        None
    }
}

#[no_mangle]
fn add2(a: i32) -> i32 {
    unsafe { __wasmedge_anna_add(a, 2) }
}
