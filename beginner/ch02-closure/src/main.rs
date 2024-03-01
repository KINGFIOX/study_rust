#![feature(test)]

struct Cacher<T, E> where T: Fn(E) -> E, E: Copy {
    query: T,
    value: Option<E>, // 这里要求 E 是一个 Copy，因为缓存嘛，然后在这个 Cacher 里面保存的也正好是一个副本
}

impl<T, E> Cacher<T, E> where T: Fn(E) -> E, E: Copy {
    fn new(query: T) -> Cacher<T, E> {
        Cacher {
            query,
            value: None,
        }
    }

    // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 1);
}

fn main() {
    println!("Hello, world!");
}
