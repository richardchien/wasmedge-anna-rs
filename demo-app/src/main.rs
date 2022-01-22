use wasmedge_anna;

fn main() {
    println!("hello!");
    println!("add result: {}", wasmedge_anna::add(1, 2));
    wasmedge_anna::put("foo", "bar".as_bytes());
    let val = wasmedge_anna::get("foo");
    println!("{:?}", val);
    let val = wasmedge_anna::get("bar");
    println!("{:?}", val);
}
