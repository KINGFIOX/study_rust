// // 函数签名 所说： input : &mut T, val : T
// // #input = &mut hello ，这里 hello : &'static str ---(in_variant)---> T_#input = &'static str
// // #val = &world => T_#val = &String => T_#val = &str => T_#val = &'world str != T_#input = &'static str
// fn assign<T>(input: &mut T, val: T) {
//     // val <: input 至少一样有用，包含 生命周期
//     *input = val;
// }

// fn main() {
//     let mut hello: &'static str = "hello";
//     {
//         let world = String::from("world");
//         assign(&mut hello, &world);
//     }
//     println!("{hello}");
// }

fn main() {}
