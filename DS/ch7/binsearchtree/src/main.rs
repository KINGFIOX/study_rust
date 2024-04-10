use std::cmp::Ordering;
use std::collections::btree_map::Keys;
use std::ops::Deref;

type Link<T, U> = Option<Box<BST<T, U>>>;

struct BST<T, U> {
    key: Option<T>,
    val: Option<U>,
    left: Link<T, U>,
    right: Link<T, U>,
}

impl<T, U> BST<T, U>
where
    T: Clone + Ord + std::fmt::Debug,
    U: Clone + std::fmt::Debug,
{
    fn new() -> Self {
        BST {
            key: None,
            val: None,
            left: None,
            right: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.key.is_none()
    }

    fn len(&self) -> usize {
        self.calc_len(0)
    }

    fn calc_len(&self, mut i: usize) -> usize {
        if self.key.is_none() {
            return i;
        }
        i += 1;
        if !self.left.is_none() {
            i = self.left.as_ref().unwrap().calc_len(i);
        }
        if !self.right.is_none() {
            i = self.right.as_ref().unwrap().calc_len(i);
        }
        i
    }

    fn preorder(&self) {
        // 根 左 右
        println!("key: {:?}, val: {:?}", self.key, self.val);
        match &self.left {
            Some(node) => node.preorder(),
            None => (),
        }
        match self.right.as_ref() {
            Some(node) => node.preorder(),
            None => (),
        }
    }

    fn inorder(&self) {
        // 根 左 右
        match &self.left {
            Some(node) => node.preorder(),
            None => (),
        }
        println!("key: {:?}, val: {:?}", self.key, self.val);
        match self.right.as_ref() {
            Some(node) => node.preorder(),
            None => (),
        }
    }

    fn postorder(&self) {
        // 根 左 右
        match &self.left {
            Some(node) => node.preorder(),
            None => (),
        }
        match self.right.as_ref() {
            Some(node) => node.preorder(),
            None => (),
        }
        println!("key: {:?}, val: {:?}", self.key, self.val);
    }

    fn insert(&mut self, key: T, val: U) {
        if self.key.is_none() {
            self.key = Some(key);
            self.val = Some(val);
        } else {
            match &self.key {
                Some(k) => {
                    // 存在 key ，更新 val
                    if key == *k {
                        self.val = Some(val);
                        return;
                    }
                    let child = if key < *k {
                        &mut self.left
                    } else {
                        &mut self.right
                    };
                    match child {
                        Some(ref mut node) => {
                            // 如果还有子节点，那么递归的插入
                            node.insert(key, val);
                        }
                        None => {
                            let mut node = BST::new();
                            node.insert(key, val);
                            *child = Some(Box::new(node));
                        }
                    };
                }
                None => (),
            }
        }
    }

    fn search(&self, key: &T) -> bool {
        match &self.key {
            // cur.key < key ? 左边找 : 右边找
            Some(k) => match k.cmp(&key) {
                Ordering::Equal => return true,
                Ordering::Less => match &self.right {
                    Some(node) => return node.search(key),
                    None => return false,
                },
                Ordering::Greater => match &self.left {
                    Some(node) => return node.search(key),
                    None => return false,
                },
            },
            None => return false,
        }
    }

    fn min(&self) -> (Option<&T>, Option<&U>) {
        match &self.left {
            Some(node) => node.min(), // 最左边的节点
            None => match &self.key {
                Some(key) => (Some(&key), self.val.as_ref()),
                None => (None, None),
            },
        }
    }

    fn max(&self) -> (Option<&T>, Option<&U>) {
        match &self.right {
            Some(node) => node.max(), // 最左边的节点
            None => match &self.key {
                Some(key) => (Some(&key), self.val.as_ref()),
                None => (None, None),
            },
        }
    }

    fn get(&self, key: &T) -> Option<&U> {
        match &self.key {
            None => None,
            Some(k) => match k.cmp(&key) {
                Ordering::Equal => self.val.as_ref(),
                Ordering::Greater => match &self.left {
                    None => None,
                    Some(node) => node.get(key),
                },
                Ordering::Less => match &self.right {
                    None => None,
                    Some(node) => node.get(key),
                },
            },
        }
    }
}

fn main() {
    println!("Hello, world!");
}
