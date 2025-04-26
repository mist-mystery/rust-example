// chapter12_5 の実装を少し変更したもの。
pub mod minigrep {
    use std::{env, error::Error, fs};

    // chapter12_5 と異なり、query と file_path の所有権は構造体が持つようにする。
    pub struct Config {
        pub query: String,
        pub file_path: String,
        pub case_sensitive: bool,
    }

    impl Config {
        // イテレータを引数で受け取るように変更。
        pub fn build(mut args: env::Args) -> Result<Self, &'static str> {
            args.next();

            // let query = match args.next() {
            //     Some(arg) => arg,
            //     None => return Err("Didn't get a query string"),
            // };

            // ↑よりこっちのほうが簡潔
            let Some(query) = args.next() else {
                return Err("Didn't get a query string");
            };

            let Some(file_path) = args.next() else {
                return Err("Didn't get a file name");
            };

            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

            Ok(Self {
                query,
                file_path,
                case_sensitive,
            })
        }
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;

        // config.case_sensitive によって search か search_case_insensitive を呼ぶか分ける。
        let results = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };

        for line in results {
            println!("{line}");
        }
        Ok(())
    }

    fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }

    #[allow(dead_code)]
    // イテレータでなく for を使った実装。
    // パフォーマンス的には、ゼロコスト抽象化によりイテレータとクロージャで抽象化したものとほぼ変わらない。
    fn search_for<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }

        results
    }

    // 大文字小文字を無視して検索する。
    fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }
}
