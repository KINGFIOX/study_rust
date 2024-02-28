use std::num::ParseIntError;

// 填空
type Res<T> = Result<T, ParseIntError>;

// 使用上面的别名来引用原来的 `Result` 类型
fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str
        .parse::<i32>()
        .and_then(|first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
        })
}

// 同样, 这里也使用了类型别名来简化代码
fn print(result: Res<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));

    println!("Success!")
}
