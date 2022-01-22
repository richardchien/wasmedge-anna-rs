use wasmedge_anna;

fn main() {
    println!("hello!");
    println!("add result: {}", wasmedge_anna::add(1, 2));
    wasmedge_anna::put("foo", "bar".as_bytes());
}
