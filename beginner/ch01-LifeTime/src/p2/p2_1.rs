/* 使用生命周期注释结构体
1. `r` 和 `s` 必须是不同生命周期
2. `s` 的生命周期需要大于 'r'
*/
struct DoubleRef<'a, 'b: 'a, T> {
    r: &'a T,
    s: &'b T,
}
fn main() {
    println!("Success!")
}
