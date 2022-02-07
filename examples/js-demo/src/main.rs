use wasmedge_quickjs::Context;

mod js_anna_api {
    use wasmedge_quickjs::*;

    struct PutFn;
    impl JsFn for PutFn {
        // Put: (key: String | Uint8Array, value: String | Uint8Array) -> bool
        fn call(_ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
            if argv.len() != 2 {
                return JsValue::Bool(false);
            }
            let key: Vec<u8>;
            let val: Vec<u8>;
            match &argv[0] {
                JsValue::String(s) => key = s.to_string().bytes().collect(),
                JsValue::ArrayBuffer(u8a) => key = u8a.to_vec(),
                _ => return JsValue::Bool(false),
            }
            match &argv[1] {
                JsValue::String(s) => val = s.to_string().bytes().collect(),
                JsValue::ArrayBuffer(u8a) => val = u8a.to_vec(),
                _ => return JsValue::Bool(false),
            }
            return JsValue::Bool(wasmedge_anna::put(key, val));
        }
    }

    struct GetFn;
    impl JsFn for GetFn {
        // Get: (key: String | Uint8Array) -> Uint8Array | null
        fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
            if argv.len() != 1 {
                return JsValue::Null;
            }
            let key: Vec<u8>;
            match &argv[0] {
                JsValue::String(s) => key = s.to_string().bytes().collect(),
                JsValue::ArrayBuffer(u8a) => key = u8a.to_vec(),
                _ => return JsValue::Bool(false),
            }
            let val = wasmedge_anna::get(key);
            val.map_or_else(
                || JsValue::Null,
                |v| JsValue::ArrayBuffer(ctx.new_array_buffer(&v)),
            )
        }
    }

    struct Module;
    impl ModuleInit for Module {
        fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
            m.add_export("put\0", ctx.new_function::<PutFn>("put").into());
            m.add_export("get\0", ctx.new_function::<GetFn>("get").into());
        }
    }

    pub fn init(ctx: &mut Context) {
        ctx.register_module("wasmedge_anna\0", Module, &["put\0", "get\0"]);
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!(
            "Usage: /path/to/wasmedge_anna <anna-config> {} <js-file>",
            args[0]
        );
        return;
    }

    let js_file = &args[1];

    let mut ctx = Context::new();
    js_anna_api::init(&mut ctx);

    let code = std::fs::read_to_string(js_file).expect("Failed to read JavaScript code");
    ctx.eval_module_str(&code, js_file);
    ctx.promise_loop_poll();
}
