use std::cell::RefCell;
use std::cell::Cell;
use std::fmt::Debug;

struct MyStruct {
    field1: i32,
    field2: String,
}

impl Debug for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {}
}

fn main() {
    let c = Cell::new("asdf");

    let one = c.get();

    c.set("qwer");

    let two = c.get();

    println!("{}, {}", one, two)
}
