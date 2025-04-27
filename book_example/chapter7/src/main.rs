// バイナリクレートからライブラリクレートを参照するには、crate:: でなくパッケージ名を先頭に使う。
use chapter7::chapter7_2::garden::vegetables::Asparagus;
use chapter7::chapter7_3;
use chapter7::chapter7_4;
use chapter7::chapter7_5;

fn main() {
    {
        let plant = Asparagus {};
        println!("I'm growing {plant:?}!");
    }
    {
        chapter7_3::restaurant::eat_at_restaurant();
    }
    {
        chapter7_4::restaurant::idiomatic();
        // hosting モジュールを re-export しているため、
        // 実装が置いてある front_of_house は private であるが、hoisting::add_to_waitlist メソッドをここで使える。
        chapter7_4::restaurant::hosting::add_to_waitlist();
    }
    {
        chapter7_5::eat_at_restaurant();
    }
}
