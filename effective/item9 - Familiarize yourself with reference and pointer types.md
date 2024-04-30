一个指针的解引用，不能是泛型的。不然这会引起 confuse 。

> As a technical aside, it's worth understanding why the Deref traits can't be generic ( Deref<Target> ) for the destination type. If they were, then it would be possible for some type ConfusedPtr to implement both Deref<TypeA> and Deref<TypeB> , and that would leave the compiler unable to deduce a single unique type for an expression like \*x . So instead the destination type is encoded as the associated type named Target .

fat pointer 应该指的是：大于 8B(64bit) 的指针。
有两种典型：

1. 切片，因为切片有两个维度：start + size
2. trait object ，因为有两个维度：point to obj + point to vtable

## range expression

```rust
pub struct Range<Idx> {
    pub start: Idx,
    pub end: Idx,
}
```

表示的区间是：`[start, end)`

## SliceIndex Trait

在 Rust 中，`SliceIndex` trait 是一个定义了如何对某种类型 `T` 的切片进行索引操作的 trait。这个 trait 的实现决定了当你使用 `[]` 运算符对切片进行索引时，可以接受哪些类型的索引，并定义了这些操作的输出类型。

### 解析 `SliceIndex` 和 `Range`

`SliceIndex` trait 定义在 `std::slice` 模块中，是标准库用来支持对切片的索引操作的一部分。这个 trait 允许你使用不同的索引类型来获取切片的部分数据。例如，你可以使用一个整数来获取单个元素，或者使用一个范围来获取切片的一个子切片。

`SliceIndex` trait 的关键组成如下：

- `index` 方法：这个方法提供对单个元素的访问。
- `index_mut` 方法：这个方法提供对单个元素的可变访问。
- `get` 方法：尝试安全地访问元素，返回 `Option<&T>`。
- `get_mut` 方法：尝试安全地进行可变访问，返回 `Option<&mut T>`。
- `Output` 类型：这个关联类型表示索引操作的结果类型。例如，当使用范围进行索引时，输出类型通常是子切片 `[T]`。

### `Range` 类型作为 `SliceIndex` 的实现

在 Rust 中，范围类型（如 `Range<usize>`, `RangeInclusive<usize>`, 等）实现了 `SliceIndex` trait，允许使用这些范围作为索引操作的参数来提取切片的部分。当你使用一个范围对切片进行索引时，你得到的是一个新的切片，这个切片是原始切片的一个子集。

例如：

```rust
let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..4]; // 使用 Range<usize> 索引，得到 &[2, 3, 4]
```

在这个例子中，`1..4` 是一个 `Range<usize>`，它告诉 Rust 从数组 `arr` 中取出从索引 `1` 到 `3`（不包括 `4`）的元素，结果是一个新的切片 `&[2, 3, 4]`。

### 为什么 `SliceIndex` 重要？

`SliceIndex` trait 的存在使得 Rust 的切片非常灵活和强大。它不仅支持多种索引类型（单个值、范围等），还通过返回 `Option` 类型的方法增加了安全性，因为这可以防止在索引超出范围时发生 panic。此外，`SliceIndex` trait 的泛型实现确保了它可以适用于不同类型的切片，不仅仅是基本数据类型。

## indexing expression && Index Trait && IndexMut Trait

```rust
use std::ops::Index;

struct MyCollection {
    data: Vec<i32>,
}

impl Index<usize> for MyCollection {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

fn main() {
    let collection = MyCollection {
        data: vec![1, 2, 3, 4, 5],
    };

    println!("Element at index 2: {}", collection[2]);
}
```

## AsRef 与 Borrow

AsRef 和 Borrow 两种 trait 在目的和实现上有些区别:

- Borrow trait 用于返回一个引用,以供外部借用该类型的部分内容。

- AsRef trait 用于将一个类型转换为另一个引用类型,主要用于类型转换。

具体来说:

- Borrow::borrow 方法直接返回一个引用,不涉及类型转换。

- AsRef::as_ref 方法返回一个新的引用类型,表示将自身类型转换为另一种引用类型。

例如:

```rust
impl Borrow<str> for String {
  fn borrow(&self) -> &str;
}

impl AsRef<str> for String {
  fn as_ref(&self) -> &str;
}
```

- Borrow 直接返回 `&String` 的引用,不做类型转换。

- AsRef 将 `&String` 转换为 `&str` 类型的引用。

使用场景:

- Borrow 一般用于集合类型返回元素引用,如 HashMap。

- AsRef 用于类型间转换,如将 String 转换为 `&str` 传入函数等。

所以总体来说:

- Borrow 用于直接返回引用不做类型转换
- AsRef 用于类型间引用转换
- 都用于引用的传递,但目的和实现上有些差异

## refcell

> If the underlying type T implements the Copy trait (indicating that a fast bit-for-bit copy produces a valid item, see Item 5), then the Cell<T> type allows interior mutation with less overhead – the get(&self) method copies out the current value, and the set(&self, val) method copies in a new value. The Cell type is used internally by both the Rc and RefCell implementations, for shared tracking of counters that can be mutated without a &mut self .

这句话指的是如果类型 `T` 实现了 `Copy` trait，那么使用 `Cell<T>` 类型可以在进行内部突变时减少开销。

首先，让我们理解一下 `Cell<T>` 的作用：它是一个容器类型，允许你在其中放置一个值 `T`，并在不使用可变引用（`&mut`）的情况下修改这个值。在 `Cell<T>` 中，值的修改是通过复制来完成的，而不是通过引用。因此，`Cell<T>` 提供了一种安全的内部可变性（interior mutability）的机制。

当类型 `T` 实现了 `Copy` trait 时，意味着它的值可以通过简单的位对位复制来进行拷贝，而不需要对原始值进行所有权的转移。
因此，对于 `Cell<T>` 来说，当你获取值时，它只是简单地执行了一个位对位的复制，然后返回了一个新的值。
同样，当你设置一个新的值时，它也只是简单地进行了一个位对位的复制，将新值放入 `Cell<T>` 中。
由于 `Copy` 类型的值可以直接进行复制，所以在使用 `Cell<T>` 时不需要进行额外的内存分配或引用计数的操作，这就减少了额外的开销。

因此，当使用 `Cell<T>` 时，如果 `T` 类型实现了 `Copy` trait，就意味着在进行内部突变时，可以减少额外的开销。

Rc + RefCell 相对的 Arc + Mutex