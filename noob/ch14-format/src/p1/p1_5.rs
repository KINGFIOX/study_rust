fn main() {
    println!("Hello {:5}!", 5); // => Hello     5!
    println!("Hello {:+}!", 5); // =>  H!ello +5!
    println!("Hello {:05}!", 5); // => Hello 00005!
    println!("Hello {:05}!", -5); // => Hello -0005!

    /* 填空 */
    assert!(format!("{number:0>width$}", number = 1, width = 6) == "000001");

    println!("Success!")
}
