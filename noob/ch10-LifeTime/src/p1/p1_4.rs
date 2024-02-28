// `print_refs` 有两个引用参数，它们的生命周期 `'a` 和 `'b` 至少得跟函数活得一样久
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

/* 让下面的代码工作 */
// 'a 是外部的生命周期，这里 _x 并没有 'a 的生命周期长
fn failed_borrow<'a>() {
    let _x = 12;

    // ERROR: `_x` 活得不够久 does not live long enough
    let y: &i32 = &_x;

    // 在函数内使用 `'a` 将会报错，原因是 `&_x` 的生命周期显然比 `'a` 要小
    // 你不能将一个小的生命周期强转成大的
}

fn main() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);
    // 这里，four 和 nice 的生命周期必须要比函数 print_refs 长

    failed_borrow();
    // `failed_borrow`  没有传入任何引用去限制生命周期 `'a`，因此，此时的 `'a` 生命周期是没有任何限制的，它默认是 `'static`
}
