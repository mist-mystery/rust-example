mod back_of_house {
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
    }

    pub fn meal_fruit() -> String {
        let mut meal = Breakfast::summer("Onigiri");
        meal.seasonal_fruit = String::from("Mikan"); // 兄弟の構造体の private フィールドは読み書きできる
        meal.seasonal_fruit // private となっているフィールド返すことは当然できる
    }
}

pub fn eat_at_restaurant() {
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

    let fruit = back_of_house::meal_fruit();
    println!("I'd like {} fruit please", fruit);
}
