//! Rc は参照カウント(Reference Count)ポインタ。
//! 値をヒープに確保し、その所有権を共有できる。
//! 内部的には値の実体及び強参照カウンタ、弱参照カウンタをヒープ上に確保している。

/// Cons リストとイミュータブルスタックを参照を駆使して実装。
/// 参照とライフタイムを使えば複数の変数で要素の共有はできるが、全要素が少なくともリストの寿命の間は生きなければならなくなる。
/// また、構造体自体が tail の所有権を持たなくなるため、構造体の外で変数に保持しなければならない（temporary lifetime extension により不要な場合もある）。
mod ref_scope {
    #[derive(Debug, PartialEq)]
    enum ListRef<'a, T> {
        Cons(T, &'a ListRef<'a, T>),
        Nil,
    }

    struct Stack<'a, T>(Option<(T, &'a Stack<'a, T>)>);

    #[derive(Debug, PartialEq)]
    struct Wrap<T>(T);

    fn listref() {
        let b;
        let c;
        {
            let a = ListRef::Cons(Wrap(5), &ListRef::Cons(Wrap(10), &ListRef::Nil));
            b = ListRef::Cons(Wrap(3), &a);
            c = ListRef::Cons(Wrap(4), &a);

            // a がスコープ内であれば b, c を利用することができる。
            // ここで b, c の所有権は _hb, _hc に移る
            let ListRef::Cons(_hb, tlb) = b else { panic!() };
            let ListRef::Cons(_hc, _tc) = c else { panic!() };
            // b; // use of partially moved value: `b`

            // *tlb をムーブするのはできないが、値の評価のための一時的なデリファレンスは可能
            // let tail = *tlb; // cannot move out of `*tlb` which is behind a shared reference
            assert_eq!(
                *tlb,
                ListRef::Cons(Wrap(5), &ListRef::Cons(Wrap(10), &ListRef::Nil))
            );

            // *tlb をパターンマッチで分解する場合、T のムーブはできない(T: Copy ならコピーは可能)。
            // もし head の値そのものが欲しいなら clone するのが最も簡単。
            // let ListRef::Cons(h, _t) = *tlb else { panic!() }; // cannot move out of `tlb` as enum variant `Cons` which is behind a shared reference
            let ListRef::Cons(h, _t) = tlb else { panic!() };
            assert_eq!(h.0, 5); // Wrap が保持する値(i32 = Copy 実装)が欲しいだけなら h のデリファレンスの必要はない。
        }

        // a（要素の一部）はスコープの外に出ると寿命が切れてしまう。このため a を参照している b, c をスコープの外で使うことはできない。
        // let ListRef::Cons(_hb, _tb) = b else { panic!() }; // use of moved value
        // let ListRef::Cons(_hc, _tc) = c else { panic!() };
    }

    // Option を使った Stack でも ListRef と基本的には同様。
    fn stack() {
        let b;
        let c;
        {
            let a = Stack(Some((Wrap(5), &Stack(Some((Wrap(10), &Stack(None)))))));
            b = Stack(Some((Wrap(3), &a)));
            c = Stack(Some((Wrap(4), &a)));

            // ListRef と同様
            let Stack(Some((_hb, tb))) = b else { panic!() };
            let Stack(Some((_hc, _tc))) = c else { panic!() };
            let Stack(Some((h, _t))) = tb else { panic!() };
            // let Stack(Some((h, _t))) = *tb else { panic!() }; // cannot move out of `tb.0` as enum variant `Some` which is behind a shared reference
            assert_eq!(h.0, 5);
        }

        // let Stack(Some((hb, tb))) = b else { panic!() };
        // let Stack(Some((hc, tc))) = c else { panic!() };
    }

    pub fn run() {
        listref();
        stack();
    }
}

/// Cons リストとイミュータブルスタックを Box で実装。
/// Box は所有権を持つため、ライフタイムの問題が発生しにくく、デリファレンスもしやすい。
/// ただし、Box だと複数のリストで要素を共有することはできず、複数の変数で持ちたいなら clone でディープコピーをする必要がある。
mod box_scope {
    use std::{
        any::{Any, TypeId},
        ops::Deref,
        ptr,
    };

    #[derive(Clone, Debug, PartialEq)]
    enum ListBox<T> {
        Cons(T, Box<ListBox<T>>),
        Nil,
    }

    impl<T> ListBox<T> {
        fn first(&self) -> Option<&T> {
            if let ListBox::Cons(v, _) = self {
                Some(v)
            } else {
                None
            }
        }
    }

    #[derive(Clone)]
    struct Stack<T>(Option<Box<(T, Stack<T>)>>);

    #[derive(Clone, Debug, PartialEq)]
    struct Wrap<T>(T);

    fn listbox() {
        let b;
        let c;
        {
            let a = ListBox::Cons(
                Wrap(5),
                Box::new(ListBox::Cons(Wrap(10), Box::new(ListBox::Nil))),
            );
            let a_box = Box::new(a); // a の所有権が奪われる。
            // Box の参照先の値が Clone を実装していれば、その値ごと clone する。ディープコピーコストがかかる。
            let a_clone = a_box.clone();
            b = ListBox::Cons(Wrap(3), a_box); // b が a を所有する
            // a はムーブ済みなので a_box を再利用して c を作ることはできない。
            // c = ListBox::Cons(Wrap(4), a_box); // use of moved value: `a_box`
            c = ListBox::Cons(Wrap(4), a_clone);
        }

        // a の所有権は b に移っているため、a の寿命が切れても b, c は問題なく使用できる。
        // ここで b, c の所有権は bbox, cbox に移る。
        let ListBox::Cons(Wrap(head_b), bbox) = b else {
            panic!()
        };
        let ListBox::Cons(Wrap(head_c), cbox) = c else {
            panic!()
        };
        assert_eq!(head_b, 3);
        assert_eq!(head_c, 4);
        // b; // use of partially moved value: `b`

        // bbox と cbox は値としては同一とみなせるが、指しているもの（アドレス）は別のものとなる
        assert_eq!(bbox, cbox, "bbox と cbox は同一とみなせる");
        // 下2行は結果も意味的にも同じではあるが、deref は通常は明示的に使わない（後述）。
        assert!(!ptr::eq(&*bbox, &*cbox));
        assert!(!ptr::eq(bbox.deref(), cbox.deref()));

        let listbox = *bbox; // Box はデリファレンスすると中身がムーブするという特殊な挙動
        // ListBox<T> の T に Copy が実装されていないと、パターンマッチで head のムーブが起きるため、listbox はもう使えなくなる。
        let ListBox::Cons(head, tailbox) = listbox else {
            panic!()
        };
        assert_eq!(head, Wrap(5));
        // assert_eq!(listbox.type_id(), TypeId::of::<ListBox<Wrap<i32>>>()); // borrow of partially moved value: `listbox`

        // Box を引数で渡したりメソッドで使う場合は、自動参照外しにより明示的に参照外しする必要はない。
        assert_eq!(tailbox.first(), Some(&Wrap(10)));
        // match でパターンマッチする場合は自分で参照外しする必要がある。
        // この場合、deref メソッドを呼ぶよりは &* のほうがよい。理由は stdlib サブパッケージの deref に記載。
        // Box の値の参照を取得するなら所有権は奪わない
        let ListBox::Cons(head2, _tailbox) = &*tailbox else {
            panic!()
        };
        assert_eq!(*head2, Wrap(10)); // Wrap<T> に Copy が実装されてないので *head2 をムーブしようとすると失敗するが、一時的なデリファレンスは可能。
        assert_eq!(tailbox.type_id(), TypeId::of::<Box<ListBox<Wrap<i32>>>>()); // tail は再利用可能

        // あるいは *tailbox としても、パターンマッチで Copy 可能なもののみ取得するのであれば *tailbox を使っても再利用可能。
        // （ここでタプルの第1要素の Box を取得しようとするとエラーになる）
        let ListBox::Cons(Wrap(head_wrapped), _) = *tailbox else {
            panic!()
        };
        assert_eq!(head_wrapped, 10);
        assert_eq!(tailbox.type_id(), TypeId::of::<Box<ListBox<Wrap<i32>>>>()); // tailbox は再利用可能
    }

    // Option を使った Stack でも ListBox と基本的には同様。
    fn stack() {
        let b;
        let c;
        {
            let a = Stack(Some(Box::new((
                Wrap(5),
                (Stack(Some(Box::new((Wrap(10), Stack(None)))))),
            ))));
            let a_clone = a.clone(); // ディープコピーコストがかかる。
            b = Stack(Some(Box::new((Wrap(3), a)))); // b が a を所有する
            // c = Stack(Some(Box::new((Wrap(4), a)))); // use of moved value: `a`
            c = Stack(Some(Box::new((Wrap(4), a_clone))));
        }

        let Stack(Some(bbox)) = b else { panic!() };
        let Stack(Some(cbox)) = c else { panic!() };
        assert_eq!(bbox.0, Wrap(3)); // Box<T> から直接 T のフィールドにアクセスしたりメソッド等を呼ぶことができる。
        assert_eq!(cbox.0, Wrap(4));
        assert!(!ptr::eq(&*bbox, &*cbox));

        let tuple = *bbox; // Box のデリファレンスによるムーブ
        let (head, Stack(tail)) = tuple; // パターンマッチで中身をムーブ
        assert_eq!(head.0, 3);
        // ムーブが起こるため bbox が使えなくなるのは ListBox と一緒。
        // bbox; // use of moved value: `bbox`

        let bx = tail.unwrap(); // Option を剥がす
        let (head2, _tail2) = &*bx; // Box の中身を参照で返す
        assert_eq!(head2.0, 5);
        assert_eq!(
            bx.type_id(),
            TypeId::of::<Box<(Wrap<i32>, Stack<Wrap<i32>>)>>()
        ); // bx は再利用可能
    }

    pub fn run() {
        listbox();
        stack();
    }
}

/// Cons リストとイミュータブルスタックを Rc で実装。
/// Rc を使うメリットは、データの所有権を構造体自体に持たせつつ、その構造体を clone してもデータ参照先の clone はせずコストがかからない、という点にある。
/// Rc::clone は他の言語のシャローコピーに近いといえる。
mod rc_scope {
    use std::rc::Rc;

    #[derive(Clone)]
    enum ListRc<T> {
        Cons(T, Rc<ListRc<T>>),
        Nil,
    }

    impl<T> ListRc<T> {
        fn first(&self) -> Option<&T> {
            if let ListRc::Cons(v, _) = self {
                Some(v)
            } else {
                None
            }
        }
    }

    #[derive(Debug, PartialEq)]
    struct Stack<T>(Option<Rc<(T, Stack<T>)>>);

    #[derive(Clone, Debug, PartialEq)]
    struct Wrap<T>(T);

    fn listrc() {
        let b;
        let c;
        {
            let a = Rc::new(ListRc::Cons(
                Wrap(5),
                Rc::new(ListRc::Cons(Wrap(10), Rc::new(ListRc::Nil))),
            ));
            assert_eq!(Rc::strong_count(&a), 1, "参照カウント1");

            // Rc::clone(&a) は参照先の値自体はクローンせず、参照カウントだけ増やして所有権を保持する。
            b = ListRc::Cons(Wrap(3), Rc::clone(&a));
            // a.clone() でも Rc::clone(&a) と同じことではあるが、Rc<T> のメソッドを呼ぶときは fully qualified syntax を使うべき。
            // - &Rc<T> は auto deref + Deref coercion で &T に自動変換されることにより Rc<T> から T のメソッドを解決できる。
            //   これによる T と Rc<T> のメソッド衝突を防ぐ。
            // - データのディープコピーをするのでないというのを明示するのに視覚的に区別する。
            c = ListRc::Cons(Wrap(4), a.clone());

            assert_eq!(Rc::strong_count(&a), 3, "a,b,c が所有している");
        }

        // 変数 a 自体の寿命は切れても、a が指すデータの所有権は b, c も持っているため、a のスコープ外でも使うことができる。
        let ListRc::Cons(_h, brc) = b else { panic!() };
        let ListRc::Cons(_h, crc) = c else { panic!() };
        assert_eq!(Rc::strong_count(&brc), 2); // a が drop されるときに参照カウントが減っている。
        assert_eq!(Rc::strong_count(&crc), 2); // brc と crc の参照カウントは当然等しい。
        {
            // brc と crc が指すものは同じ
            // - Rc::into_raw(Rc<T>) でもポインタは取得できるが、こちらは引数がムーブされるものの参照カウントが減らずそのままだとメモリリークする。
            let brc_ptr = Rc::as_ptr(&brc);
            let crc_ptr = Rc::as_ptr(&crc);
            assert_eq!(brc_ptr, crc_ptr);

            // ムーブされると参照カウントは減る
            let _ = move || crc;
            assert_eq!(Rc::strong_count(&brc), 1);
        }

        // Rc を引数で渡したりメソッドで使う場合は、自動参照外しにより明示的に参照外しする必要はない。
        assert_eq!(brc.first(), Some(&Wrap(5)));
        // match でパターンマッチする場合は自分で参照外しする必要がある。
        // Rc::as_ref(&self) でも結果は同じだが、AsRef は意味論的に「デリファレンス」を表すとは限らない。
        let ListRc::Cons(second, _tail) = &*brc else {
            panic!()
        };
        assert_eq!(*second, Wrap(5)); // Wrap<T> に Copy が実装されてないので *second をムーブしようとすると失敗するが、一時的なデリファレンスは可能。

        // デリファレンスによるムーブは（普通は）できないため、Rc<T> が T: Copy でないと以下2つは失敗する。
        // let rc: ListRc<Wrap<i32>> = *brc; // cannot move out of an `Rc`
        // let ListRc::Cons(head, tail) = *brc else { panic!() }; // cannot move out of an `Rc`

        // try_unwrap は Result を返し、強参照カウンタが1なら Ok で Rc を剥がして所有権を移す。
        // 強参照カウンタが1以外なら Err で元の Rc を返すが、この場合安全に取り出すことはできないため clone している。
        let ListRc::Cons(second, rc) = Rc::try_unwrap(brc).unwrap_or_else(|rc| (*rc).clone())
        else {
            panic!()
        };
        assert_eq!(Rc::strong_count(&rc), 1);
        assert_eq!(second, Wrap(5));

        {
            // Rc<T> が T: Copy であればデリファレンス時に参照先の値がコピーされる
            let rc = Rc::new(1);
            let v = *rc;
            assert_eq!(v, 1);
        }
    }

    // Option を使った Stack でも ListRc と基本的には同様。
    fn stack() {
        let b;
        let c;
        {
            let a = Rc::new((Wrap(5), Stack(Some(Rc::new((Wrap(10), Stack(None)))))));
            assert_eq!(Rc::strong_count(&a), 1, "参照カウント1");

            // Rc::clone(&a) は参照先の値自体はクローンせず、参照カウントだけ増やして所有権を保持する。
            b = Stack(Some(Rc::clone(&a)));
            c = Stack(Some(Rc::clone(&a)));
            assert_eq!(Rc::strong_count(&a), 3, "a,b,c が所有している");
        }

        let Stack(Some(brc)) = b else { panic!() };
        let Stack(Some(crc)) = c else { panic!() };
        assert_eq!(Rc::strong_count(&brc), 2); // a が drop されるときに参照カウントが減っている。
        assert_eq!(Rc::strong_count(&crc), 2); // brc と crc の参照カウントは当然等しい。
        assert_eq!(brc.0, Wrap(5)); // Rc<T> から直接 T のフィールドにアクセスしたりメソッド等を呼ぶことができる。
        assert_eq!(Rc::as_ptr(&brc), Rc::as_ptr(&crc)); // brc と crc が指すものは同じ

        // ムーブされると参照カウントは減る
        let _ = move || crc;
        assert_eq!(Rc::strong_count(&brc), 1);

        // Rc のムーブはできない。Rc<T> が T: Copy であればコピーされる。
        // let rc = *brc; // cannot move out of an `Rc`
        // let (head, tail) = *brc; // cannot move out of an `Rc`
        let (Wrap(head_wrapped), _) = *brc; // i32 は Copy 可能なため一応これならできる。
        assert_eq!(head_wrapped, 5);

        let (head, Stack(tail)) = &*brc; // Rc の中身を参照で返す
        assert_eq!(*head, Wrap(5)); // Wrap<T> に Copy が実装されてないので *head をムーブしようとすると失敗するが、一時的なデリファレンスは可能。
        let drc = Rc::clone(&brc); // brc は再利用可能
        assert_eq!(Rc::strong_count(&drc), 2);

        let rc = tail.as_ref().unwrap(); // 次の Rc の参照を取得
        assert_eq!(Rc::strong_count(rc), 1); // 強参照カウントは brc などと当然異なる。
        let (second, last) = &**rc; // &Rc の中身を参照で返す
        assert_eq!(second, &Wrap(10));
        assert_eq!(*last, Stack(None));
    }

    pub fn run() {
        listrc();
        stack();
    }
}

/// イミュータブルスタック(参照)にメソッドを追加。
/// 値型を使うと無限再帰になってしまうため無理やり参照型を使ってみたが、スタックなのに Stack の所有権が自分にないのは不自然。
/// さらに、参照は制約が多く Box や Rc と同じシグネチャにするのは困難。
mod stack_ref {
    /// clone が必要なら #[derive(Clone)] を加える。
    struct Stack<'a, T>(Option<(T, &'a Stack<'a, T>)>);

    impl<'a, T> Stack<'a, T> {
        pub fn new() -> Self {
            Self(None)
        }

        // 後述する Box や Rc のスタックと異なり、&self を引数に取る。
        // もし self がムーブだと、戻り値に使う &Stack は参照であり関数終了時に drop されてしまうため不可能。
        pub fn push(&'a self, x: T) -> Self {
            Self(Some((x, self)))
        }

        pub fn peek(&self) -> Option<&T> {
            self.0.as_ref().map(|(head, _)| head)
        }
    }

    impl<T: Clone> Stack<'_, T> {
        // Stack(Option<(T1, &Stack1(Option<(T2, &Stack2)>))>) の T は所有権があるため、戻り値にこれをそのまま利用。
        // &Stack1 については所有権がないため、&Stack1 の中にある T2 に対して clone して所有権を得た後、
        // 残りの &Stack2 を使って Stack を作り直し（所有権がある）、これを戻り値に使う。
        fn pop(self) -> (Self, Option<T>) {
            self.0.map_or_else(
                || (Self(None), None),
                |(head, Stack(tail))| {
                    (
                        tail.as_ref()
                            .map_or_else(|| Self(None), |(v, stack)| stack.push(v.clone())),
                        Some(head),
                    )
                },
            )
        }
    }

    pub fn run() {
        let s = Stack::new();
        assert_eq!(s.peek(), None);

        let s = s.push(42);
        assert_eq!(s.peek(), Some(&42));

        let s = s.push(1);
        let (s, head) = s.pop();
        assert_eq!(head, Some(1));
        assert_eq!(s.peek(), Some(&42));

        let (s, head) = s.pop();
        assert_eq!(head, Some(42));
        assert_eq!(s.peek(), None);

        let (s, head) = s.pop();
        assert_eq!(head, None);
        assert_eq!(s.peek(), None);
    }
}

/// イミュータブルスタック(Box実装)にメソッドを追加。
/// データを占有しているため clone するのであればディープコピーになるが、それ以外は Rc と同じように扱える。
mod stack_box {
    /// clone が必要なら #[derive(Clone)] を加える。
    struct Stack<T>(Option<Box<(T, Stack<T>)>>);

    impl<T> Stack<T> {
        pub fn new() -> Self {
            Self(None)
        }

        pub fn push(self, x: T) -> Self {
            Self(Some(Box::new((x, self))))
        }

        pub fn peek(&self) -> Option<&T> {
            self.0.as_ref().map(|bx| &bx.0)
        }

        pub fn pop(self) -> (Self, Option<T>) {
            self.0.map_or_else(
                || (Self(None), None),
                |bx| {
                    let (head, tail) = *bx;
                    (tail, Some(head))
                },
            )
        }
    }

    pub fn run() {
        let s = Stack::new();
        assert_eq!(s.peek(), None);

        let s = s.push(42);
        assert_eq!(s.peek(), Some(&42));

        let (s, head) = s.pop();
        assert_eq!(head, Some(42));
        assert_eq!(s.peek(), None);
    }
}

mod immutable_stack_rc {
    use std::rc::Rc;

    struct Stack<T>(Option<Rc<(T, Stack<T>)>>);

    // O(1) コピー
    impl<T> Clone for Stack<T> {
        fn clone(&self) -> Self {
            // 冗長な変換
            // {
            //     let opt: Option<Rc<(T, Stack<T>)>> = match &self.0 {
            //         Some(stack) => Some(Rc::clone(stack)),
            //         None => None,
            //     };
            //     Self(opt)
            // }

            // 上を clippy で変換
            // {
            //     let rc_ref = self.0.as_ref();
            //     let opt = rc_ref.map(Rc::clone);
            //     Self(opt)
            // }

            // もっと短く書ける。
            //
            // Option::clone(&self) の実装は以下の通り。
            // x が Rc<T> であれば x.clone() のところで Rc::clone(&x) が呼ばれるため、コストは O(1) で済む。
            // ```
            // fn clone(&self) -> Self {
            //     match self {
            //         Some(x) => Some(x.clone()),
            //         None => None,
            //     }
            // }
            // ```
            Self(self.0.clone())
        }
    }

    impl<T> Stack<T> {
        pub fn new() -> Self {
            Self(None)
        }

        pub fn push(self, x: T) -> Self {
            Self(Some(Rc::new((x, self))))
        }

        pub fn peek(&self) -> Option<&T> {
            self.0.as_ref().map(|rc| &rc.0)
        }
    }

    impl<T: Clone> Stack<T> {
        pub fn pop(self) -> (Self, Option<T>) {
            if let Some(rc) = self.0 {
                let result = Rc::try_unwrap(rc);
                let (head, tail) = result.unwrap_or_else(|rc| (*rc).clone());

                (tail, Some(head))
            } else {
                (Self(None), None)
            }
        }
    }

    pub fn run() {
        let s1 = Stack::new();
        assert_eq!(s1.peek(), None);

        let s1 = s1.push(42);
        let s2 = s1.clone();
        assert_eq!(s1.peek(), Some(&42));
        assert_eq!(s1.0.as_ref().map_or(0, Rc::strong_count), 2);
        assert_eq!(s2.0.as_ref().map_or(0, Rc::strong_count), 2);

        let (s1, head) = s1.pop();
        assert_eq!(head, Some(42));
        assert_eq!(s1.peek(), None);
        assert_eq!(s1.0.as_ref().map_or(0, Rc::strong_count), 0);
        assert_eq!(s2.0.as_ref().map_or(0, Rc::strong_count), 1);
    }
}

fn main() {
    box_scope::run();
    stack_box::run();
    ref_scope::run();
    stack_ref::run();
    rc_scope::run();
    immutable_stack_rc::run();
}
