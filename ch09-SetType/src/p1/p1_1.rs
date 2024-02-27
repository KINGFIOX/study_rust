fn main() {
    let arr: [u8; 3] = [1, 2, 3];

    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    // vec!(..) 和 vec![..] 是同样的宏，宏可以使用 []、()、{}三种形式，因此...
    let v = vec![1, 2, 3];
    is_vec(&v);

    // ...在下面的代码中, v 是 Vec<[u8; 3]> , 而不是 Vec<u8>
    // 使用 Vec::new 和 `for` 来重写下面这段代码
    // let v1 = vec![arr];
    let mut v1 = Vec::new();
    for i in &v {
        v1.push(*i);
    }
    is_vec(&v1);

    assert_eq!(format!("{:?}", v), format!("{:?}", v1));

    println!("Success!")
}

fn is_vec(v: &Vec<u8>) {}
