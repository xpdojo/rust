use lang;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
fn main() {
    // cargo run
    let add1: i32 = lang::add(1, 2);
    println!("Hello, world! {}", &add1.to_string());
}
