use std::arch::asm;

fn main() {
    let x: u64 = 3;
    let y: u64;

    unsafe {
        asm!("add {0}, 5", inout(reg) x => y);
    }

    println!("{x}, {y}")
}
