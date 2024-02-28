struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

// // Returns some struct that implements Animal, but we don't know which one at compile time.
// // FIX the erros here, you can make a fake random, or you can use trait object
// fn random_animal(random_number: f64) -> impl Animal {
//     if random_number < 0.5 {
//         Sheep {}
//     } else {
//         Sheep {}
//     }
//     // 这里，两个返回语句是 相同的，也就是 虚假随机
// }

// 特征对象
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 { Box::new(Sheep {}) } else { Box::new(Cow {}) }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}
