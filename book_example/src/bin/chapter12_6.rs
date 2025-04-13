use std::{env, process};

use book_example::chapter12_5::minigrep::{self, Config};

// エラーメッセージを出力するコードを、標準エラー出力に書き込むように変更。
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
