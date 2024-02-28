// 修复错误并实现缺失的代码
fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v[i]);
    }

    for i in 0..5 {
        // 实现这里的代码...
        if let Some(x) = v.get(i) {
            v[i] = x + 1;
        } else {
            v.push(i + 2);
        }
        // match v.get(i) {
        //     Some(x) => {
        //         v[i] = x + 1;
        //     }
        //     None => {
        //         v.push(i + 1);
        //     }
        // }
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!")
}
