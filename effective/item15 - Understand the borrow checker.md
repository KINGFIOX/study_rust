## 自引用类型

```rust
struct SelfRefIdx {
    text: String,
    title: Option<Range<usize>>,
}

impl SelfRefIdx {
    fn new(text: String, title_start: usize, title_end: usize) -> Self {
        SelfRefIdx {
            text,
            title: Some(title_start..title_end),
        }
    }

    fn title(&self) -> Option<&str> {
        self.title.as_ref().map(|range| &self.text[range.clone()])
    }
}

fn main() {
    let text = "Hello, World! This is an example text.".to_string();
    let doc = SelfRefIdx::new(text, 0, 12);

    if let Some(title) = doc.title() {
        println!("Title: {}", title);
    }
}
```

这里的关键点是 `Range<usize>` 不持有字符串数据的所有权或引用，而仅仅存储了指向 `text` 字符串内部某部分的起始和结束索引。这意味着，尽管结构体中的 `text` 和 `title` 看起来是自引用的，但从 Rust 的内存安全性角度看，这是完全合法的。因为 `Range<usize>` 实际上只是一个包含两个 `usize` 值的简单结构体，它本身不创建对 `text` 字符串内容的引用。

自引用是不好的：

```rust
struct SelfRef<'a> { 
	text: String,
	title: Option<&'a str>,  // 引用一定要有 lifetime tick
}
```

但是 lifetime tick 却具有传播性。 自引用在语义上也是有问题的，
特别是如果对象在栈上，实际上返回的时候，对象的地址会发生改变。
当然也可以用 pin 啦

可以使用一些 crate 来处理自引用类型

Where possible, **avoid self-referential data structures** or try to find library crates that encapsulate the difficulties for you (e.g. [`ouroborous`](https://crates.io/crates/ouroboros)).
