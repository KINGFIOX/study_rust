use std::ops::Mul;

// By the fundamental theorem of arithmetic, rational numbers in lowest
// terms are unique. So, by keeping `Rational`s in reduced form, we can
// derive `Eq` and `PartialEq`.

// 实现 fn multiply 方法
// 如上所述，`+` 需要 `T` 类型实现 `std::ops::Add` 特征
// 那么, `*` 运算符需要实现什么特征呢? 你可以在这里找到答案: https://doc.rust-lang.org/core/ops/
fn multiply<T: std::ops::Mul<Output = T>>(x: T, y: T) -> T {
    x * y
}

fn main() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("Success!")
}
