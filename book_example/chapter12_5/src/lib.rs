//! 環境変数で case insensitive な検索を可能に

pub mod minigrep {
    use std::{env, error::Error, fs};

    pub struct Config<'a> {
        pub query: &'a str,
        pub file_path: &'a str,
        pub case_sensitive: bool, // 追加
    }

    impl Config<'_> {
        pub fn build(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }

            let query = &args[1];
            let file_path = &args[2];
            // （引数でなく）CASE_INSENSITIVE 環境変数がセットされていれば true, いなければ false を返す。
            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

            Ok(Config {
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
            search(config.query, &contents)
        } else {
            search_case_insensitive(config.query, &contents)
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

    // 大文字小文字を無視して検索する。
    fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn case_sensitive() {
            let query = "duct";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

            assert_eq!(vec!["safe, fast, productive."], search(query, contents));
        }

        #[test]
        fn case_insensitive() {
            let query = "rUsT";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

            assert_eq!(
                vec!["Rust:", "Trust me."],
                search_case_insensitive(query, contents)
            );
        }
    }
}
