/* 使下面代码正常运行 */
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

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
