use std::{
    io,
    net::{IpAddr, Ipv4Addr},
};

fn main() {
    legal_ipv4();
    guarantee_guess();
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

pub struct Guess {
    value: u32,
}

// 値が1～100までの制約を付けたインスタンス。
// Guess の value フィールドは非公開で、関連関数 new を通じてのみ Guess インスタンスを生成できる。
// また、value メソッド（getter）を通じてのみ取得可能。
// こうすることで、確実に Guess インスタンスの value は 1～100までという保証をしている。
impl Guess {
    pub fn new(value: u32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Self { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn guarantee_guess() {
    println!("Please input value between 1 and 100.");
    let mut guess = String::new();
    // read_line が返す Result は実は処理しなくてもエラーにはならないが、guess に何が入っているかの保証がない。
    io::stdin().read_line(&mut guess).unwrap();
    let guess = Guess::new(guess.trim().parse::<u32>().unwrap());
    println!("input value is {}.", guess.value);
}
