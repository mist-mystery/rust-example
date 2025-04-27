//! Rust 2018 Edition から、extern crate はほぼ必要なくなった。
//! @see https://doc.rust-jp.rs/edition-guide/rust-2018/path-changes.html
// extern crate chapter11;

use chapter11::chapter11_3;

// テストで使う共通部分を切り出す。
// このファイルがテスト対象ファイルと認識されてしまわないように、ディレクトリを切って mod.rs の名前でファイルを作成する。
mod common;

// 若干記述量が増えるが、こっちだと mod.rs の名前に拘る必要がなくなる。
mod common_new {
    pub mod setup;
}

#[test]
fn it_adds_two() {
    common::setup();
    common_new::setup::dummy();
    assert_eq!(chapter11_3::add_two(2), 4);
}
