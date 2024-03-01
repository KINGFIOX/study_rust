/* 添加类型约束使下面代码正常运行 */

// y = x，也就是 x 赋值到 y，那么 x 的生命周期 肯定要比 y 长
fn f<'a: 'b, 'b>(x: &'a i32, mut y: &'b i32) {
    y = x;
    let r: &'b &'a i32 = &&0;
}

fn main() {
    println!("Success!")
}
