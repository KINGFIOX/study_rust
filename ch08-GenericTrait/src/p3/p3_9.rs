// 填空
fn example1() {
    // `T: Trait` 是最常使用的方式
    // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.value(10), 11u32);
    assert_eq!(cacher.value(15), 11u32);
}

fn example2() {
    // 还可以使用 `where` 来约束 T
    struct Cacher<T> where T: Fn(u32) -> u32 {
        // T 是
        calculation: T, // 保存一个函数
        value: Option<u32>,
    }

    impl<T> Cacher<T> where T: Fn(u32) -> u32 {
        // new
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None, // new 出来，默认是 None
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    let mut cacher = Cacher::new(|x| x + 1);

    assert_eq!(cacher.value(20), 21u32); // 如果是 None ，那么就 match 逻辑中
    assert_eq!(cacher.value(25), 21u32);
}

fn main() {
    example1();
    example2();

    println!("Success!")
}
