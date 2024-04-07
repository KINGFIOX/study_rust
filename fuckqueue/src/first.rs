use std::rc::Rc;
use std::cell::{ Ref, RefMut, RefCell };

/* ---------- ---------- 双端队列的实现 ---------- ---------- */

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(Node {
                elem,
                prev: None,
                next: None,
            })
        )
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    /*
     * 头插
     */
    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        // 将 head 的所有权转移到临时对象中
        match self.head.take() {
            Some(old_head) => {
                // 非空链表，将新的节点跟老的头部相链接
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                // 空链表，将 head 与 tail 都设置为 传入的节点
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            // 总是转移头结点的下一个节点
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // 非空链表
                    new_head.borrow_mut().prev.take(); // 将 第二个节点的 prev 设置为空
                    self.head = Some(new_head);
                }
                // 如果头结点的下一个节点就是 None 的话，那么说明 tail == head ，只有一个元素了，那么就将 tail/head 的所有权转移走
                None => {
                    self.tail.take();
                }
            }
            // old_head.borrow_mut().elem
            // old_head.into_inner().elem
            // Rc::try_unwrap(old_head) 尝试获取 Rc 中值的所有权，返回值是 Result<T, Rc<T>>，失败情况就是啥也没变，毕竟只是 try
            // Result::ok 就是：将 Result 转换为 Option
            // Option::unwrap 就是：Some(T) --> T，如果是空的，那么就引发 panic
            // // 当然，根据分析边界条件，实际上并不会有
            // RefCell::into_inner 将 RefCell 中的值取出来，在这里也就是 Node
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        // node.brrow() 返回了一个临时对象 fn borrow<'a>(&'a self) -> Ref<'a, T>
        // self.head.as_ref().map(|node| { &node.borrow().elem })

        // 首先 as_ref(&self) -> Option<&T>，如果是 Some(xxx)，那么就会把 xxx 传入到 lambda 中；如果是 None，那么什么都不做
        // Ref::map 是 RefCell 模式的一个高级用法，看 README
        self.head.as_ref().map(|node| { Ref::map(node.borrow(), |node| &node.elem) })
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| { RefMut::map(node.borrow_mut(), |node| &mut node.elem) })
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| { Ref::map(node.borrow(), |node| &node.elem) })
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| { RefMut::map(node.borrow_mut(), |node| &mut node.elem) })
    }
}

/*
 * 最麻烦的就是：循环引用的问题了，这干脆不停的 头删
 */
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

/* ---------- ---------- 迭代器（所有权） ---------- ---------- */

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

/* ---------- ---------- 测试用例 ---------- ---------- */

#[test]
fn peek() {
    let mut list = List::new();
    assert!(list.peek_front().is_none());
    assert!(list.peek_back().is_none());
    assert!(list.peek_front_mut().is_none());
    assert!(list.peek_back_mut().is_none());

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    assert_eq!(&*list.peek_front().unwrap(), &3);
    assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
    assert_eq!(&*list.peek_back().unwrap(), &1);
    assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
}

#[test]
fn basics() {
    let mut list = List::new();

    // Check empty list behaves right
    assert_eq!(list.pop_front(), None);

    // Populate list
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push_front(4);
    list.push_front(5);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);

    // ---- back -----

    // Check empty list behaves right
    assert_eq!(list.pop_back(), None);

    // Populate list
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    // Check normal removal
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push_back(4);
    list.push_back(5);

    // Check normal removal
    assert_eq!(list.pop_back(), Some(5));
    assert_eq!(list.pop_back(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
}
