pub struct List<T> {
    head: Link<T>,
}

// enum Link {
//     Empty,
//     More(Box<Node>),
// }
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            // next: mem::replace(&mut self.head, None),
            // Takes the value out of the option, leaving a None in its place
            // let mut x = Some(2);
            // let y = x.take();
            // assert_eq!(x, None);
            // assert_eq!(y, Some(2));
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // match mem::replace(&mut self.head, None) {
        // match self.head.take() {
        //     None => None,
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        }) // 闭包，突然让我想到了 Monad
    }

    pub fn peek(&self) -> Option<&T> {
        // self.head.map(|node| { &node.elem })
        self.head.as_ref().map(|node| { &node.elem })
    }
}

impl<T> Drop for List<T> {
    // 析构函数
    fn drop(&mut self) {
        // let mut cur_link = mem::replace(&mut self.head, None);
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            // cur_link = mem::replace(&mut boxed_node.next, None);
            cur_link = boxed_node.next.take();
        }
    }
}

/* ---------- ---------- 拿走所有权 迭代器 ---------- ---------- */

// next 是负责将 iter 指针往后移动一个元素

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop() // self.0 的类型是 List<T>
    }
}

#[test]
fn into_iter() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

/* ---------- ---------- 不可变借用 迭代器 ---------- ---------- */

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    // 要求 self 至少与 Iter  活的一样久
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // Iter { next: self.head.as_ref().map(|node| &**node) }
        Iter { next: self.head.as_deref() }
    }

    // pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    //     Iter { next: self.head.map(|node| &node) }
    // }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // self.next = node.next.as_ref().map(|node| &*node);  // 错误，因为类型不匹配，引用
            // self.next = node.next.as_ref().map(|node| &**node);  // as_ref
            self.next = node.next.as_deref(); // 向后偏移一个元素
            &node.elem // 并返回当前节点的元素
        })
    }
}

/* ---------- ---------- 可变借用 迭代器 ---------- ---------- */

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[test]
fn iter_mut() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
}
