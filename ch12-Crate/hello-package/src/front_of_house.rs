// in lib.rs

// 填空并修复错误

// 提示：你需要通过 `pub` 将一些项标记为公有的，这样模块 `front_of_house` 中的项才能被模块外的项访问
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}

        // Maybe you don't want the guest hearing the your complaining about them
        // So just make it private
        fn complain() {}
    }
}

pub fn eat_at_restaurant() {
    // 使用绝对路径调用
    // crate::front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::front_of_house::hosting::add_to_waitlist();

    // 使用相对路径调用
    front_of_house::hosting::add_to_waitlist();
}

/* ---------- ---------- 分割线 ---------- ----------  */
