//! match 式
//! - match はパターンマッチが強力である一方 primitive であり、冗長かつ多義的になりがちである。
//!   Option や Result など既存の enum の多くには便利なメソッドが備わっており、基本的にはそちらを使用する方がいい。

fn main() {}

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    enum UsState {
        #[allow(dead_code)]
        Alabama,
        Alaska,
        // ... などなど
    }

    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    #[test]
    fn use_value_in_cents() {
        assert_eq!(value_in_cents(Coin::Quarter(UsState::Alaska)), 25);

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
    }

    #[test]
    fn option() {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        assert_eq!(six, Some(6));
        assert_eq!(none, None);
        assert_eq!(plus_one_operator(five), Some(6));
        assert_eq!(plus_one_operator(none), None);

        fn plus_one(x: Option<i32>) -> Option<i32> {
            // match は全可能性を網羅する必要がある
            // match x {
            //     None => None, // 仮にこの行をコメントアウトすると non-exhaustive patterns のコンパイルエラー
            //     Some(i) => Some(i + 1),
            // }

            // このケースであればこちらの方が簡単
            x.map(|i| i + 1)
        }

        // ? Operator を使って同じことを簡潔にできる（chapter9_2）
        fn plus_one_operator(x: Option<i32>) -> Option<i32> {
            Some(x? + 1)
        }
    }

    #[test]
    fn catch_all() {
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
