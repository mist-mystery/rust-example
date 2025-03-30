// `mod モジュール名` の後に {} でなく ; を使うと、モジュール名と同名のファイルから中身を読み込む。
// 今のパスが crate::chapter7_5 であるから、`mod front_of_house` は src/chapter7_5/front_of_house.rs から読み込むことになる。
// 読み込むことができるのは子孫のパスに限られる。兄弟でもダメ（クレートルートの場合は crate がパスなので、例外的にディレクトリ構造的に兄弟の位置にあるファイルを読み込める）。
//   ⇒ 他の言語の "include" とは異なり、mod 単体では任意のパスのモジュールを読み込むことはできない。
//   ※ mod.rs という特別なファイル名を使う方法もあるが、古い方法のようなので考えない。
mod front_of_house;

// src/chapter7_5/back_of_house/kitchen.rs を読み込むのに、
// src/chapter7_5/back_of_house.rs を経由するのでなく直接読み込むには、以下のようにネストが必要になる。
mod back_of_house {
    pub mod kitchen;
}

use back_of_house::kitchen;
// mod front_of_house がないと use できない（順番は前後しても問題ない）。
use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    kitchen::cook();
}
