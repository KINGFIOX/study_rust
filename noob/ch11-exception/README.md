# README - 错误处理

## and then 与 map

你没有见过的传新语法

```rust
use std::num::ParseIntError;

// 使用两种方式填空: map, and then
// fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
//     // map 返回的是 Result
//     n_str.parse::<i32>().map(|x| x + 2)
// }

fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
    // map 返回的是 Result
    n_str.parse::<i32>().and_then(|x| Ok(x + 2))
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!")
}
```

## 二级 lambda

```rust
// 重写上面的 `multiply` ，让它尽量简洁
// 提示：使用 `and_then` 和 `map`
fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    n1_str.parse::<i32>().and_then(|x| { n2_str.parse::<i32>().and_then(|y| Ok(x * y)) })
}
```

## type 类型别名

```rust
type Res<T> = Result<T, ParseIntError>;
```
