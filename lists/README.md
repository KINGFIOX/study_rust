# README - 链表

## as_ref 的使用

在 Rust 中，`Option` 类型的 `map` 方法期望传入的函数能够作用于存储在 `Some` 中的值。
但是，当你直接在 `Option` 上调用 `map` 方法时，
这要求 `Option` 必须拥有值的所有权或者值必须是 `Copy` 类型的，
因为 `map` 方法会尝试消费（或移动）`Option` 中的值。

然而，当你的目标是在不取得值所有权的情况下仅访问 `Option` 中的值，就需要使用 `as_ref` 方法。
`as_ref` 将 `Option<T>` 转换为 `Option<&T>`，即从一个拥有值的 `Option` 到一个拥有值引用的 `Option`。
这样做的结果是你可以在不获取所有权的情况下，仍然对 `Option` 中的值进行操作。

在你的代码片段中：

```rust
pub fn peek(&self) -> Option<&T> {
    // self.head.map(|node| { &node.elem })
    self.head.as_ref().map(|node| { &node.elem })
}
```

- 直接使用 `self.head.map(|node| { &node.elem })` 会因为所有权问题而失败，
  除非 `self.head` 的类型是 `Copy`，这在许多情况下并不成立（特别是对于复杂的或自定义的类型）。
- 通过使用 `as_ref`，`self.head` 被转换为一个对原始数据的引用的 `Option`（即 `Option<&Node>`），
  这样 `map` 方法就可以在不取得所有权的情况下应用到这个引用上。
  这意味着我们可以安全地访问 `node.elem` 的引用，并将其作为 `Option<&T>` 返回。

简而言之，`as_ref` 的使用允许你在保持 `Option` 不变的前提下，对其中的值进行只读访问。
这对于实现如 `peek` 这样的方法特别有用，因为你想要返回存储在 `Option` 中的值的引用，而不是值本身，同时不改变原始 `Option`。

我的评价是：因为这里的 self 是引用类型，然后`map(|node| &node.elem)`这里实际上有 move，
然而我们这里的`peek(&self)`并没有所有权（并且我们也不希望将所有权转移），不加所有权，我们就要用`Option<&T>`，
而不是`Option<T>`

## 再一次体会 as_ref

```rust
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // self.next = node.next.map(|node| &node);
            // self.next = node.next.map(|node| &*node);
            // self.next = node.next.as_ref().map(|node| &*node);  // 错误，因为类型不匹配，引用
            // self.next = node.next.as_ref().map(|node| &**node);  // as_ref
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}
```

- 第一次出现错误：

```rust
self.next = node.next.map(|node| &node);
|           ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `second::Node`, found struct `std::boxed::Box`
```

这里期待的是：`Node`而不是`boxed::Box`（因为接受者是 self.next）

改错，解引用

- 第二次出现错误

```rust
Iter { next: self.head.map(|node| &*node) }
|                                 ^^^^^^ returns a reference to data owned by the current function
```

self.head.map，这个时候，map 会取得 self.head 的所有权，并传入 闭包中。
闭包返回的是引用，但是闭包依然持有所有权。当闭包结束以后，就出现了悬垂引用。

那么就可以使用`as_ref`改正

- 第三次出现错误

```rust
Iter { next: self.head.as_ref().map(|node| &*node) }
|            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `second::Node`, found struct `std::boxed::Box`
```

返回的是`*node`的`&`（引用），但是这里的 node 捕获到的类型是：智能指针的引用，
这里只解引用了一层，得到了指针指针`boxed::Box`。

修正以后就得到了：

```rust
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);  // as_ref
            &node.elem
        })
    }
```

这种`&**node`语法确实很操蛋，可以使用一种语法糖

```rust
        self.next = node.next.as_deref();
```
