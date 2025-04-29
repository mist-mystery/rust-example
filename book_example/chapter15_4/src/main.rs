use std::{ops::Deref, ptr, rc::Rc};

fn main() {
    {
        #[derive(Clone, Debug, PartialEq)]
        // Box で Cons リストを作ると、複数のリストで要素を共有することはできない。
        enum ListBox<T> {
            Cons(T, Box<ListBox<T>>),
            Nil,
        }

        let b;
        let c;
        {
            let a = ListBox::Cons(5, Box::new(ListBox::Cons(10, Box::new(ListBox::Nil))));
            let a_box = Box::new(a);
            let a_clone = a_box.clone();
            b = ListBox::Cons(3, a_box); // b が a を所有する
            // c = ListBox::Cons(4, a_box); // a はムーブ済みなので c を作ることはできない。
            c = ListBox::Cons(4, a_clone); // Box の中身が Clone を実装していれば clone はできるが、指しているのは別モノ。ディープコピーコストもかかる。
        }

        let ListBox::Cons(i, bbox) = b else { panic!() };
        let ListBox::Cons(j, cbox) = c else { panic!() };
        assert_eq!(i, 3);
        assert_eq!(j, 4);
        assert_eq!(bbox, cbox, "bbox と cbox の中身としては同一");
        assert!(
            !ptr::eq(bbox.deref(), cbox.deref()),
            "bbox と cbox が指すものは違う"
        );
    }
    {
        // 参照とライフタイムを使えば複数の変数で要素の共有はできるが、全要素が少なくともリストの寿命の間は生きなければならなくなる。
        enum ListRef<'a, T> {
            Cons(T, &'a ListRef<'a, T>),
            Nil,
        }

        let b;
        let c;
        {
            let a = ListRef::Cons(5, &ListRef::Cons(10, &ListRef::Nil));
            b = ListRef::Cons(3, &a);
            c = ListRef::Cons(4, &a);

            // a がスコープ内であれば b, c を利用することができる。
            let ListRef::Cons(_i, _b) = b else { panic!() };
            let ListRef::Cons(_j, _b) = c else { panic!() };
        }

        // a（要素の一部）はスコープの外に出ると寿命が切れてしまう。このため a を参照している b, c をスコープの外で使うことはできない。
        // let ListRef::Cons(i, _b) = b else { panic!() };
        // let ListRef::Cons(j, _b) = c else { panic!() };
    }
    {
        // Rc を使うことで、単独の値に複数の所有者を持たせることができる。
        enum ListRc<T> {
            Cons(T, Rc<ListRc<T>>),
            Nil,
        }

        let b;
        let c;
        {
            let a = Rc::new(ListRc::Cons(
                5,
                Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil))),
            ));
            assert_eq!(Rc::strong_count(&a), 1, "参照カウント1");

            // Rc::clone(&a) はデータ自体はクローンせず、参照カウントだけ増やして所有権を保持する。
            b = ListRc::Cons(3, Rc::clone(&a));
            // a.clone() でも Rc::clone(&a) と同じことだが、データのディープコピーをするのでないというのを明示するのに
            // わざと Rc::clone(&a) と書いて視覚的に区別するのが普通。
            c = ListRc::Cons(4, a.clone());

            assert_eq!(Rc::strong_count(&a), 3, "a,b,c が所有している");
        }

        // 変数 a 自体の寿命は切れても、a が指すデータの所有権は b, c も持っているため、a のスコープ外でも使うことができる。
        let ListRc::Cons(_, brc) = b else { panic!() };
        let ListRc::Cons(_, crc) = c else { panic!() };
        assert_eq!(Rc::strong_count(&brc), 2); // a が drop されるときに参照カウントが減っている。
        assert_eq!(Rc::strong_count(&crc), 2);
        assert!(
            ptr::eq(brc.deref(), crc.deref()),
            "brc と crc が指すものは同じ"
        );

        // Rc から値を参照で取り出すには Rc::as_ref(&self) を使う。
        let ListRc::Cons(second, _tail) = brc.as_ref() else {
            panic!()
        };
        assert_eq!(*second, 5);
    }
}
