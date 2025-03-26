fn main() {
    define();
    match_control();
    if_let();
}

enum IpAddrKind {
    V4,
    V6,
}

// 6.1
fn define() {
    {
        // struct を使って IpAddrKind の列挙子を保持する。
        struct IpAddrStruct {
            kind: IpAddrKind,
            address: String,
        }

        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
        route(four);
        route(six);

        let home = IpAddrStruct {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };
        let _loopback = IpAddrStruct {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
        if let IpAddrKind::V4 = home.kind {
            println!("home.address = \"{}\"", home.address);
        }
    }
    {
        // 列挙子に直接値を紐づけることができるため、IpAddrStruct のような構造体は必要ない。
        // また、列挙子にそれぞれ異なる型を持たせることができる。
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        // enum だけを使って IP アドレスを保持する。
        // V4とV6のアドレスの型が違っていてもいい。
        let home = IpAddr::V4(127, 0, 0, 1);
        let _loopback = IpAddr::V6(String::from("::1"));

        match home {
            IpAddr::V4(a, b, c, d) => println!("{a}.{b}.{c}.{d}"),
            IpAddr::V6(ip) => println!("{}", ip),
        }

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        // enum にメソッドや関連関数を定義できる
        impl Message {
            fn call(&self) {
                match self {
                    Self::Quit => (),
                    Self::Move { x, y } => println!("Move to ({x}, {y})"),
                    Self::Write(str) => println!("Write \"{str}\""),
                    Self::ChangeColor(r, g, b) => println!("rgb({r}, {g}, {b})"),
                }
            }
            fn new() {}
        }

        let _m = Message::Quit;
        let _m = Message::Move { x: 0, y: 1 };
        let _m = Message::ChangeColor(0, 0, 0);
        let m = Message::Write(String::from("hello"));
        m.call();
        Message::new();
    }
    {
        let some_number = Some(5);
        let _some_char = Some('e');
        let _absent_number: Option<i32> = None; // None を使うときは Option<T> の型が何になるか明示する必要がある
        let x = 5;
        // let sum = x + some_number; // i32 と Option<i32> が異なる型であるため足し合わせることはできない
        if let Some(n) = some_number {
            println!("{}", n + x);
        }
    }
}

fn route(_ip_type: IpAddrKind) {}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// 6.2
fn match_control() {
    {
        value_in_cents(Coin::Quarter(UsState::Alaska));
        let _c = Coin::Penny;
        let _c = Coin::Nickel;
        let _c = Coin::Dime;
        let _c = Coin::Quarter(UsState::Alabama);
    }
    {
        let five = Some(5);
        let _six = plus_one(five);
        let _none = plus_one(None);
    }
    {
        let dice_roll = 9;
        match dice_roll {
            3 => println!("add_fancy_hat"),
            7 => println!("remove_fancy_hat"),
            other => println!("move_player: {other}"), // 3,7以外の値の場合に実行される
        }
        match dice_roll {
            3 => println!("add_fancy_hat"),
            7 => println!("remove_fancy_hat"),
            _ => (), // _ はプレースホルダーで、値が不要な場合の catch-all パターンで用いる
        }
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // enum の列挙子から値を取り出すことができる
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match は全可能性を網羅する必要がある
    match x {
        None => None, // 仮にこの行をコメントアウトすると non-exhaustive patterns のコンパイルエラー
        Some(i) => Some(i + 1),
    }
}

// 6.3
fn if_let() {
    {
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("The maximum is configured to be {max}"),
            _ => (),
        }

        // `if let` を使うと短く書ける。
        if let Some(max) = config_max {
            println!("The maximum is configured to be {max}");
        }
    }
    {
        let coin = Coin::Penny;
        let mut count = 0;
        match &coin {
            // そのままだと束縛した値がムーブされてしまうため、coin は参照を取る
            Coin::Quarter(state) => println!("State quarter from {state:?}!"),
            _ => count += 1,
        }

        // else も使用可。state は if の中でのみ使用可能
        if let Coin::Quarter(state) = &coin {
            println!("State quarter from {state:?}!");
        } else {
            count += 1;
        }
        println!("count = {count}");

        let coin = Coin::Quarter(UsState::Alaska);
        if let Some(msg) = describe_state_quarter(&coin) {
            println!("{msg}");
        }
        let coin = Coin::Quarter(UsState::Alabama);
        if let Some(msg) = describe_state_quarter_happy(&coin) {
            println!("{msg}");
        }
    }
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

// coin が Coin::Quater(state) で、state に応じて場合分けする。素直に書くとネストが深くなる
fn describe_state_quarter(coin: &Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

// TODO: while let のマッチングについても記載

// 早期 return を利用
fn describe_state_quarter_happy(coin: &Coin) -> Option<String> {
    // state をローカル変数として取得
    let _state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    // let-else シンタックスでより簡潔に state を取得できる
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
