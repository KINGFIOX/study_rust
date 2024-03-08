use hello_macro_derive::{ HelloMacro, MyDefault };

trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Sunfei;

#[derive(HelloMacro)]
struct Sunface;

fn main() {
    Sunfei::hello_macro();
    Sunface::hello_macro();
    println!("Hello, world!");
}

pub trait MyDefault: Sized {
    fn default() -> Self;
}

#[derive(MyDefault)]
struct SomeData(u32, String);
