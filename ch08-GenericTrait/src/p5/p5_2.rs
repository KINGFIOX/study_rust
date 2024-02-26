use std::{ ops::Sub, process::Output };

/**
 * 这个文件讲了 如何为泛型 运算符重构
 */

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

// 用三种方法填空: 其中两种使用默认的泛型参数，另外一种不使用

// // 方法 1
// impl<T: Sub<Output = T>> Sub<Point<T>> for Point<T> {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self::Output {
//         Point {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }

// // 方法 2
// impl<T: Sub<Output = T>> Sub<Self> for Point<T> {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self::Output {
//         Point {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }

// 方法 3
impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 }, Point { x: 1, y: 3 });
    println!("Success!")
}
