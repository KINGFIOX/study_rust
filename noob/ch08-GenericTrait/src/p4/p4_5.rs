// 使用至少两种方法让代码工作
// 不要添加/删除任何代码行
trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> Self {
        42
    }
}

impl MyTrait for String {
    fn f(&self) -> Self {
        self.clone()
    }
}

fn my_function<T: MyTrait>(x: Box<T>) -> impl MyTrait {
    x.f()
}

fn main() {
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));

    println!("Success!")
}
