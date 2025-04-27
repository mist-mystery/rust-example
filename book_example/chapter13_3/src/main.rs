use std::{env, process};

use chapter13_3::minigrep::{self, Config};

// chapter12_6 で Config::build メソッドが String スライスの代わりにイテレータを受け取るように変更。
fn main() {
    // build メソッドは env::args() で生成したイテレータの所有権は奪うが、
    // その中身の String 自体の所有権を奪うわけではない。
    // ただし、env::args() を呼ぶたびに環境から文字列バッファをコピーするため、
    // 一つの Vec が所有権を持ち、その参照を取りまわすよりはコストがかかる。
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // env::args() を再度呼び出せば、コマンドライン引数を再び String で取り出すことが可能。
    let mut args = env::args();
    assert_eq!(args.next(), Some(String::from("target/debug/chapter13_3")));

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
