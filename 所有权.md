### 简单介绍 string

通过字面量创建 string

```rust
let s = String::from("hello");
```

`::`是 “调用” 操作符

在 字符串 末尾插入 元素

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() 在字符串后追加字面值

println!("{}", s); // 将打印 `hello, world!`
```

String 类型是一个复杂类型，由存储在栈中的堆指针、字符串长度、字符串容量共同组成。
容量是堆内存分配空间的大小，长度是目前已经使用的大小

### 可 copy 性

Rust 有一个叫做 Copy 的特征，可以用在类似整型这样在栈中存储的类型。
如果一个类型拥有 Copy 特征，一个旧的变量在被赋值给其他变量后仍然可用。

那么什么类型是可 Copy 的呢？可以查看给定类型的文档来确认，不过作为一个通用的规则： 任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的。如下是一些 Copy 的类型：

- 所有整数类型，比如 u32
- 布尔类型，bool，它的值是 true 和 false
- 所有浮点数类型，比如 f64
- 字符类型，char
- 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是
- 不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意: 可变引用 &mut T 是不可以 Copy 的

### 传值 给 函数，所有权转移

```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
```

### 返回值，所有权转移

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership 将返回值
                                        // 移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                        // takes_and_gives_back 中,
                                        // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
                                             // 调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}
```

### 所有权 之 相同的值存在

1. 拷贝一份

```rust
fn main() {
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);
}
```

2. 并非 String，而是 只是 字面量

```rust
fn main() {
    let x = "hello, world";
    let y = x;
    println!("{},{}",x,y);
}
```

3. 引用

在这段 Rust 代码中，`&` 符号用于创建一个引用（reference）。在这里，`&String::from("hello, world")` 创建了一个对 `String::from("hello, world")` 所有权的不可变借用（immutable borrow）。

通过使用引用，我们可以在不转移所有权的情况下访问值。在这个例子中，变量 `x` 是对 `String` 类型值的引用，它指向存储在堆上的字符串数据。变量 `y` 则将 `x` 的引用赋值给了自己。

由于引用是不可变的，所以在打印 `x` 和 `y` 的值时，它们的值是相同的。这是因为引用只是指向原始数据的指针，并没有创建数据的副本。

需要注意的是，通过引用访问数据时，原始数据必须保持有效。在这个例子中，由于 `x` 是对 `String` 所有权的借用，所以在 `println!` 宏中使用 `x` 和 `y` 的引用是合法的。

引用是 Rust 中重要的概念之一，它使得在不转移所有权的情况下借用数据成为可能，并且在编译时进行了严格的生命周期检查以确保安全性。

```rust
fn main() {
    let x = &String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}
```

4. as_str

在 Rust 中，`as_str` 是一个字符串类型（例如 `String`）的方法，用于将字符串类型转换为 `&str` 类型的引用。`&str` 类型是 Rust 中的一种字符串切片类型，它是对字符串数据的不可变借用。

使用 `as_str` 方法可以方便地将 `String` 类型的字符串转换为 `&str` 类型的引用，从而可以在不转移所有权的情况下访问字符串数据。这在需要使用字符串切片作为函数参数或返回值时非常有用。

需要注意的是，由于 `&str` 类型是对原始数据的借用，因此在使用 `as_str` 方法时需要确保原始数据的有效性。如果原始数据已经被释放或不再有效，引用将会变得无效，可能会导致程序崩溃或出现其他错误。

```rust
fn main() {
    let x = String::from("hello, world");
    let y = x.as_str();
    println!("{},{}",x,y);
}
```

### 允许修改 string

在 Rust 中，`mut` 是一个关键字，用于声明可变绑定（mutable binding）。
通过在变量名之前加上 `mut` 关键字，可以将变量声明为可变的，允许对其进行修改。

在你提供的代码中，变量 `s1` 被声明为可变绑定，
即 `mut s1`。这意味着可以对 `s1` 进行修改，包括改变其值。
在这种情况下，你使用 `let mut s1 = s;` 将 `s` 的所有权转移给了 `s1`，
使得 `s1` 成为了拥有字符串数据的 `String` 类型变量。

随后，你尝试使用 `s1.push_str("world")` 向 `s1` 中追加字符串 "world"。
由于 `s1` 是可变的，这个操作是合法的，并且会修改 `s1` 的值。

如果你省略了 `mut` 关键字，例如 `let s1 = s;`，
则会将 `s` 的所有权转移给 `s1`，但此时 `s1` 是不可变的，
无法对其进行修改。因此，尝试调用 `push_str` 方法会导致编译错误。

通过使用 `mut` 关键字，你可以显式地声明变量为可变绑定，从而在需要修改变量值的情况下编译通过。

```rust
fn main() {
    let s = String::from("hello, ");

    // 只修改下面这行代码 !
    let mut s1 = s;

    s1.push_str("world")
}
```

### 解引用

在这段代码中，`y` 是一个可变绑定的变量名，它是通过对 `x` 进行克隆（`clone`）操作得到的。
`clone` 方法用于创建一个值的深拷贝，这意味着它会复制原始值的所有内容并创建一个新的独立副本。

在这里，`x` 是一个 `Box<i32>` 类型的智能指针，它指向堆上分配的整数值 `5`。
通过调用 `x.clone()`，我们创建了一个新的 `Box<i32>` 类型的变量 `y`，它也指向堆上的整数值 `5` 的副本。

接下来，我们尝试通过解引用操作符 `*` 修改 `y` 的值为 `4`，即 `*y = 4;`。
由于 `y` 是一个可变绑定，这个操作是合法的，并且会修改 `y` 所指向的整数值。

最后，我们使用断言宏 `assert_eq!` 来断言 `*x` 的值为 `5`。
由于 `x` 和 `y` 是独立的副本，修改 `y` 的值不会影响到 `x` 的值，因此断言成功。

需要注意的是，在这段代码中，解引用操作符 `*` 被用于修改指针所指向的值。
而在 Rust 中，解引用操作符也可以用于获取指针所指向的值。
例如，可以使用 `*x` 来获取 `x` 指针所指向的整数值。

```rust
fn main() {
    let x = Box::new(5);

    let mut y = Box::new(3);       // implement this line, dont change other lines!

    *y = 4;

    assert_eq!(*x, 5);
}
```

### 解构？ 引用？ 所有权？

```rust
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);
}
```

### 使用 结构体成员

```rust
fn main() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0;

   println!("{:?}", t.1);
}
```

在这段代码中，`{:?}` 是一个格式化字符串的占位符，用于在打印语句中以调试格式打印值。

具体来说，`{:?}` 会将值以调试格式打印出来，通常用于调试目的。对于字符串类型，它会以双引号包裹字符串并显示转义字符。对于其他类型，它会以一种可读性较高的方式打印出值的内容。

在你提供的代码中，`println!("{:?}", t.1);` 使用 `{:?}` 占位符来打印元组 `t` 的第二个元素 `t.1` 的值。这将以调试格式打印出 `t.1` 的内容。

通过使用 `{:?}` 占位符，我们可以方便地在开发过程中查看变量的值，以便进行调试和错误排查。

### 可变引用

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

TIPS: 可变引用在 同一作用域 中，只能存在一个

### 所有权转移 与 引用

这段代码是 "可变引用"

```rust
fn main() {
    let mut s = String::from("hello, ");

    // fill the blank to make it work
    let p = &mut s;

    p.push_str("world");
}
```

这段代码只是所有权的转移

```rust
fn main() {
    let mut s = String::from("hello, ");

    // 填写空白处，让代码工作
    let mut p = s;

    p.push_str("world");
}
```

### ref 引用 与 & 的区别 ？

```rust
fn main() {
    let c = '中';

    let r1 = &c;
    // fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));
}

// get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
```

在 Rust 中，`ref` 和 `&` 是用于创建引用的关键字，但它们有一些不同之处。

`ref` 关键字通常用于模式匹配的一部分，用于将值绑定到新的变量上。
例如，`let ref x = value;` 将 `value` 的引用绑定到变量 `x` 上。
使用 `ref` 创建的引用是不可变的，并且其生命周期与绑定的变量的作用域相同。

`&` 符号是用于创建引用的操作符。
通过使用 `&`，可以创建不可变引用和可变引用。
不可变引用允许读取数据，但不能修改数据，而可变引用允许读取和修改数据。
引用的生命周期由程序的上下文和借用规则决定。

总结一下：

- `ref` 是模式匹配的一部分，用于创建不可变引用，并将其绑定到新的变量上。
- `&` 是用于创建引用的操作符，可以创建不可变引用和可变引用。

需要注意的是，这里提到的是 Rust 中的关键字和操作符，它们与其他编程语言中类似的语法元素可能有不同的含义和行为。

### 不能将 不可变对象 搞成 可变引用
