mod host {
    #[link(wasm_import_module = "wasmedge_anna")]
    extern "C" {
        pub fn put(
            key_size: usize,
            key_ptr: *const u8,
            val_size: usize,
            val_ptr: *const u8,
        ) -> bool;
        pub fn get(
            key_size: usize,
            key_ptr: *const u8,
            val_buf_size: usize,
            val_buf_ptr: *mut u8,
        ) -> usize;
    }
}

pub fn put(key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> bool {
    let key = key.as_ref();
    let val = value.as_ref();
    unsafe { host::put(key.len(), key.as_ptr(), val.len(), val.as_ptr()) }
}

pub fn get(key: impl AsRef<[u8]>) -> Option<Vec<u8>> {
    let key = key.as_ref();
    let mut val_size;
    let mut val_buf = Vec::new();

    loop {
        val_size =
            unsafe { host::get(key.len(), key.as_ptr(), val_buf.len(), val_buf.as_mut_ptr()) };
        if val_size == 0 {
            return None;
        }
        if val_size > val_buf.len() {
            // buffer is too small
            val_buf.resize(val_size, Default::default());
        } else {
            break;
        }
        // loop in case the value is changed during this little period
    }

    val_buf.resize(val_size, Default::default());
    Some(val_buf)
}
