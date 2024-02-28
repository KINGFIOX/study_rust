use std::fmt::Debug;

struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}

// T 表明了 一定要实现 Debug
impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
    // ...
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArrayPair").field("left", &self.left).field("right", &self.right).finish()
    }
}
