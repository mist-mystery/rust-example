mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 相対パスでモジュールをスコープに持ち込む。
// use の前に pub をつければ re-exporting となる。
pub use front_of_house::hosting;
// as でエイリアスを指定できる。
use std::collections::HashMap as CollectionsHashMap;

pub fn idiomatic() {
    // use でモジュールをスコープに持ち込む。関数は慣例的に親モジュールを use で持ち込む。
    hosting::add_to_waitlist();

    // 構造体や enum を use で持ち込むときは、フルパスで書くのが慣例的。
    let mut map = CollectionsHashMap::new();
    map.insert(1, 2);
}

// 7-12
mod customer {
    pub fn _eat_at_restaurant() {
        // use と異なるスコープにあるためコンパイルエラーになる
        // hosting::add_to_waitlist();

        super::hosting::add_to_waitlist();
    }
}
