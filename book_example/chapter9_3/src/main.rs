//! 基本的に panic より Result を返す方がいいが、以下の場面では panic を検討してもいい。
//! ## 例、プロトタイプ、テストコード
//! - 例においては、unwrap 等を使って本質だけを記載してエラーハンドリングを省略するほうがいい。
//!   アプリケーションコードでは unwrap 等を使っている場所を適切なエラーハンドリングに書き換える。
//! - プロトタイプではとりあえず正常系だけ作るのには unwrap 等が便利。これも後で適切なエラーハンドリングに置き換える。
//! - テストでは期待する結果を得られなかった場合に失敗するのが合理的であることから、unwrap 等が適切。
//! ## コンパイラが把握できない部分で Err にならない確信がある場合
//! - この場合 expect でエラーになり得ない理由を記載しておく。
//! ## 処理を続けることが有害である場合
//! - 境界外メモリアクセスの発生等

use std::net::{IpAddr, Ipv4Addr};

fn main() {
    legal_ipv4();
    guessing_game::guarantee_guess();
}

// 127.0.0.1 は合法なIPアドレスと分かっているので Err にならないと確信できる。
// こういう場面では unwrap メソッドを利用すると便利。
// 逆に、ユーザー入力文字列をパースする場合などは失敗する可能性があるため、関数からは Result 型を返した方がいい。
fn legal_ipv4() -> IpAddr {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    let ipv4 = Ipv4Addr::new(127, 0, 0, 1);
    assert_eq!(home, ipv4);
    home
}

mod guessing_game {
    use std::io;

    // フィールドは private にしておく
    struct Guess {
        value: i32,
    }

    // 値が1～100までの制約を付けたインスタンス。
    // Guess の value フィールドは非公開で、関連関数 new を通じてのみ Guess インスタンスを生成できる。
    // また、value メソッド（getter）を通じてのみ取得可能。
    // こうすることで、確実に Guess インスタンスの value は 1～100までという保証をしている。
    impl Guess {
        pub fn new(value: i32) -> Self {
            if !(1..=100).contains(&value) {
                panic!("Guess value must be between 1 and 100, got {value}.");
            }

            Self { value }
        }

        /// getter
        pub fn value(&self) -> i32 {
            self.value
        }
    }

    pub fn guarantee_guess() {
        println!("Please input value between 1 and 100.");
        let mut guess = String::new();
        // read_line が返す Result は実は処理しなくてもエラーにはならないが、guess に何が入っているかの保証がない。
        io::stdin().read_line(&mut guess).unwrap();
        let guess = Guess::new(guess.trim().parse::<i32>().unwrap());
        println!("input value is {}.", guess.value());
    }
}
