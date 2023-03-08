use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

fn main() {
    PanCake::hello_macro();
    println!("Hello, world!");
}

#[derive(HelloMacro)]
struct PanCake;
