idiomatic 符合习惯的。
这里也就是说明：应当要青睐于：错误处理惯例

> In the previous example, the error types lined up: both the inner and outer methods expressed errors as [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html). That's often not the case; one function may accumulate errors from a variety of different sub-libraries, each of which uses different error types

就是如果向上抛出错误，那么 caller 如果再次抛出错误，那么 Err 的类型要包含有这个 callee 的 错误类型，那么就会需要 err map

- map Err

```rust
pub fn find_user(username: &str) -> Result<UserId, String> {
	let f = match std::fs::File::open("/etc/passwd") {
		Ok(f) => f,
		Err(e) => {
			return Err(format!("Failed to open password file: {:?}", e))
		}
	};
	// ...
}
```

下面是可以的 map

```rust
pub fn find_user(username: &str) -> Result<UserId, String> {
	let f = std::fs::File::open("/etc/passwd")
		.map_err(|e| format!("Failed to open password file: {:?}", e))?;
	// ...
}
```

![](Pasted%20image%2020240427220026.png)

Error 应该要实现的 trait

The first thing to notice is that the only hard requirement for `Error` types is the trait bounds: any type that implements `Error` also has to implement both:

- the `Display` trait, meaning that it can be `format!`ed with `{}`, and
- the `Debug` trait, meaning that it can be `format!`ed with `{:?}`.

## nested (嵌套) Error

可以实现 Error 中的 source() 来访问 错误链

```rust
use std::error::Error;
use std::fmt;

// 定义一个自定义错误类型
#[derive(Debug)]
struct MyError {
    message: String,
}

// std::error::Error 需要实现这个的
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MyError {
    // 不提供 source，因为这里它是链的底部
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// 生成一个错误，其中包含另一个错误作为源
fn get_error() -> Box<dyn Error> {
    let source_error = MyError {
        message: "the root cause".to_string(),
    };
    let main_error = MyError {
        message: "high-level operation failed".to_string(),
    };

    // 使用错误盒子来模拟复杂的错误场景
    Box::new(main_error)
}

fn main() {
    // 模拟错误处理
    let error = get_error();

    println!("Error encountered:");
    print_error_chain(&error);
}

// 打印错误链的函数
fn print_error_chain(err: &dyn Error) {
    println!("{}", err);
    let mut source = err.source();
	//
    while let Some(cause) = source
    {
        println!("Caused by: {}", cause);
        source = cause.source();
    }
}
```

## 特征对象 与 error

这样就不用写一个 enum ，然后再写很多个 From 了，
但是这也有一个问题，就是 特征对象会忘记自己的 子类型

## 库程序员 与 应用程序程序员 不同的错误处理视角

> The final advice of the previous section included the qualification "…for error handling *in applications*". That's because there's often a distinction between code that's written for re-use in a library, and code that forms a top-level application[3](https://www.lurklurk.org/effective-rust/errors.html#footnote-3).

> Code that's written for a library can't predict the environment in which the code is used, so it's preferable to emit concrete, detailed error information, and leave the caller to figure out how to use that information. This leans towards the `enum`-style nested errors described previously (and also avoids a dependency on `anyhow` in the public API of the library, cf. [Item 24](https://www.lurklurk.org/effective-rust/re-export.html)).

anyhow 好像挺好用的

> However, application code typically needs to concentrate more on how to present errors to the user. It also potentially has to cope with all of the different error types emitted by all of the libraries that are present in its dependency graph ([Item 25](https://www.lurklurk.org/effective-rust/dep-graph.html)). As such, a more dynamic error type (such as [`anyhow::Error`](https://docs.rs/anyhow/latest/anyhow/struct.Error.html)) makes error handling simpler and more consistent across the application.

## anyhow

`anyhow` 是 Rust 社区中广泛使用的一个错误处理库，它提供了一个方便的方式来处理那些不需要复杂错误类型或不需要错误枚举的错误。这个 crate 特别适合用于应用程序和原型开发，其中错误处理通常更灵活，且不需要严格定义错误的各种可能类型。

### 特点

1. **简化的错误类型**：`anyhow::Error` 类型是一个动态的、兼容 `std::error::Error` 的错误类型。它可以包装几乎任何类型的错误，只要这些错误类型实现了 `std::error::Error`。

2. **易于使用**：`anyhow` 允许你轻松地创建错误，仅通过提供一个错误消息或使用 `?` 操作符来自动从大多数标准的错误类型转换。

3. **链式错误**：`anyhow` 支持自动的错误链功能，无需显式地调用错误的 `source` 方法。当你打印 `anyhow::Error` 时，它会展示整个错误链，帮助你追踪错误原因。

4. **无需定义错误枚举**：在很多错误处理场景中，定义一个详细的错误枚举可能是过度设计。`anyhow` 提供了一种更为简洁的方法来处理错误，而不需要预定义所有可能的错误类型。

### 使用示例

这里是一个使用 `anyhow` 的简单示例，展示了如何在函数中使用它来处理错误：

```rust
use anyhow::{Result, anyhow};

fn may_fail(flag: bool) -> Result<()> {
    if flag {
        Ok(())
    } else {
        Err(anyhow!("Something went wrong"))
    }
}

fn main() -> Result<()> {
    may_fail(true)?;
    may_fail(false)?; // 这里会因为错误而提前退出
    Ok(())
}
```

在这个例子中：

- `may_fail` 函数使用 `anyhow!` 宏来创建一个错误，当函数的参数 `flag` 为 `false` 时触发。
- 主函数 `main` 使用 `?` 操作符尝试调用 `may_fail`。如果 `may_fail` 返回错误，`main` 也将返回这个错误。

### 集成与调试

`anyhow` 非常适合用在顶层应用逻辑中，它与 Rust 的 `std::error::Error` 特性兼容，可以与其他使用标准错误特性的库无缝集成。此外，当使用如 `log` 或 `env_logger` 等日志库时，`anyhow` 提供的错误信息尤其详尽，有助于快速定位问题。

综上，`anyhow` 提供了一种非常灵活且强大的方式来处理错误，特别是在那些不需要详细区分错误类型的场景下。通过简化错误处理流程，它让你可以更专注于实际的应用逻辑开发。
