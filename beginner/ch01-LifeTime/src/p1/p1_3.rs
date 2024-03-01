fn main() {
    let static_string: &str;
    {
        // 字符串字面量能跟程序活得一样久，因此 `static_string` 的生命周期是 `'static`
        static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // 当 `static_string` 超出作用域时，该引用就无法再被使用，但是引用指向的数据( 字符串字面量 ) 依然保存在二进制 binary 所占用的内存中
    }

    println!("static_string reference remains alive: {}", static_string);
}
