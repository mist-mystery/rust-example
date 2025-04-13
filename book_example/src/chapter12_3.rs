pub mod minigrep {
    use std::{error::Error, fs};

    // 引数値の構造体
    pub struct Config<'a> {
        pub query: &'a str,
        pub file_path: &'a str,
    }

    impl Config<'_> {
        // Config インスタンスを作成するのが目的であれば、関数より Config の関連関数で作成するほうがよい。
        // 関連関数名は Config を返すのであれば new のほうがいいが、new だと失敗しないことを期待されがちであるため、
        // Result 型を返すのであれば build などの名前の方が適切。
        // 引数が足りなければ Err を返す。
        pub fn build(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }

            let query = &args[1];
            let file_path = &args[2];

            Ok(Config { query, file_path })
        }
    }

    // 戻り値のエラー型としてトレイトオブジェクトを使用。
    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;
        println!("With text:\n{contents}");
        Ok(()) // 成功型の () は、関数を副作用のためだけに呼び出しているという慣習的な方法
    }
}
