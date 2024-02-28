use std::process;
use std::env;

use minigrep::Config;
use minigrep::run;

/*

main.rs 应该有的功能

- 解析命令行参数
- 初始化其它配置
- 调用 lib.rs 中的 run 函数，以启动逻辑代码的运行
- 如果 run 返回一个错误，需要对该错误进行处理

*/

fn main() {
    // 使用 迭代器 直接改善代码
    // let args: Vec<String> = env::args().collect();

    // unwrap_or_else 返回：如果是 Ok，返回原始数据；如果是 Err，传入到闭包函数中处理
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {} in file {}", config.query, config.file_path);

    if let Err(e) = run(config) {
        println!("application error: {e}"); // 捕捉上下文中的 e
        process::exit(1);
    }
}
