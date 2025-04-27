mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 相対パスでモジュールをスコープに持ち込む。関数は慣例的に親モジュールを use で持ち込む。
// use の前に pub をつければ re-exporting となる。
pub use front_of_house::hosting;
// 構造体や enum を use で持ち込むときは、フルパスで書くのが慣例的。
// as でエイリアスを指定できる。
// {}を使って複数のクレートやモジュールをスコープに取り込むことができる。
use std::{collections::HashMap, fmt::Result, io::Result as IoResult};

pub fn idiomatic() {
    hosting::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, 2);

    let _ = function1();
    let _ = function2();
}

mod customer {
    pub fn _eat_at_restaurant() {
        // use と異なるスコープにあるためコンパイルエラーになる
        // hosting::add_to_waitlist();

        super::hosting::add_to_waitlist();
    }
}

// 同じスコープに io::Result があるので、fmt::Result としてもいい。
fn function1() -> Result {
    Ok(())
}

// エイリアスを使わず、io::Result としてもいい。
fn function2() -> IoResult<()> {
    Ok(())
}
