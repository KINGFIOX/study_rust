// fn main() {
//     use std::collections::HashMap;
//     use std::hash::Hash;

//     fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
//         where K: Clone + Eq + Hash, V: Default
//     {
//         // get_mut 是返回一个 可变的借用，可以通过 *value = xx 来修改值
//         if let Some(value) = map.get_mut(&key) {
//             value
//         } else {
//             map.insert(key.clone(), V::default());
//             map.get_mut(&key).unwrap()
//         }
//     }
// }

mod eeee;

// mod p1;

mod p2;

fn main() {}
