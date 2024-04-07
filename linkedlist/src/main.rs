// in third.rs
use std::rc::Rc;
use std::cell::RefCell;

/* ---------- ---------- 链表 ---------- ---------- */

struct Node<T> {
    elem: T,
    next: Link<T>,
} // 节点的定义

type Link<T> = Option<Rc<Node<T>>>; // 指针的定义

pub struct List<T> {
    head: Link<T>, // 链表的定义
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(
                Rc::new(Node {
                    elem,
                    next: self.head.clone(),
                })
            ),
        }
    }

    pub fn tail(&self) -> List<T> {
        // mismatched types expected enum `Option<Rc<_>>` found enum `Option<Option<Rc<_>>>`
        // List { head: self.head.as_ref().map(|node| node.next.clone()) }
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // take 就是所有权转移的意思，他实际上是：cur_link = self.head; self.head = None
        // 这里就是不停的将 self 每一个节点设置为了 None
        let mut head = self.head.take();
        while let Some(node) = head {
            // try_unwrap 如果只有一个 强引用，那么就返回 Ok ，否则返回 Self（就不 unwrap 呗）
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

/* ---------- ---------- 迭代器（不可变借用） ---------- ---------- */

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        // self.head 的类型是 Link<T> == Option<Rc<Node<T>>>
        // Node<T> {elem : T, next : Link<T> }
        Iter { next: self.head.as_deref() } // 这里得到的是 Node<T>
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    // 这些泛型应该如何添加呢？
    // 首先 Iter 有两个 泛型，因此 impl 就要有两个泛型
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
        // 相当于： node = self.next，并且这个 lambda 中捕获了 self
        // self.next = node.next == self.next.next
        // return node.elem 也就是 self.next.elem
    }
}

/* ---------- ---------- 迭代器（可变借用） ---------- ---------- */

// 这一段代码不一定能实现，因为 Rc<T> 并不支持 可变引用

// // 定义 IterMut 结构体，与 Iter 类似，但用于可变引用
// pub struct IterMut<'a, T> {
//     next: Option<&'a mut Node<T>>, // 与 Iter 不同，这里持有的是可变引用
// }

// impl<T> List<T> {
//     pub fn iter_mut(&mut self) -> IterMut<'_, T> {
//         IterMut { next: self.head.as_deref_mut() } // 使用 as_deref_mut 获取可变引用
//     }
// }

// impl<'a, T> Iterator for IterMut<'a, T> {
//     type Item = &'a mut T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.take().map(|node| {
//             self.next = node.next.map(|next| &mut **next); // 更新为下一个节点的可变引用
//             &mut node.elem
//         })
//     }
// }

fn main() {
    println!("Hello, world!");
}
