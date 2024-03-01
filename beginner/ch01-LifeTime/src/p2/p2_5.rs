/* 通过重新排序一些代码使下面代码正常运行 */
fn main() {
    let mut data = 10;
    let ref1 = &mut data;
    let ref2 = &mut *ref1;

    *ref2 += 2;

    *ref1 += 1;

    println!("{}", data);
}
