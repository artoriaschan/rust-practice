use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // unwrap_or_else方法, 当OK时, 执行逻辑与unwrap相同, 当返回Err时, 执行定义的闭包
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
