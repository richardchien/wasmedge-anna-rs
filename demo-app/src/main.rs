use wasmedge_anna;

fn main() {
    println!("hello!");
    wasmedge_anna::put("foo", "bar".as_bytes());
    let val = wasmedge_anna::get("foo");
    println!("{:?}", val);
    let val = wasmedge_anna::get("bar");
    println!("{:?}", val);
}
