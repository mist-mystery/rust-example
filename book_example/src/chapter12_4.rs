//! minigrep
//! TDD

pub mod minigrep {
    use std::{error::Error, fs};

    pub struct Config<'a> {
        pub query: &'a str,
        pub file_path: &'a str,
    }

    impl Config<'_> {
        pub fn build(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }

            let query = &args[1];
            let file_path = &args[2];

            Ok(Config { query, file_path })
        }
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;
        for line in search(config.query, &contents) {
            println!("{line}");
        }
        Ok(())
    }

    #[allow(dead_code)]
    // 1. コンパイルが通るだけの失敗する関数を用意する。
    // ライフタイムの 'a が contents と戻り値で使用されており、
    // これは、search 関数に返されるデータは、contents 引数で渡されるデータと同期間生きることを示す。
    fn search_fail<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
        vec![]
    }

    // 2. 関数にテストを通過させるコードを書く。
    fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }

        results
    }

    #[allow(dead_code)]
    // 3. イテレータを活用してリファクタ
    fn search_iter<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn one_result() {
            let query = "duct";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.";

            // assert_eq!(vec!["safe, fast, productive."], search_fail(query, contents));
            assert_eq!(vec!["safe, fast, productive."], search(query, contents));
            assert_eq!(
                vec!["safe, fast, productive."],
                search_iter(query, contents)
            )
        }
    }
}
