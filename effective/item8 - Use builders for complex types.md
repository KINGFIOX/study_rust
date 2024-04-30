建造者模式

```rust
use chrono::{Date, Utc};

// 产品：人物详细信息
pub struct PersonDetails {
    first_name: String,
    last_name: String,
    birth_date: Date<Utc>,
    middle_name: Option<String>,
    preferred_name: Option<String>,
    last_seen: Option<Date<Utc>>,
}

// 建造者
pub struct DetailsBuilder {
    first_name: String,
    last_name: String,
    birth_date: Date<Utc>,
    middle_name: Option<String>,
    preferred_name: Option<String>,
    last_seen: Option<Date<Utc>>,
}

impl DetailsBuilder {
    // 初始化建造者
    pub fn new(first_name: &str, last_name: &str, birth_date: Date<Utc>) -> Self {
        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            birth_date,
            middle_name: None,
            preferred_name: None,
            last_seen: None,
        }
    }

    // 设置中间名
    pub fn middle_name(mut self, middle_name: &str) -> Self {
        self.middle_name = Some(middle_name.to_string());
        self
    }

    // 设置首选名
    pub fn preferred_name(mut self, preferred_name: &str) -> Self {
        self.preferred_name = Some(preferred_name.to_string());
        self
    }

    // 设置最后见面的时间为现在
    pub fn just_seen(mut self) -> Self {
        self.last_seen = Some(Utc::now().date());
        self
    }

    // 构建最终产品
    pub fn build(self) -> PersonDetails {
        PersonDetails {
            first_name: self.first_name,
            last_name: self.last_name,
            birth_date: self.birth_date,
            middle_name: self.middle_name,
            preferred_name: self.preferred_name,
            last_seen: self.last_seen,
        }
    }
}

fn main() {
    let also_bob = DetailsBuilder::new("Robert", "Builder", Utc.ymd(1998, 11, 28))
        .middle_name("the")
        .preferred_name("Bob")
        .just_seen()
        .build();

    println!("Person created: {}, {} {}", also_bob.preferred_name.unwrap_or("".to_string()), also_bob.middle_name.unwrap_or("".to_string()), also_bob.last_name);
}
```

这种建造者模式，也就是：先用 default ，在提供一些方法能够修改这种 default field ，最后再使用 build 方法，返回实体。

就是可以看到 builder 实际上是 获取所有权的。获取所有权意味着：用完了就扔掉的。

当然，如果是想要复用的话，那么要注意 `mut self` `&selt` `&mut self` 啦。
