fn f<'a, T>(x: *const T) -> &'a T {
    unsafe { &*x }
}
