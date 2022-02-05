# wasmedge-anna-rs

**wasmedge-anna-rs** is a Rust SDK for [wasmedge-anna](https://github.com/second-state/wasmedge-anna).

## Usage

```toml
# Cargo.toml

[package]
name = "app-name"
version = "0.1.0"
edition = "2021"

[dependencies]
wasmedge-anna = { path = "/path/to/wasmedge-anna" }
```

```rust
// src/main.rs

use wasmedge_anna;

fn main() {
    let ok: bool = wasmedge_anna::put("foo", "bar".as_bytes());
    assert!(ok);
    let val: Option<Vec<u8>> = wasmedge_anna::get("foo");
    assert_eq!(val, Some(b"bar".to_vec()));
}
```

```shell
cargo build --target wasm32-wasi
/path/to/wasmedge_anna /path/to/anna/conf/anna-local.yml target/wasm32-wasi/debug/app-name.wasm
```

## Examples

- [hello](examples/hello)
- [js-demo](examples/js-demo)
