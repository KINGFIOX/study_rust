// From 被包含在 `std::prelude` 中，因此我们没必要手动将其引入到当前作用域来
// use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    // 实现 `from` 方法
    fn from(value: i32) -> Self {
        Number { value }
    }
}

// 填空
fn main() {
    let num = Number::from(30);
    assert_eq!(num.value, 30);

    // let num: Number = Number { value: 30 };
    let num: Number = (30).into();
    assert_eq!(num.value, 30);

    println!("Success!")
}
