# ch10 - 宏

## 过程宏

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use hello_macro_derive::HelloMacro;
trait HelloMacro {
    fn hello_macro();
}
struct Sunfei;
impl HelloMacro for Sunfei {
    fn hello_macro() {
        {
            ::std::io::_print(format_args!("hello, macro! my name is {0}!\n", "Sunfei"));
        };
    }
}
struct Sunface;
impl HelloMacro for Sunface {
    fn hello_macro() {
        {
            ::std::io::_print(
                format_args!("hello, macro! my name is {0}!\n", "Sunface"),
            );
        };
    }
}
fn main() {
    Sunfei::hello_macro();
    Sunface::hello_macro();
    {
        ::std::io::_print(format_args!("Hello, world!\n"));
    };
}
```
