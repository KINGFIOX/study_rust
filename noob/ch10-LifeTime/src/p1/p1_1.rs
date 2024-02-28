/* 为 `i` 和 `borrow2` 标注合适的生命周期范围 */

// `i` 拥有最长的生命周期，因为它的作用域完整的包含了 `borrow1` 和 `borrow2` 。
// 而 `borrow1` 和 `borrow2` 的生命周期并无关联，因为它们的作用域没有重叠
fn main() {
    let i = 3;
    {
        let borrow1 = &i; // `borrow1` 生命周期开始. ──┐
        //                                                │
        println!("borrow1: {}", borrow1); //              │
    } // `borrow1` 生命周期结束. ──────────────────────────────────┘
    {
        let borrow2 = &i;

        println!("borrow2: {}", borrow2);
    }
}
