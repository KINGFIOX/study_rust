## ?Sized

在 Rust 中，`?Sized` 是一个特殊的 trait bound，用于指示某个类型参数可能不是固定大小的。默认情况下，Rust 中的所有泛型类型参数都必须是已知大小的，即它们必须实现 `Sized` trait。`Sized` trait 是 Rust 的一种内置 trait，表示类型的大小在编译时是已知的。

### 为什么需要 `?Sized`
有些类型，如切片类型（例如 `&[T]`）和特征对象（例如 `dyn Trait`），在编译时大小是未知的。这些类型被称为“动态大小类型”（Dynamically Sized Types，DST）。要在泛型中使用这些类型，你必须显式地使用 `?Sized` bound 来表明该泛型参数可以是非固定大小的。

### 使用 `?Sized` 的示例
假设你要写一个函数，这个函数接收一个可能是动态大小的类型的引用。你可以这样定义：

```rust
fn print_info<T: ?Sized>(value: &T) {
    println!("The value is {:?}", value);
}
```

这里，`T: ?Sized` 表明 `T` 可以是任何类型，包括那些不具备固定大小的类型。如果你省略了 `?Sized`，Rust 编译器将默认 `T` 必须实现 `Sized` trait。

### 使用场景
通常，你会在需要处理泛型和动态大小类型的交互时使用 `?Sized`。例如，在设计一些工具 trait 或者基于 trait 的抽象时，这个特性非常有用。另一个常见的使用场景是当你在编写类似于标准库中的 `Box<T>` 或者 `&T` 这样的类型时，这些类型需要能够处理动态大小类型。

### 具体示例

```rust
trait Print {
    fn print(&self);
}

impl Print for str {
    fn print(&self) {
        println!("{}", self);
    }
}

fn print_it<T: Print + ?Sized>(t: &T) {
    t.print();
}

fn main() {
    let my_str = "Hello, world!";
    print_it(my_str);
}
```

在上面的例子中，`print_it` 函数接受一个实现了 `Print` trait 的任何类型的引用，包括动态大小的类型。`str` 是一个动态大小类型，我们可以通过一个具体的字符串切片引用来调用 `print_it` 函数。

通过使用 `?Sized`，你可以编写更灵活的代码，这些代码可以操作多种类型，包括那些在编译时大小未知的类型。这是 Rust 强大的类型系统和安全保证的又一表现。