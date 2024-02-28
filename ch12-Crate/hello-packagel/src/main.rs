mod front_of_house;

// 填空并修复错误
fn main() {
    assert_eq!(front_of_house::hosting::seat_at_table(), "sit down please");

    // 注意一下这里，这里是 crate 的名称
    assert_eq!(hello_packagel::eat_at_restaurant(), "yummy yummy!");
}
