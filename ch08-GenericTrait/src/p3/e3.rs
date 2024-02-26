struct Sheep {
    naked: bool,
    name: String,
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // `Sheep` 结构体上定义的方法可以调用 `Sheep` 所实现的特征的方法
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

trait Animal {
    // 关联函数签名；`Self` 指代实现者的类型
    // 例如我们在为 Pig 类型实现特征时，那 `new` 函数就会返回一个 `Pig` 类型的实例，这里的 `Self` 指代的就是 `Pig` 类型
    fn new(name: String) -> Self;

    // 方法签名
    fn name(&self) -> String;

    fn noise(&self) -> String;

    // 方法还能提供默认的定义实现
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Animal for Sheep {
    // `Self` 被替换成具体的实现者类型： `Sheep`
    fn new(name: String) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn noise(&self) -> String {
        if self.is_naked() { "baaaaah?".to_string() } else { "baaaaah!".to_string() }
    }

    // 默认的特征方法可以被重写
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // 这里的类型注释时必须的
    // let mut dolly: Sheep = Animal::new("Dolly".to_string());
    let mut dolly = Animal::new("Dolly".to_string());
    // TODO ^ 尝试去除类型注释，看看会发生什么

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
