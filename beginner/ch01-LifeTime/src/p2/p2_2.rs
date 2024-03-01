/* 添加类型约束使下面代码正常运行 */
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 返回 importantExcerpt 中的 str，那么就是要让 结构体的生命周期 比 返回的字段的生命周期长
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    println!("Success!")
}
