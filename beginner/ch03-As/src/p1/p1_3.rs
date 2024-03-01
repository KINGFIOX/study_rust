fn main() {
    assert_eq!(1000 as u16, 1000);

    // assert_eq!(1000 as u8, 0xe8);

    // 事实上，之前说的规则对于正整数而言，就是如下的取模
    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, 255);

    // 从 Rust 1.45 开始，当浮点数超出目标整数的范围时，转化会直接取正整数取值范围的最大或最小值
    // 原来如此
    // assert_eq!(300.1_f32 as u8, 0xcd);
    // assert_eq!(-100.1_f32 as u8, 0x33);
    assert_eq!(300.1_f32 as u8, 255);
    assert_eq!(-100.1_f32 as u8, 0);

    // 上面的浮点数转换有一点性能损耗，如果大家对于某段代码有极致的性能要求，
    // 可以考虑下面的方法，但是这些方法的结果可能会溢出并且返回一些无意义的值
    // 总之，请小心使用
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", (300.0_f32).to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
