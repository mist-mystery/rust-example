use std::{env, process};

use book_example::chapter12_4::minigrep::{self, Config};

// main 関数の責任は以下に限られる。
// - コマンドライン引数の解析ロジックを呼び出す。
// - ライブラリクレートの run 関数を呼ぶ。
// - エラーを返すときの処理をする。
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}
