// 修复错误
struct Array<T, const N: usize> {
    data: [T; N],
}

// 没法修复，除非更改下面代码
fn main() {
    let arrays = [
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1.0, 2.0, 3.0],
        },
        Array {
            data: [1, 2],
        },
    ];
}
