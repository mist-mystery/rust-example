use std::{error::Error, fs::File};

/// main 関数でも Result 型を返すようにすれば?演算子を使うことも可能。(main 関数は std::process::Termination トレイトを実装する型を返すことが可能)
/// エラーの型の Box<dyn Error> は、任意の種類のエラーを表すトレイトオブジェクト(chapter18_2)を使用。
fn main() -> Result<(), Box<dyn Error>> {
    let _greeting_file = File::open("hello.txt")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::{self, ErrorKind, Read},
    };

    /// ファイルを開く。ファイルがなければ作成する。Result に対して match 式を使用。
    fn open_file_match(path: &str) -> File {
        let greeting_file_result = File::open(path);

        // match 式でファイルハンドルもしくはエラーインスタンスを取得。
        match greeting_file_result {
            Ok(file) => file,
            // kind メソッドで ErrorKind を取得し、match 式でファイルが存在しない場合の処理を分岐
            Err(error) => match error.kind() {
                // ファイルが存在しない場合は、ファイルを作成する
                ErrorKind::NotFound => match File::create(path) {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {e:?}"),
                },
                other_error => {
                    panic!("Problem opening the file: {other_error:?}");
                }
            },
        }
    }

    /// ファイルを開く。ファイルがなければ作成する。Result に対して unwrap_or_else を使うことでより簡潔に。
    fn open_file_method(path: &str) -> File {
        File::open(path).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(path).unwrap_or_else(|err| {
                    panic!("Problem creating the file: {err:?}");
                })
            } else {
                panic!("Problem opening the file: {error:?}");
            }
        })
    }

    #[test]
    fn open_file() -> Result<(), io::Error> {
        open_file_match("hello.txt");
        fs::remove_file("hello.txt")?;
        open_file_method("hello.txt");
        fs::remove_file("hello.txt")?;
        Ok(())
    }

    #[test]
    #[should_panic = "No such file or directory"]
    /// 失敗したら panic させるだけでいいなら unwrap が便利。
    fn open_file_unwrap() {
        let _f = File::open("hello_unwrap.txt").unwrap();
    }

    #[test]
    #[should_panic = "Failed to open hello_expect.txt"]
    /// 失敗したら panic させ、メッセージを表示。
    fn open_file_expect() {
        let _f = File::open("hello_expect.txt").expect("Failed to open hello_expect.txt");
    }

    /// Result 型を返すことで、エラー時の処理を呼び出し元に任せる（エラーの委譲）。
    /// 今回は、File::open も read_to_string も Err の中身が io::Error なので、戻り値の失敗の型をそれにしている。
    fn read_username_from_file(path: &str) -> Result<String, io::Error> {
        // cargo test で実行するならカレントディレクトリは chapter9_2 ディレクトリとなる。
        let username_file_result = File::open(path);

        #[allow(clippy::question_mark)]
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_size) => Ok(username),
            Err(e) => Err(e),
        }
    }

    /// ?演算子を使って上の関数のボイラープレートを減らして簡潔に。
    /// Result の値が Err であれば関数の戻り値がその Err となって早期 return される。
    /// 関数末尾で Ok の値を返す。
    /// ?演算子を使うには、戻り値が Result 型である、全てのエラー型が一つのエラー型で表現できる（または From トレイトを実装している）必要がある。
    fn read_username_from_file_try(path: &str) -> Result<String, io::Error> {
        let mut username = String::new();
        // ?の後にメソッド呼び出しを連結できる。
        File::open(path)?.read_to_string(&mut username)?;

        Ok(username)
    }

    // 実用的には、ファイルから文字列を読み込むだけなら?演算子も可変変数を使う必要もなく、ワンライナーで完結する。
    fn read_username_with_fs(path: &str) -> Result<String, io::Error> {
        fs::read_to_string(path)
    }

    #[test]
    fn return_result() {
        let username = read_username_from_file("username.txt").unwrap();
        assert_eq!(username, "Alice");

        let username = read_username_from_file_try("username.txt").unwrap();
        assert_eq!(username, "Alice");

        let username = read_username_with_fs("username.txt").unwrap();
        assert_eq!(username, "Alice");
    }

    #[test]
    fn option_try() {
        // Result と同様に、Option でも ? 演算子を使用することができる。クロージャでも使える。
        let last_char_of_first_line = |text: &str| text.lines().next()?.chars().last();
        assert_eq!(last_char_of_first_line("hello\nworld"), Some('o'));
        assert_eq!(last_char_of_first_line("\nhi"), None);
    }
}
