/// Box<T> はヒープにデータを確保する。
fn box_use() {
    let b = Box::new(5);
    println!("b = {b}");
    assert_eq!(
        *b, 5,
        "Box は参照のようにデリファレンスできる（Derefトレイトを実装している）"
    );
}

#[derive(Debug)]
// 列挙子が Cons(i32, List) だと再帰的な型となり、サイズが無限になってしまう。
// Cons(i32, Box<List>) とする（List の代わりにリストのポインタを保持する）ことで、List が必要とするサイズを決定できる。
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn cons_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{list:?}");

    let Cons(i, list) = list else {
        panic!();
    };
    assert_eq!(i, 1);
    println!("{list:?}");
}

fn main() {
    box_use();
    cons_list();
}
