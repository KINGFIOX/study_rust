fn foo() -> i32 {
    0
}

fn main() {
    let pointer = foo as *const ();
    let function = unsafe { std::mem::transmute::<*const (), fn() -> i32>(pointer) };
    assert_eq!(function(), 0);
}
