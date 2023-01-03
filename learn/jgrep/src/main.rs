use std::{env, process};

use jgrep::Config;
fn main() {
    // 读取参数

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument:{}", err);
        process::exit(1);
    });

    // 开始获取文件内容

    if let Err(e) = jgrep::run(config) {
        eprintln!("Application error:{}", e);
        process::exit(1);
    };
}
