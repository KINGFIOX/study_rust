// fn main() {
//     println!("Hello, world!");

//     let mut x = 10;

//     // 不可变指针 可变数据
//     let mut data1: i32 = 10;
//     let p1: &mut i32 = &mut data1; // 不可变指针指向可变数据
//     *p1 = 100;
//     // p1 = &mut x;
//     println!("{data1}");

//     let mut data2: i32 = 10;
//     let mut p2: &mut i32 = &mut data2; // 不可变指针指向可变数据
//     let _p2: &mut &mut i32 = &mut p2;

//     let data3: i32 = 10;
//     let mut p3: &i32 = &data3;
//     let _p3: &mut &i32 = &mut p3;
//     println!("*p3={}", *p3);
//     println!("*p3={}", p3);
// }

// struct Inspector<'a>(&'a u8);

// struct World<'a> {
//     inspector: Option<Inspector<'a>>,
//     days: Box<u8>,
// }

// impl<'a> Drop for Inspector<'a> {
//     fn drop(&mut self) {
//         println!("I was only {} days from retirement!", self.0);
//     }
// }

// fn main() {
//     let mut world = World {
//         inspector: None,
//         days: Box::new(1),
//     };
//     world.inspector = Some(Inspector(&world.days));
//     // 如果 `days` 碰巧在这里被析构了，然后 Inspector 才被析构，就会造成`内存释放后读取`的问题！
// }

fn opaque_read(val: &i32) {
    println!("{}", val);
}

use std::cell::UnsafeCell;
fn main() {
    unsafe {
        let mut data = UnsafeCell::new(10);
        let mref1 = &mut data; // Mutable ref to the *outside*
        let ptr2 = mref1.get(); // Get a raw pointer to the insides
        let sref3 = &*mref1; // Get a shared ref to the *outside*

        *ptr2 += 2; // Mutate with the raw pointer
        opaque_read(&*sref3.get()); // Read from the shared ref

        *sref3.get() += 3; // Write through the shared ref
        *mref1.get() += 1; // Mutate with the mutable ref

        println!("{}", *data.get());
    }
}

//     let mut data = 10;
//     let mref1 = &mut data;
//     let sref2 = &mref1;
//     let sref3 = sref2;
//     let sref4 = &*sref2;

//     // Random hash of shared reference reads
//     opaque_read(sref3);
//     opaque_read(sref2);
//     opaque_read(sref4);
//     opaque_read(sref2);
//     opaque_read(sref3);

//     *mref1 += 1;

//     opaque_read(&data);
// }
