use std::{env, process};

use book_example::chapter12_3::minigrep::{self, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Err 時は unwrap_or_else で panic! の代わりのエラー処理をクロージャ内に定義する。
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1); // 即座にプログラムを停止させ、渡された数字を終了コードとして返す。
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // Result 型でエラー時の後始末を記述する。
    // if let で Err を返したかどうかを確認し、そうであれば process::exit(1) を呼び出す。
    // Config::new() の unwrap_or_else と異なり、成功時の値に興味がないのでこちらのほうが適切。
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}

#[cfg(test)]
mod tests {
    use book_example::chapter12_3::minigrep::Config;

    // 引数解析器を分離し、Config 構造体を作成して返す関数（Config の関連関数から作成する方がいいため、main では使わない）。
    // 引数が足りなければ None を返す。
    fn parse_config(args: &[String]) -> Option<Config> {
        let query = args.get(1)?;
        let file_path = args.get(2)?;

        Some(Config { query, file_path })
    }

    #[test]
    fn it_works() {
        let args = [
            String::from("target/debug/deps/chapter12_3"),
            String::from("the"),
            String::from("src/public/poem.txt"),
        ];
        let config = parse_config(&args).unwrap();
        assert_eq!(config.query, "the");
        assert_eq!(config.file_path, "src/public/poem.txt");
    }
}
