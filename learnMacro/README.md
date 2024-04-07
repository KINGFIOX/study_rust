# 宏

## 卫生性

```rust
macro_rules! using_a {
    ($e:expr) => {
        {
            let a = 42;
            $e
        }
    }
}

fn main() {
    let a = 1000;
    let four = using_a!(a / 10);
}
```

上面这段展开就是：

```rust
fn main() {
    let a = 1000;
    let four = {
        let a = 42;
        a / 42 // 这里输出是 21
    };
}
```

但是如果我们换成 b ，就是这个效果

```rust
fn main() {
    let b = 1000;
    let four = using_a!(b / 10);
}

fn main() {
    let b = 1000;
    let four = {
        let a = 42;
        b / 10  // 100
    }
}
```

但是这里很操蛋的一点，一方面会捕捉上下文，另一方面会遮盖上下文
