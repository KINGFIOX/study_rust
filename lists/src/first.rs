pub struct List {
    head: Link,
}

impl List {
    // 默认构造，初始化为 Link::Empty
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    // 头插
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            // 将 self.head 替换到 next 中，并且让 self.head 指向 Link::Empty
            next: std::mem::replace(&mut self.head, Link::Empty),
        });

        // 重新设置新的 head
        self.head = Link::More(new_node);
    }

    // 头删
    pub fn pop(&mut self) -> Option<i32> {
        // 先将 self.head 替换成 Link::Empty
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next; // 跳到下一个节点
                Some(node.elem)
            }
        }
    }
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

// in first.rs
#[cfg(test)]
mod test {
    use crate::first::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

// impl Drop for List {
//     fn drop(&mut self) {
//         // NOTE: 在 Rust 代码中，我们不能显式的调用 `drop` 方法，只能调用 std::mem::drop 函数
//         // 这里只是在模拟编译器!
//         self.head.drop(); // 尾递归 - good!
//     }
// }

impl Drop for List {
    fn drop(&mut self) {
        // 取走 self.head，并赋值为 Link::Empty，将取走的 self.head 放入 cur_link
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}
