
mod back_of_house {
    // enum を公開するとその列挙子は全て公開される
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() -> String {
    // 列挙子は全て公開される
    let order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
    match order1 {
        back_of_house::Appetizer::Soup => String::from("Soup"),
        back_of_house::Appetizer::Salad => String::from("Salad"),
    }
}
