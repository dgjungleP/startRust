use std::{env, process};

use jgrep::Config;
fn main() {
    // 读取参数
    let args: Vec<String> = env::args().collect();
    println!("Args {:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument:{}", err);
        process::exit(1);
    });

    println!("Search string:{}", config.query);
    println!("In file:{}", config.filename);

    // 开始获取文件内容

    if let Err(e) = jgrep::run(config) {
        println!("Application error:{}", e);
        process::exit(1);
    };
}
