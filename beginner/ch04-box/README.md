# ch04 - 多线程

## 内存顺序

然而，使用 `AtomicBool` 的 `store` 方法与 `Ordering::Release` 语义，
以及 `load` 方法与 `Ordering::Acquire` 语义，确保了一种 happens-before 关系。
当 `consumer` 线程通过 `Ordering::Acquire` 观察到 `READY` 变为 `true` 时，
它也保证能看到 `producer` 线程中发生的所有先前的写入操作（在本例中，即 `DATA = 100`）。
这就意味着，不管线程的启动顺序如何，`consumer` 线程总是能安全地读取 `DATA` 的值，并且该值是 `producer` 线程写入的值。

简而言之，虽然线程的启动顺序在代码中是确定的，
但 `producer` 线程对 `DATA` 的写入和对 `READY` 的 `store` 操作，
以及 `consumer` 线程的 `load` 操作之间的顺序保证，
是通过 `Ordering::Release` 和 `Ordering::Acquire` 内存顺序来实现的，
而不是依赖于线程的启动顺序。这样确保了数据的一致性和内存的可见性，避免了数据竞争和内存顺序错误。

## 关键字 ref

在 Rust 中，`ref` 关键字用于在模式匹配（比如 `match` 或 `if let`）或解构（如变量绑定）时，
获取对匹配值的引用，而不是取得其所有权。这使得你可以在不获取值所有权的情况下，仍然能够访问或修改这个值。

当你匹配或解构一个结构体、枚举或者其他复合类型的时候，使用 `ref` 可以创建一个对这个值的不可变引用。
如果你需要一个可变引用，则可以使用 `ref mut`。

### 示例

以下是 `ref` 和 `ref mut` 的一些使用例子：

- 使用 `ref` 获取不可变引用：

```rust
let tuple = (String::from("hello"), String::from("world"));
let (ref s1, ref s2) = tuple;
// `s1` 和 `s2` 都是对 `tuple` 中字符串的不可变引用
```

- 使用 `ref mut` 获取可变引用：

```rust
let mut tuple = (String::from("hello"), String::from("world"));
let (ref mut s1, _) = tuple;
// `s1` 是对 `tuple` 第一个元素的可变引用
s1.push_str(", Rust");
```

### 为什么需要 `ref` 和 `ref mut`

在 Rust 中，默认情况下，模式匹配会移动值。对于实现了 `Copy` 特征的类型来说，这意味着值会被复制。
但对于没有实现 `Copy` 的类型（如 `String`），这意味着值的所有权会被移动，导致原变量不能再被使用。
使用 `ref` 关键字，你可以避免这种所有权的移动，仅创建一个值的引用。

### 总结

总的来说，`ref` 和 `ref mut` 关键字在 Rust 中用于在模式匹配和解构时创建对值的引用，
这样你就可以在不获取值的所有权的情况下操作这些值。这在处理不需要或不想获取所有权的复杂数据结构时非常有用。
