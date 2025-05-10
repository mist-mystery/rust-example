use rand::Rng;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
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

enum Coin {
    Penny,
    // Nickel,
    // Dime,
    Quarter(UsState),
}

fn main() {
    {
        // match 式を使う場合
        let config_max = Some(3u8);
        #[allow(clippy::single_match)] // 説明用のため if let でなくこちらを記述
        match config_max {
            Some(max) => println!("The maximum is configured to be {max}"),
            _ => (),
        }

        // `if let` を使うと値が一つのパターンにマッチしたときにコードを走らせ、他は無視する。
        // 上のような match の糖衣構文と考えることができる。
        // 短く書けるが、強制的な包括性チェック（exhaustive checking）を失う。
        if let Some(max) = config_max {
            println!("The maximum is configured to be {max}");
        }
    }
    {
        // while-let

        // loop と match で条件を満たさなくなるまで処理を継続
        let mut optional = Some(0);
        #[allow(clippy::while_let_loop)] // 説明用のため while let でなくこちらを記述
        loop {
            match optional {
                Some(i) => {
                    if i > 9 {
                        println!("Greater than 9, quit!");
                        optional = None;
                    } else {
                        println!("`i` is `{:?}`. Try again.", i);
                        optional = Some(i + 1);
                    }
                }
                _ => {
                    break;
                }
            }
        }

        // while-let を使って上のループと同じことを実行。
        optional = Some(0);
        while let Some(i) = optional {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
        }
    }
    {
        // coin が Coin::Quarter 以外であれば _ のアームが実行される
        let coin = Coin::Penny;
        let mut count = 0;
        match &coin {
            // そのままだと束縛した値がムーブされてしまうため、coin は参照を取る必要がある。
            Coin::Quarter(state) => println!("State quarter from {state:?}!"),
            _ => count += 1,
        }
        assert_eq!(count, 1);

        // if let に else アームを加えることで、上記の match 式と同様にマッチしなかった場合の処理を記述できる。
        // state は最初のアーム内でのみ使用可能。
        if let Coin::Quarter(state) = &coin {
            println!("State quarter from {state:?}!");
        } else {
            count += 1;
        }
        assert_eq!(count, 2);
    }
    {
        let coin = Coin::Quarter(if rand::rng().random_range(0..=1) > 0 {
            UsState::Alaska
        } else {
            UsState::Alabama
        });
        if let Some(msg) = describe_state_quarter(&coin) {
            println!("nested: {msg}");
        }

        if let Some(msg) = describe_state_quarter_early(&coin) {
            println!("early : {msg}");
        }

        if let Some(msg) = describe_state_quarter_happy(&coin) {
            println!("happy : {msg}");
        }
    }
}

// coin が Coin::Quater(state) で、さらに state に応じて場合分けする。素直に書くとネストが深くなる。
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

// 早期 return
fn describe_state_quarter_early(coin: &Coin) -> Option<String> {
    // if let でパターンにマッチした値 state を戻り値として、ローカル変数 coin_state に束縛する。
    let coin_state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if coin_state.existed_in(1900) {
        Some(format!("{coin_state:?} is pretty old, for America!"))
    } else {
        Some(format!("{coin_state:?} is relatively new."))
    }
}

// let-else を利用
fn describe_state_quarter_happy(coin: &Coin) -> Option<String> {
    // let-else シンタックスで、パターンの値が外部スコープに束縛される。
    // パターンにマッチしなければ else アームが実行されて早期 return される。
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
