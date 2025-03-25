fn serve_order() {
    println!("serve order");
}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        serve_order();
        super::serve_order(); // 親（ルート）の serve_order() を呼び出す
    }

    fn serve_order() {
        println!("serve order to back of house");
    }
}
