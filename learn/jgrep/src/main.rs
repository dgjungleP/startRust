use std::{env, process};

use jgrep::Config;
fn main() {
    // 读取参数
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument:{}", err);
        process::exit(1);
    });

    // 开始获取文件内容

    if let Err(e) = jgrep::run(config) {
        eprintln!("Application error:{}", e);
        process::exit(1);
    };
}
