//! Box はスマートポインタの一種で、ヒープに値を確保し、（参照と異なり）その値の所有権を持つ。
//! 実態としては値のポインタをタプル構造体に保持している。
//!
//! - temporary lifetime extension

/// Box<T> はヒープにデータを確保する。
fn box_use() {
    {
        let b1 = Box::new(5);
        println!("b = {b1}");

        // Box は Copy を実装しておらず、同じ中身を指す Box を複数作ることはできない。
        let b2 = b1; // ムーブ
        // assert_eq!(*b1, 5); // borrow of moved value: `b1`
        assert_eq!(*b2, 5);

        #[derive(Debug, PartialEq)]
        struct MyStruct;
        let b3 = Box::new(MyStruct);
        // Box<T> は T が Copy を実装していない型の場合、デリファレンスによりムーブが発生するという特殊な挙動をする。
        // https://doc.rust-lang.org/reference/expressions.html#r-expr.move.movable-place
        let ref_b1 = *b3;
        assert_eq!(ref_b1, MyStruct);
        // assert_eq!(b3, Box::new(MyStruct)); // borrow of moved value: `b3`
    }

    {
        // 参照型を clone すると、参照元の値が clone され新しい値が生まれる。
        #[derive(Clone)]
        struct Point<T>(T);
        let mut p = Point(1);

        let r1 = &mut p;
        let r2 = r1.clone(); // r2 は r1(の参照先) とは独立した新しい値
        r1.0 = 10;

        assert_eq!(r1.0, 10);
        assert_eq!(p.0, 10, "r1.0 の変更が参照元の値にも反映される。");
        assert_eq!(r2.0, 1, "r2 は変更の影響を受けない。");
    }
    {
        // Box を clone すると、その中身の値が clone され新しい値が生まれる。
        #[derive(Clone)]
        struct Point<T>(T);
        let p = Point(1);

        let mut b1 = Box::new(p); // p をムーブ (Point は Copy を実装していないため)
        let b2 = b1.clone(); // clone メソッドを使うには、Box の中身が Clone を実装する必要がある。
        b1.0 = 10;
        let bm = b1; // b1 をムーブ

        // assert_eq!(p.0, 10, "p はムーブされているため使用不可。");
        assert_eq!(bm.0, 10);
        assert_eq!(b2.0, 1, "b1 の変更の影響を受けない。");
    }
}

#[derive(Debug)]
/// 列挙子が Cons(T, List) だと再帰的な型となり、サイズが無限になってしまう。
/// Cons(T, Box<List>) とする（List の代わりにリストのポインタを保持する）ことで、List が必要とするサイズを決定できる。
enum ListBox<T> {
    Cons(T, Box<ListBox<T>>),
    Nil,
}

#[derive(Debug)]
/// Box でなく参照とライフタイムを使用する例。
enum ListRef<'a, T> {
    Cons(T, &'a ListRef<'a, T>),
    Nil,
}

fn cons_list() {
    {
        let list = ListBox::Cons(
            1,
            Box::new(ListBox::Cons(
                2,
                Box::new(ListBox::Cons(3, Box::new(ListBox::Nil))),
            )),
        );
        println!("{list:?}");

        let ListBox::Cons(i, list) = list else {
            panic!();
        };
        assert_eq!(i, 1);
        println!("{list:?}");
    }
    {
        // `&ListRef::Nil` などを変数に持たない場合、普通に考えれば参照元が即座に解放されそうであるが、そうはならない。
        // これができるのは temporary lifetime extension のおかげ？
        let list = ListRef::Cons(1, &ListRef::Cons(2, &ListRef::Cons(3, &ListRef::Nil)));
        println!("{list:?}");

        let ListRef::Cons(i, list) = list else {
            panic!();
        };
        assert_eq!(i, 1);
        println!("{list:?}");
    }
    {
        let list = make(3, &ListRef::Nil);

        // temporary lifetime extension は文の終わりまで生存するルールである。
        // temporary value を関数の引数として渡し、それを使った新しい値を返すようなケースではコンパイルエラーになる。
        // temporary value is freed at the end of this statement.
        // let list = make(2, &make(3, &ListRef::Nil));

        println!("{list:?}");
    }
}

fn make<'a, T>(v: T, l: &'a ListRef<'a, T>) -> ListRef<'a, T> {
    ListRef::Cons(v, l)
}

fn main() {
    box_use();
    cons_list();
}
