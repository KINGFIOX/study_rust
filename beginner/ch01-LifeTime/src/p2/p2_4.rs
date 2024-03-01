/* 添加 HRTB 使下面代码正常运行！ */
fn call_on_ref_zero<F>(f: F) where F: for<'a> Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}

fn main() {
    println!("Success!")
}
