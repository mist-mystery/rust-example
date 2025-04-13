use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

// main 関数でも Result 型を返すようにすれば?演算子を使うことも可能。
fn main() -> Result<(), io::Error> {
    open_file();
    println!("{}", read_username_from_file().unwrap());
    println!("{}", read_username_from_file_try().unwrap());
    println!("{}", fs_read_username()?);
    Ok(())
}

fn open_file() -> File {
    let greeting_file_result = File::open("hello.txt");

    // match 式でファイルハンドルもしくはエラーインスタンスを取得。
    match greeting_file_result {
        Ok(file) => file,
        // kind メソッドで ErrorKind を取得し、match 式でファイルが存在しない場合の処理を分岐
        Err(error) => match error.kind() {
            // ファイルが存在しない場合は、ファイルを作成する
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    }
}

// Result 型を返すことで、エラー時の処理を呼び出し元に任せる（エラーの委譲）。
// 今回は、File::open も read_to_string も Err の中身が io::Error なので、返り値の失敗の型をそれにしている。
fn read_username_from_file() -> Result<String, io::Error> {
    // cargo run で実行するならカレントディレクトリはプロジェクトルートとなる。
    let username_file_result = File::open("src/public/username.txt");

    let mut username_file = username_file_result?;

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// ?演算子を使って上の関数を簡潔に。
// Result の値が Err であれば関数の返り値がその Err となって早期 return される。
// 関数末尾で Ok の値を返す。
// ?演算子を使うには、返り値が Result 型である、全てのエラー型が一つのエラー型で表現できる（または From トレイトを実装している）必要がある。
fn read_username_from_file_try() -> Result<String, io::Error> {
    let mut username = String::new();
    // ?の後にメソッド呼び出しを連結できる。
    File::open("src/public/username.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// 実用的には、ファイルから文字列を読み込むだけなら?演算子も可変変数を使う必要もなく、ワンライナーで完結する。
fn fs_read_username() -> Result<String, io::Error> {
    fs::read_to_string("src/public/username.txt")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    #[should_panic = "No such file or directory"]
    // 失敗したら panic させるだけでいいなら unwrap が便利。
    fn open_file_unwrap() {
        let _f = File::open("hello_unwrap.txt").unwrap();
    }

    #[test]
    #[should_panic = "Failed to open hello_expect.txt"]
    // 失敗したら panic させ、メッセージを表示。
    fn open_file_expect() {
        let _f = File::open("hello_expect.txt").expect("Failed to open hello_expect.txt");
    }

    #[test]
    fn return_result() {
        let username = read_username_from_file().unwrap();
        assert_eq!(username, "Alice");

        let username = read_username_from_file_try().unwrap();
        assert_eq!(username, "Alice");

        let username = fs_read_username().unwrap();
        assert_eq!(username, "Alice");
    }
}
