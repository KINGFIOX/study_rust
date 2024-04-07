# README - 分叉链表

```rust
impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}
```

这个意思就是：Iter 中引用的数据，存活的时间不会比 self 长

可以选择自动推导生命周期

```rust
impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}
```
