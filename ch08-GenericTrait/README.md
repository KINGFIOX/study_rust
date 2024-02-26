# ch08 - 泛型与特征

## debug for <一个类型>

在 Rust 中，为 `ArrayPair` 实现 `Debug` trait 的意思是提供一种方式来格式化 `ArrayPair` 实例的信息为一个字符串，这通常用于调试目的。
`Debug` trait 是 Rust 标准库中的一个特质（trait），它定义在 `std::fmt` 模块下。
当你为一个类型实现了 `Debug` trait，你就能够使用 `{:?}` 或 `{:#?}` 格式化占位符通过 `println!` 宏或其他格式化宏打印其实例，从而查看其内部状态。

实现 `Debug` trait 对于泛型结构体尤其重要，因为它允许开发者在开发过程中轻松地检查和调试泛型结构体实例的状态，
无论其具体的类型参数 `T` 和数组长度 `N` 是什么。

下面是如何为 `ArrayPair` 结构体实现 `Debug` 的一个简单例子：

```rust
use std::fmt::{self, Debug, Formatter};

struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}

impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArrayPair")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}
```

在这个例子中：

- `fmt` 方法是 `Debug` trait 的一部分，需要被实现。它定义了如何将 `ArrayPair` 实例格式化为一个字符串。
- `f.debug_struct("ArrayPair")` 开始一个新的结构体格式化过程，并指定结构体的名称。
- `.field("left", &self.left)` 和 `.field("right", &self.right)` 分别为 `left` 和 `right` 字段添加格式化条目，这意味着它们也会被格式化为字符串，前提是它们的类型实现了 `Debug`。
- `finish()` 完成格式化过程并返回最终的格式化结果。

通过这种方式，如果你有一个 `ArrayPair<i32, 2>` 的实例，
你可以使用 `println!("{:?}", instance);` 来打印如下格式的字符串（具体输出取决于 `left` 和 `right` 的值）：

```plaintext
ArrayPair { left: [1, 2], right: [3, 4] }
```

这为开发者提供了一种方便的方法来检查 `ArrayPair` 实例的当前状态。

## 切片大小

```rust
let array = ["hello你好"; 5]; // 这里的 5 是数组的长度
// 切成了 5 个切片，每个切片维护两个数据：指针 + 数组大小
// 然后我又是 64 位平台，因此这个 array 的长度是 16 x 5 = 80
```

## let 与 借用 与 所有权转移

在 Rust 中，`let`、`let &`、`let &mut` 和 `let mut` 的含义确实与变量的所有权和借用有关，但它们的具体含义如下：

### `let`

`let` 用于声明一个新的变量。默认情况下，变量是不可变的。
这并不意味着它总是“转移所有权”，而是根据使用的上下文而定。
如果你用 `let` 绑定一个值到一个新变量，它会根据该值的类型决定是移动所有权还是进行拷贝。

- 对于实现了 `Copy` trait 的类型（如整数、浮点数、字符和布尔类型以及其他一些复合类型），使用 `let` 会拷贝值。
- 对于没有实现 `Copy` trait 的类型（如大多数结构体和枚举），使用 `let` 会转移所有权。

### `let &`

`let &` 并不是 Rust 语法的一部分。如果你看到类似 `let &var = &expr;` 的代码，这其实是模式匹配的一种用法，用于解构引用，而不是创建它。
通常，你会使用 `let` 加上一个模式来解构值，而不是用来声明引用。

### `let &mut`

和 `let &` 一样，`let &mut` 也不直接用于声明变量的借用。
如果你看到 `let &mut var = &mut expr;`，这同样是一种模式匹配的用法，用于解构可变引用。

### `let mut`

`let mut` 用于声明一个可变变量。这意味着变量的值可以被改变。使用 `let mut` 并不直接意味着“拷贝”。
它只是指这个变量是可变的。是否发生拷贝取决于赋给该变量的值的类型，类似于 `let` 的行为：

- 如果值类型实现了 `Copy` trait，那么会发生拷贝。
- 如果没有实现 `Copy` trait，那么会转移所有权。

### 借用

- 不可变借用：通过 `&expr` 创建，可以通过 `let` 绑定到一个变量上（`let ref_var = &expr;`），使得 `ref_var` 成为一个不可变引用。
- 可变借用：通过 `&mut expr` 创建，可以通过 `let mut` 绑定到一个变量上（`let mut ref_var = &mut expr;`），使得 `ref_var` 成为一个可变引用。

总之，`let` 用于变量绑定，其行为（拷贝还是转移所有权）取决于值的类型。
`let mut` 声明一个可变变量。
直接使用 `&` 和 `&mut` 与 `let` 绑定结合时，通常涉及到借用（不可变或可变），而不是所有权的转移或拷贝。

## 对 Copy 的 Trait 转移所有权

在 Rust 中，如果一个类型实现了 `Copy` trait，意味着该类型的值可以安全地被拷贝，并且在赋值或函数传参时，默认行为是进行拷贝而不是转移所有权。
对于实现了 `Copy` trait 的类型，Rust 不允许显式地进行所有权转移，因为这违背了 `Copy` trait 的设计初衷。
`Copy` trait 的存在就是为了表明这种类型的值可以被无缝地复制，而不需要担心所有权的问题。

如果你确实需要在逻辑上表达所有权的转移，即使对于实现了 `Copy` trait 的类型，
你可以通过一些设计上的方法来达到目的，尽管这些方法更多地是逻辑上的区分，而非强制性的所有权转移。以下是一些可能的方法：

### 1. 使用非 `Copy` 类型

如果你希望一个值的传递明确地表达所有权转移，你可以考虑将该值封装在一个不实现 `Copy` trait 的类型中。
例如，你可以定义一个新的结构体，该结构体不实现 `Copy` trait：

```rust
struct NonCopyable<T>(T);

impl<T> NonCopyable<T> {
    fn new(value: T) -> Self {
        NonCopyable(value)
    }
}
```

这样，即使 `T` 是一个实现了 `Copy` trait 的类型，`NonCopyable<T>` 的实例在赋值或传递时将遵循所有权的规则，即发生所有权的转移。

### 2. 显式消费

对于实现了 `Copy` trait 的类型，你可以通过显式地“消费”变量来逻辑上表达所有权的转移，
比如通过将其传递给一个消费该变量的函数：

```rust
fn consume<T>(_value: T) {
    // 在这里，_value 被消费。
}

let x = 5; // 假设 x 的类型实现了 Copy
consume(x); // 逻辑上表达了所有权的转移，尽管实际上 x 被拷贝了
// 从逻辑上讲，x 不应再被使用（尽管在代码中它仍然有效）
```

### 3. 使用文档和约定

在一些情况下，明确文档化函数或代码块的预期用法，指出某个参数在逻辑上是被“转移所有权”到函数内部，是一种简单但有效的方法。
虽然这种方法依赖于开发者遵守约定，但它在实践中可以是处理轻量级 `Copy` 类型的有用策略。

### 结论

虽然 Rust 不允许对实现了 `Copy` trait 的类型进行显式的所有权转移操作，但你可以通过上述方法在逻辑层面上模拟所有权的转移。
这些方法可以帮助你在保持类型安全性的同时表达程序的设计意图。
