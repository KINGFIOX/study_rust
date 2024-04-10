struct MutStr<'a, 'b> {
    s: &'a mut &'b str,
}

fn main() {
    let mut r = "hello";
    // rust 在修改 可变引用指向的值的时候，需要 用 * 操作符来解引用
    *MutStr { s: &mut r }.s = "hellow";
    println!("{r}")
}
