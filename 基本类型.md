### 未使用的变量

1.

```rust
#[allow(unused_variables)]
```

2. 加一个下划线

```rust
let _x = 0;
```

### 如果没有显示设置返回值

返回`()`

### 元组拆包 (解构)

### 一些宏 println! assert_eq!

### statement expression

```rust
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 下面表达式的值将被赋给 `y`
        x_cube + x_squared + x
    };

    let z = {
        // 分号让表达式变成了语句，因此返回的不再是表达式 `2 * x` 的值，而是语句的值 `()`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
```

返回值是：

```rust
x is 5
y is 155
z is ()
```

### 复数 使用库

在 Cargo.toml 中的 `[dependencies]` 下添加一行 num = "0.4.0"

然后

```rust
use num::complex::Complex;

 fn main() {
   let a = Complex { re: 2.1, im: -1.2 };
   let b = Complex::new(11.1, 22.2);
   let result = a + b;

   println!("{} + {}i", result.re, result.im)
 }
```

### NaN

一些奇奇怪怪的值，比方说`(-42.0_f32).sqrt()`

### 自动推导

```rust
fn main() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;

    let z = 10; // 这里 z 的类型是?
}
```

这里的 y 是自动推导了 x 的类型

### as 类型转换

```rust
// 填空
fn main() {
    let v: u16 = 38_u8 as u16;
}
```

### 溢出 检查

```rust
// 解决代码中的错误和 `panic`
fn main() {
   let v1 = 247_u8 + 8;
   let v2 = i8::checked_add(119, 8).unwrap();
   println!("{},{}",v1,v2);
}
```

`let v2 = i8::checked_add(119, 8).unwrap();` 这段代码的意思是将 119 和 8 相加，并将结果赋值给变量 v2。`checked_add` 是一个方法，用于检查加法操作是否会导致溢出。如果没有溢出，它会返回 `Some(result)`，否则返回 `None`。在这里，我们使用 `unwrap()` 方法来获取实际的加法结果。由于 i8 类型的范围是 -128 到 127，所以这段代码应该不会导致溢出。

### 浮点数判断相等

1.

```rust
fn main() {
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
}
```

2. 小于 EPS

```rust
fn main() {
    assert!((0.1_f64+ 0.2 - 0.3).abs() < 0.001);
}
```

### 序列

```rust
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
}
```

### 计算

```rust

// 填空，并解决错误
fn main() {
    // 整数加法
    assert!(1u32 + 2 == 3u32);

    // 整数减法
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9.6 / 3.2 == 3.0_f32); // error ! 修改它让代码工作

    assert!(24 % 5 == 4);

    // 逻辑与或非操作
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // 位操作
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
```

### 获取 对象 的大小

```rust
`use std::mem::size_of_val;` 这段代码的意思是在 Rust 代码中导入 `std::mem` 模块中的 `size_of_val` 函数。`size_of_val` 函数用于返回给定值的大小（以字节为单位），可以用于检查变量或类型的大小。例如，可以使用 `size_of_val(&my_variable)` 来获取变量 `my_variable` 的大小。
```

### 函数的返回值与参数

```rust
fn main() {
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);
}

// 要设置 返回值，参数
fn sum(x:i32 , y: i32) -> i32 {
    x + y
}
```

### 发散函数

`panic!` 是 Rust 中的一个宏，用于在程序出现无法处理的错误时终止程序的执行。
当程序执行到 `panic!` 宏时，它会立即停止并打印一条错误消息。
这可以帮助开发人员快速发现问题并进行调试。

在 Rust 中，`panic!` 宏通常用于表示发生了无法恢复的错误，
例如数组越界、除以零等。
当出现这些错误时，程序无法继续执行，
因此使用 `panic!` 宏来终止程序的执行是一种合理的做法。

当然，在实际开发中，我们应该尽可能地避免使用 `panic!` 宏，
而是使用 Rust 提供的更安全的错误处理机制，
例如 `Result` 类型和 `unwrap_or_else` 方法。这样可以使代码更加健壮和可维护。

1. 错误

```rust
fn main() {
    never_return();
}

fn never_return() -> ! {
    // implement this function, don't modify fn signatures
    panic!("I return nothing!")
}
```

2. 死循环

```rust
fn main() {
    never_return();
}

use std::thread;
use std::time;

fn never_return() -> ! {
    // implement this function, don't modify fn signatures
    loop {
        println!("I return nothing");
        // sleeping for 1 second to avoid exhausting the cpu resource
        thread::sleep(time::Duration::from_secs(1))
    }
}
```

3. unimplemented 未实现的

```rust
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    never_return_fn()
}

// IMPLEMENT this function
// DON'T change any code else
fn never_return_fn() -> ! {
    unimplemented!()
}
```

4. todo

```rust
// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    todo!();
}
```

5. 时间

```rust
// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}
```

`std::thread::sleep(std::time::Duration::from_secs(1))` 是 Rust 中用于使当前线程休眠指定时间的代码。其中 `std::thread::sleep` 是 Rust 标准库中的一个函数，用于使当前线程休眠指定的时间。`std::time::Duration::from_secs(1)` 是一个 `std::time::Duration` 类型的值，表示一秒钟的时间长度。将这个值作为参数传递给 `std::thread::sleep` 函数，就可以使当前线程休眠一秒钟。

在 Rust 中，线程是轻量级的执行单元，可以同时运行多个线程来执行不同的任务。通过使线程休眠，我们可以控制线程的执行时间，从而实现更加灵活的任务调度和资源管理。

请注意，在使用 `std::thread::sleep` 函数时需要注意线程安全性和性能问题。如果在多线程环境中使用 `std::thread::sleep` 函数，需要确保程序的正确性和可靠性。此外，频繁地使用 `std::thread::sleep` 函数可能会影响程序的性能，因此需要谨慎使用。
