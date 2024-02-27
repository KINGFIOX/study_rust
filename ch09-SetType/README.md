# README - 集合类型

## if let 与 match

下面是将原来使用`if let`的代码改写成使用`match`语句的形式。
这里的例子是针对处理`Option<i32>`类型的值，我们会匹配`Some`和`None`两种情况：

原`if let`代码：

```rust
let some_option: Option<i32> = Some(10);

if let Some(value) = some_option {
    println!("找到一个值：{}", value);
} else {
    println!("找不到值。");
}
```

改写成`match`的形式：

```rust
let some_option: Option<i32> = Some(10);

match some_option {
    Some(value) => println!("找到一个值：{}", value),
    None => println!("找不到值。"),
}
```

在这个改写的版本中，`match`语句直接匹配`some_option`的值。
如果`some_option`是`Some(value)`，那么它会执行第一个分支，打印出里面的值。
如果`some_option`是`None`，则执行第二个分支，打印出一个表示值未找到的消息。
这种方式比`if let`稍微冗长一点，但是它使得处理所有可能的匹配情况变得直接和明确。
