use std::fs;
use std::env;
use std::error::Error;

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let query = query.to_lowercase();
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }

    // results

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

// in lib.rs
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// 错误处理最好在一个地方 统一的完成
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // 跳过 可执行程序名

        // let query = args[1].clone();
        let query = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Didn't get a query string");
            }
        };

        // let file_path = args[2].clone();
        let file_path = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Didn't get a file path");
            }
        };

        let ignore_case = env::var("IGNORE_CASE").map_or(false, |var| var.eq("1"));

        Ok(Config { query, file_path, ignore_case })
    }
}

/*
 测试驱动开发模式 test drive develop

- 编写一个注定失败的测试，并且失败的原因和你指定的一样
- 编写一个成功的测试
- 编写你的逻辑代码，直到通过测试

 */

// in src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
