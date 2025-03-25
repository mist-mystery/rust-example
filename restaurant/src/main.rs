// `mod モジュール名` の後に {} でなく ; を使うと、モジュール名と同名のファイルから中身を読み込む。
// 他の言語の "include" とは異なる。mod 単体では任意のパスのモジュールを読み込むこともできない。
mod appetizer_enum;
mod breakfast_struct;
mod front_of_house;
mod super_mod;
mod r#use;

fn main() {
    // super
    super_mod::back_of_house::fix_incorrect_order();

    // 構造体と列挙体
    breakfast_struct::eat_at_restaurant();
    appetizer_enum::eat_at_restaurant();

    r#use::idiomatic();
    // use モジュールの hosting 関数を re-export しているためここで使える
    r#use::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}
