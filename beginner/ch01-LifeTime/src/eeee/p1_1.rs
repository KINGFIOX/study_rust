struct Interface<'a, 'b: 'a> {
    manager: &'a mut Manager<'b>,
}

impl<'a, 'b: 'a> Interface<'a, 'b> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str,
}

// list 里面会保存 manager 的所有权
struct List<'a> {
    manager: Manager<'a>,
}

impl<'b> List<'b> {
    pub fn get_interface<'a>(&'a mut self) -> Interface<'a, 'b> {
        Interface {
            manager: &mut self.manager,
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager {
            text: "hello",
        },
    };

    // get_interface 返回一个 interface，这个 interface 里面有 List 中的 manager 的可变借用
    // 按道理来说 interface 里的可变借用，在这里就会归还了，但是有 生命周期的缘故
    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    // 下面的调用会失败，因为同时有不可变/可变借用
    // 但是Interface在之前调用完成后就应该被释放了
    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
