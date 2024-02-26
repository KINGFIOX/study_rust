// 实现一个结构体 Point 让代码工作
struct Point<T, U> {
    x: T,
    y: U,
}

fn p3() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
