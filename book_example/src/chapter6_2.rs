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

pub fn main() {
    {
        value_in_cents(Coin::Quarter(UsState::Alaska));
        let _c = Coin::Penny;
        let _c = Coin::Nickel;
        let _c = Coin::Dime;
        let _c = Coin::Quarter(UsState::Alabama);
    }
    {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        assert_eq!(six, Some(6));
        assert_eq!(none, None);
        assert_eq!(plus_one_operator(five), Some(6));
        assert_eq!(plus_one_operator(none), None);
    }
    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other), // 3,7以外の値の場合に実行される
        }
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => (), // _ はプレースホルダーで、値が不要な場合の catch-all パターンで用いる
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn move_player(_num_spaces: u8) {}
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    // match 式は if と違い、条件が論理値に評価されなくても問題ない
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

// ? Operator を使って同じことを簡潔にできる（chapter9_2）
fn plus_one_operator(x: Option<i32>) -> Option<i32> {
    Some(x? + 1)
}
