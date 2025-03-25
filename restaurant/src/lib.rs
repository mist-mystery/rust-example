mod front_of_house {
    pub mod hosting {
        // add_to_waitlist を front_of_house の外で呼ぶなら、
        // hosting も add_to_waitlist も公開していなければならない
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute Path
    // front_of_house がクレートルートに定義されているため絶対パスで呼べるが、
    // このファイル(lib.rs)がクレートルートでないならコンパイルエラーになる。
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist();
}
