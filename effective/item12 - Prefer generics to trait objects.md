也就是更推荐：静态多态 而不是 动态多态

## Monomorphization

在 Rust 中，泛型的 monomorphization 是一个编译时过程，它将泛型代码转换成特定类型的具体代码实例。这个过程允许 Rust 程序在运行时保持高效，同时在编译时提供强类型检查和泛型的灵活性。我们来详细探讨一下这个概念。

Monomorphization 是一种编译技术，用于实现泛型。在其他一些语言中，比如 Java 或 C#，泛型是通过类型擦除来实现的：运行时会忽略类型参数，这意味着编译后的代码使用泛型的地方不包含有关这些类型的具体信息。相反，Rust 通过 monomorphization 来实现泛型，这意味着编译器会为每个使用的具体类型生成专门的代码。

### 工作原理

当你使用泛型函数或结构体时，Rust 编译器会观察所有使用这些泛型的地方，并为每个具体的类型参数生成特定的代码实例。这样，生成的机器码就不需要处理类型的泛化，因为每个实例都是针对特定类型优化的。

#### 示例

假设你有一个泛型函数：

```rust
fn print_value<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}
```

如果你在程序中调用了 `print_value` 函数，分别传递 `i32` 和 `f64` 类型的参数，Rust 编辑器将为这两种类型生成两个不同的 `print_value` 函数版本：

- 一个是 `print_value<i32>`，专门用于处理 `i32` 类型。
- 另一个是 `print_value<f64>`，专门用于处理 `f64` 类型。

### 优点与缺点

#### 优点

- **性能优化**：因为生成的代码是针对特定类型的，所以可以进行更多的优化，例如内联函数调用。
- **类型安全**：编译时的类型检查仍然有效，你可以保证类型的正确使用，而不会有运行时的类型错误。

#### 缺点

- **代码膨胀**：对于每个不同的类型，编译器都需要生成新的代码。如果泛型被广泛使用，这可能会导致编译后的二进制文件显著增大。

### 在实际应用中的权衡

在设计使用泛型的 Rust 程序时，你需要权衡使用泛型带来的灵活性和类型安全，以及可能增加的编译后代码量之间的关系。虽然 monomorphization 在性能上通常是有益的，但如果泛型使用过度，可能会对最终的程序大小产生不利影响。

总的来说，Rust 的泛型 monomorphization 提供了一种强大的机制，以空间换时间，通过为每个具体的类型生成专门的代码来提高运行时的效率。这种方法在系统编程中尤为重要，因为它既保证了高性能，又维护了强类型的安全性。

## 静态多态

```rust
trait Drawable {
    fn bounds(&self) -> Bounds;
}

struct Container<T>(T);

impl<T: Drawable> Container<T> {
	// The `area` method is available for all `Drawable` containers.
	fn area(&self) -> i64 {
		let bounds = self.0.bounds();
		(bounds.bottom_right.x - bounds.top_left.x)
			* (bounds.bottom_right.y - bounds.top_left.y)
	}
}

impl<T: Drawable + Debug> Container<T> {
	// The `show` method is only available if `Debug` is also implemented.
	fn show(&self) {
		println!("{:?} has bounds {:?}", self.0, self.0.bounds());
	}
}

fn main() {
    let square = Container(Square::new(1, 2, 2)); // Square is not Debug
    let circle = Container(Circle::new(3, 4, 1)); // Circle is Debug

    println!("area(square) = {}", square.area());
    println!("area(circle) = {}", circle.area());
    circle.show();
    // The following line would not compile.
    // square.show();
}
```
