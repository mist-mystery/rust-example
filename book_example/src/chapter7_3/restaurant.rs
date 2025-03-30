// eat_at_restaurant メソッドからは front_of_house モジュールまでは見える（∵兄弟）が、
// その子孫は明示的に pub にしないとアクセス不可。
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        // super で親（ルート）の deliver_order() を呼び出す。
        // 子から親は public でなくてもアクセス可能。
        super::deliver_order();
    }

    fn cook_order() {}

    // 構造体はフィールドごとに公開か否かを決められる
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        // getter を定義
        pub fn seasonal_fruit(&self) -> &String {
            &self.seasonal_fruit
        }
    }

    // enum を公開するとその列挙子は全て公開される
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    {
        // Absolute path
        crate::chapter7_3::restaurant::front_of_house::hosting::add_to_waitlist();

        // Relative path
        front_of_house::hosting::add_to_waitlist();

        back_of_house::fix_incorrect_order();
    }
    {
        // 関連関数を使って Breakfast インスタンスを作成
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // toast フィールドは公開されているので読み書きできる
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
        // private なフィールドは読み書きが禁止されている
        // meal.seasonal_fruit = String::from("blueberries");
        // println!("I'd like {} fruit please", meal.seasonal_fruit);

        // private なフィールドがある構造体は関連関数を使わないと作成できない
        // back_of_house::Breakfast {
        //     toast: String::from("Rye"),
        //     seasonal_fruit: String::from("Banana"),
        // };

        let fruit = meal.seasonal_fruit();
        assert_eq!(fruit, "peaches");
    }
    {
        // 列挙子は全て公開されている
        let _order1 = back_of_house::Appetizer::Soup;
        let _order2 = back_of_house::Appetizer::Salad;
    }
}

fn deliver_order() {}
