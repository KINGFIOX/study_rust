use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    // 实现 from 方法
    fn from(value: io::Error) -> Self {
        CliError::IoError(value)
    }
}

impl From<num::ParseIntError> for CliError {
    // 实现 from 方法
    fn from(value: num::ParseIntError) -> Self {
        CliError::ParseError(value)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? 自动将 io::Error 转换成 CliError
    let contents = fs::read_to_string(&file_name)?;

    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn main() {
    println!("Success!")
}
