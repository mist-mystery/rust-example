//! Shareable mutable containers: Cell, RefCell, UnsafeCell, etc
//! - Cell<T>, RefCell<T> の説明は関数のコメントを参照。
//! - Rust コンパイラは、不変参照 &T はイミュータブルなデータを指しているという知識を元に最適化を行う。
//!   そのため通常は &T となっているデータを &mut T のように、つまり参照先を変更すると未定義動作となってしまう。
//!   しかし &UnsafeCell<T> が指すデータに対しては特別に「&T はイミュータブルだから最適化可能」という仮定が適用されなくなる。
//!   これにより、不変参照を可変参照のように扱えるようになり、内部可変性を実現できる。
//!   この挙動はコンパイラレベルで定まっているため、他の型では代用できない。
//! - Cell<T>, RefCell<T> はどちらも内部で UnsafeCell<T> を使っている。
//! - UnsafeCell<T> が保証するのは &T を &mut T として安全に扱えるようにすることだけであって、
//!   可変参照が複数存在したり、不変参照と可変参照が両方あるような状態は未定義動作となる。
//!   UnsafeCell<T> では基本的に get メソッドでポインタを取得してそれを値、不変参照、可変参照のいずれかに変換することになるが、
//!   借用規則に違反しないようにするのはプログラマの責任となる。
//!
//! keywords
//! - Interior Mutability (内部可変性)
//! - aliasing(&T), mutablity(&mut T)

use std::cell::{Cell, RefCell};

#[derive(Debug, PartialEq, Default)]
struct Wrap<T>(T);

/// - (immutable な) `Cell<T>` にはセルの中身への不変参照及び可変参照を取得する方法が用意されておらず、セルの内外に値をムーブすることで内部可変性を実装している。
///   セルの中身が欲しいなら、`swap`, `replace`, `take` で値を入れ替えたり、`into_inner` で `Cell<T>` を消費（所有権を移譲）して値を取り出したりする。
/// - mutable な `Cell<T>` であってもセルの中身への不変参照を得る方法はない。
///   一方、`&mut Cell<T>` でのみ使える `get_mut` メソッドで `&mut T` を1つだけ取得することは可能であるが、
///   これが生きている間は `Cell<T>` への操作は全て制限されるため、`Cell<T>` の中身の読み書きは取得した `&mut T` を通してしかできなくなる。
///   これによりセルの中身を安全に書き換えることができるようになっている。
/// - `T: Copy` であれば `get` メソッドで内部の値を複製して取得することができる。
fn cell() {
    {
        // 任意の T に対して使えるメソッド new, replace, swap, into_inner
        let cell1 = Cell::new(Wrap(1)); // new でセルの中身の初期値を設定（ムーブ）
        let cell2 = Cell::new(Wrap(2));
        let wrap3 = Wrap(3);

        // replace で Cell に値をムーブして中身を変更しつつ、元の Cell の中身をムーブする。
        let wrap1 = cell1.replace(wrap3);
        // assert_eq!(wrap3, Wrap(3)); // borrow of moved value: `wrap3`
        assert_eq!(wrap1, Wrap(1));

        // swap で2つの Cell の所有権を奪わずに中身を交換する。
        cell1.swap(&cell2);
        assert_eq!(cell1.into_inner(), Wrap(2)); // into_inner で Cell<T> の所有権を奪って中身を取得する。
        assert_eq!(cell2.into_inner(), Wrap(3));
        // cell1.swap(&cell2); // 所有権がなくなったため cell1, cell2 ともに使用不可
    }
    {
        // 任意の T に対して使えるメソッド set
        // なお、RefCell や UnsafeCell には set はない。
        let cell = Cell::new(Wrap(1));
        cell.set(Wrap(2)); // set で Cell<T> の中身を変更する。
        assert_eq!(cell.into_inner(), Wrap(2));
    }
    {
        // T: Default に対して使えるメソッド take
        let cell = Cell::new(Wrap(1));
        assert_eq!(cell.take(), Wrap(1)); // take でセルの中身をデフォルト値と交換する（derive で自動実装可能）。
        assert_eq!(cell.into_inner(), Wrap(0));
    }
    {
        // T: Copy に対して使えるメソッド get
        // なお、RefCell には get はなく、UnsafeCell の get は中身のポインタを取得する別物である。
        let cell = Cell::new(1);
        let value1 = cell.get(); // get で Cell<T> の中身をコピーして取得
        assert_eq!(value1, 1);
        let value2 = cell.into_inner();
        assert_eq!(value2, 1);
        assert!(!std::ptr::eq(&value1, &value2)); // value1 と value2 は値が同じ別物
    }
    {
        // &mut self (mutable な Cell<T>) に対して使えるメソッド get_mut
        // get_mut で mutable な cell から（通常の借用規則と同様に）可変借用を1つだけ取得可能。
        // とはいえ、Cell<T> を使うのは大体が内部可変性が欲しいからなので、mutable にすることは少ないと思われる。
        let mut cell = Cell::new(Wrap(1));
        let cell_ref = &cell;
        cell_ref.set(Wrap(2)); // 適当に Cell<T> を借用するメソッドを使用。ここでの使用は問題ない。

        // get_mut で Cell<T> の中身への可変参照を得た状態だと、Cell<T> に対して何の操作もできなくなる。
        // &mut で可変参照を取得すると、可変参照が生きている間は参照元の変数に対して何もできなくなるのと同じ。
        let cell_get_mut = cell.get_mut(); // 可変借用開始
        // cannot borrow `cell` as mutable because it is also borrowed as immutable mutable borrow occurs here
        // ↓を有効にすると、cell_ref の不変参照が↑での可変参照取得より後まで残ってしまうため、借用規則に違反しエラーになる。
        // let cell2 = cell_ref.replace(Wrap(3));

        // cannot borrow `cell` as mutable more than once at a time second mutable borrow occurs here
        // assert_eq!(cell.get_mut(), &mut Wrap(2)); // cell_get_mut が生きているうちは、cell の可変借用は許可されない。

        // cannot borrow `cell` as immutable because it is also borrowed as mutable immutable borrow occurs here
        // cell.set(Wrap(3)); // cell の不変参照を引数に取るメソッドも許可されない。

        // cannot move out of `cell` because it is borrowed move out of `cell` occurs here
        // assert_eq!(cell.into_inner(), Wrap(2)); // cell のムーブも許可されない。

        *cell_get_mut = Wrap(3); // 可変参照経由で cell の中身を変更
        let _ = cell_get_mut; // 可変参照を drop
        // 可変参照がなくなったので cell は再び参照を取ることやムーブが可能になる。
        assert_eq!(cell.replace(Wrap(4)), Wrap(3)); // 不変参照
        assert_eq!(cell.into_inner(), Wrap(4)); // ムーブ
    }
}

/// RefCell<T> は格納している値の不変参照を borrow で、可変参照 borrow_mut で取得できる。
/// 借用チェックは動的に、つまり実行時に検査され、このとき借用規則に違反していれば panic する。
/// 仕組みはざっくり以下の通り。borrow メソッドを呼んだ流れを書いたが、borrow_mut メソッドを呼んだ場合も流れは同じ。
/// 1. RefCell<T> に借用の数をカウントする borrow という private field があり、不変借用で1加算され、可変借用で -1 が設定される。
/// 2. borrow メソッドを呼んだときに内部で BorrowRef インスタンスが作成され、ここで RefCell の borrow フィールドを検査している。
/// 3. borrow メソッドで得た Ref<'_, T> は private field に borrow を持っており、これは結果的に RefCell<T> の borrow を参照している。
/// 4. Ref<'_, T> の drop 時には borrow フィールドの値を戻すことで、RefCell<T> の borrow フィールドの値を変えている。
fn ref_cell() {
    {
        // RefCell だけで使える借用のためのメソッド borrow, borrow_mut, try_borrow, try_borrow_mut
        // RefCell を使う主目的である内部可変性は borrow_mut を使うことで実現できる。
        // 具体的には、RefCell の borrow_mut 呼ぶことで可変参照を取得し、それを経由して変更することで、RefCell の中身が書き変わる。
        // コンパイル時の借用チェックができず、実行時に借用チェックを行うので、参照を残したままにしていると panic しかねない。
        // 取得した Ref, RefMut は可能な限り使い捨てるのが基本（borrow, borrow_mut のたびに借用チェックするため僅かにコストはかかる）。
        let ref_cell = RefCell::new(Wrap(1));
        *ref_cell.borrow_mut() = Wrap(2); // 可変参照は文の終了時に破棄される。
        assert_eq!(*ref_cell.borrow(), Wrap(2)); // borrow で不変参照を取得し、RefCell の中身を確認する
        // Copy できない場合に値の所有権を得るなら clone なりが必要。
        // let wrap = *ref_cell.borrow(); // cannot move out of dereference of `std::cell::Ref<'_, Wrap<i32>>`

        // borrow_mut で取得した可変参照を変数に保持し RefCell の中身を書き換える場合、変数は mutable にする。
        // 戻り値の RefMut<'_, T> は、deref coercion により &mut T として扱われる。
        let mut ref_mut_wrap = ref_cell.borrow_mut();
        *ref_mut_wrap = Wrap(3);
        assert_eq!(*ref_mut_wrap, Wrap(3));
        drop(ref_mut_wrap); // ここで可変参照を drop することで、この行以降 ref_cell で borrow や borrow_mut を呼んでも問題なくなる。

        let ref_wrap = ref_cell.borrow(); // 不変参照の取得を問題なく行える。
        assert_eq!(*ref_wrap, Wrap(3));

        // try_borrow と try_borrow_mut は Result 型を返す以外は borrow, borrow_mut と同じ。
        assert_eq!(*ref_cell.try_borrow().unwrap(), Wrap(3)); // ref_wrap が生きていても不変借用は複数可
        assert!(ref_cell.try_borrow_mut().is_err()); // ここでの可変借用は許可されずエラーとなる

        let ref_wrap_scope;
        {
            // borrow や borrow_mut が返す Ref, RefMut は寿命が RefCell と等しいため、
            // RefCell のスコープ外で Ref, RefMut を使うことはできない。
            let _ref_cell2 = RefCell::new(Wrap(42));
            // ref_wrap_scope = _ref_cell2.borrow(); // `_ref_cell2` does not live long enough borrowed value does not live long enough

            ref_wrap_scope = ref_cell.borrow();
        }
        assert_eq!(*ref_wrap_scope, Wrap(3));
    }
    {
        // 任意の T に対して使えるメソッド new, replace, replace_with, swap, into_inner
        let ref_cell1 = RefCell::new(Wrap(1)); // new でセルの中身の初期値を設定（ムーブ）
        let ref_cell2 = RefCell::new(Wrap(2));
        let wrap = Wrap(3);

        // replace でセルの中に値をムーブして中身を変更しつつ、元のセルの中身が外にムーブされる。
        let replaced_wrap = ref_cell1.replace(wrap);
        // assert_eq!(wrap, Wrap(3)); // borrow of moved value: `wrap3`
        assert_eq!(replaced_wrap, Wrap(1));

        // RefCell 限定で replace_with が使える（内部で borrow_mut を使用している）。
        // closure が返す値をセルの中にムーブし、元のセルの中身が外にムーブされる。
        let replaced_with_wrap = ref_cell1.replace_with(|&mut Wrap(i)| Wrap(i + 1));
        assert_eq!(replaced_with_wrap, Wrap(3));

        // swap で2つの Cell の所有権を奪わずに中身を交換する。
        ref_cell1.swap(&ref_cell2);
        assert_eq!(ref_cell1.into_inner(), Wrap(2)); // into_inner で Cell<T> の所有権を奪って中身を取得する。
        assert_eq!(ref_cell2.into_inner(), Wrap(4));
        // ref_cell1.swap(&ref_cell2); // 所有権がなくなったため ref_cell1, ref_cell2 ともに使用不可
    }
    {
        // T: Default に対して使えるメソッド take
        let cell = RefCell::new(Wrap(1));
        assert_eq!(cell.take(), Wrap(1)); // take でセルの中身をデフォルト値と交換する（derive で自動実装可能）。
        assert_eq!(cell.into_inner(), Wrap(0));
    }
    {
        // &mut self (mutable な Cell<T>) に対して使えるメソッド get_mut
        // 使い方は Cell と同じだし、RefCell は borrow_mut で可変参照を得られるため使う機会はほぼないと思われる。
        let mut ref_cell = RefCell::new(Wrap(1));

        let cell_get_mut = ref_cell.get_mut(); // 可変借用開始
        // assert_eq!(ref_cell.get_mut(), &mut Wrap(1)); // cell_get_mut が生きているうちは、ref_cell の可変借用は許可されない。
        // ref_cell.replace(Wrap(2)); // ref_cell の不変参照を引数に取るメソッドも許可されない。
        // assert_eq!(ref_cell.into_inner(), Wrap(1)); // ref_cell のムーブも許可されない。

        *cell_get_mut = Wrap(2); // 可変参照経由で ref_cell の中身を変更
        let _ = cell_get_mut; // 可変参照を drop
        assert_eq!(ref_cell.into_inner(), Wrap(2)); // ムーブ
    }
}

// これのテストを tests::{immutable_messenger, mutable_messenger, refcell_messenger} に記述。
mod tracker {
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    // とりあえず適当な Messenger を実装して動かす
    // DummyMessenger はメッセージを出力するだけで、DummyMessenger 内部の状態は何も変えない
    pub fn dummy_messanger() {
        struct DummyMessenger;
        impl Messenger for DummyMessenger {
            fn send(&self, msg: &str) {
                println!("{msg}");
            }
        }
        let mut tracker = LimitTracker::new(&DummyMessenger, 100);
        tracker.set_value(90);
    }
}

// 構造体でリストやスタックを表現するとき、以下を全て満たしたいとする。
// 1. 構造体の要素が自身の所有権を持つ。
// 2. 構造体の要素を共有して複数箇所で使用する。
// 3. 構造体の要素を可変にする。
//
// 満たしたいのが1と2だけなら Rc<T>, 1と3だけなら Box<T>, 2と3だけなら &mut T など他の手段もとれるが、
// 全部満たしたいとなると Rc と RefCell (あるいは Cell) の組合せが必須となる。
// なお、マルチスレッドの場合はまた事情が異なってくる（Arc + Mutex を利用）。
mod rc_refcell {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    struct Wrap<T>(T);

    #[derive(Debug)]
    enum List<T> {
        #[allow(dead_code)]
        Cons(Rc<RefCell<T>>, Rc<List<T>>),
        Nil,
    }

    #[derive(Debug)]
    struct Stack<T>(Option<Rc<(RefCell<T>, Stack<T>)>>);

    impl<T> Clone for Stack<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<T> Stack<T> {
        pub fn new() -> Self {
            Self(None)
        }

        pub fn push(&mut self, x: T) {
            let this = Self(self.0.take());
            self.0 = Some(Rc::new((RefCell::new(x), this)));
        }

        pub fn peek(&self) -> Option<&RefCell<T>> {
            self.0.as_ref().map(|rc| &rc.0)
        }
    }

    fn list() {
        // 可変かつ共有したい要素
        let value = Rc::new(RefCell::new(Wrap(5)));

        let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));
        let b = List::Cons(Rc::new(RefCell::new(Wrap(3))), Rc::clone(&a));
        let c = List::Cons(Rc::new(RefCell::new(Wrap(4))), Rc::clone(&a));

        {
            // これをスコープで囲まないと、println! のときに可変借用を取得したままであるため、正しく表示できなくなる。
            let mut w = value.borrow_mut();
            w.0 += 5;
        }
        value.borrow_mut().0 += 5; // もしくは即座に可変借用を手放せば問題ない。

        println!("a after = {a:?}");
        println!("b after = {b:?}");
        println!("c after = {c:?}");
    }

    fn stack() {
        let mut s1 = Stack::new();
        s1.push(Wrap(5)); // 共有したい値は Wrap(5) で、これはムーブして所有権を Stack が持つようにする。
        let ref_cell = s1.peek().unwrap(); // Wrap(5) を持つ RefCell への参照を取得しておく。

        let mut s2 = s1.clone();
        let mut s3 = s1.clone();
        s2.push(Wrap(3));
        s3.push(Wrap(4));

        // ここで ref_cell から可変参照を取得して値を変更する。この変更は s1 だけでなく s2 や s3 にも影響を与える。
        ref_cell.borrow_mut().0 += 10;

        println!("{s1:?}");
        println!("{s2:?}");
        println!("{s3:?}");
    }

    pub fn run() {
        list();
        stack();
    }
}

fn main() {
    cell();
    ref_cell();
    tracker::dummy_messanger();
    rc_refcell::run();
}

#[cfg(test)]
mod tests {
    /// borrow_mut が panic する確認だけはここで行う。
    mod borrow_mut {
        use std::cell::RefCell;

        #[test]
        #[should_panic = "already mutably borrowed: BorrowError"]
        /// RefMut が有効な間に Ref を作ろうとしたら panic する。
        fn mutably_borrow() {
            let c = RefCell::new(5);

            let _m = c.borrow_mut();
            assert_eq!(*c.borrow(), 5); // this causes a panic
        }
    }

    // Messenger の send メソッドのシグネチャを &self (不変参照)のまま何とかしたい、という状態。
    mod immutable_messenger {
        use crate::tracker::{LimitTracker, Messenger};

        struct MockMessenger {
            sent_messages: Vec<String>,
        }

        impl MockMessenger {
            fn new() -> Self {
                Self {
                    sent_messages: vec![],
                }
            }
        }

        impl Messenger for MockMessenger {
            fn send(&self, _message: &str) {
                // self が 不変参照であるため push は使えない。
                // self.sent_messages.push(String::from(message));
            }
        }

        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
            limit_tracker.set_value(80);
            assert_eq!(mock_messenger.sent_messages.len(), 0);
        }
    }

    // Messenger のシグネチャ自体を変えて可変にしてしまうパターン
    // うまくいくが LimitTracker が外部ライブラリの場合はどうしようもなくなる。
    mod mutable_messenger {
        struct MockMessenger {
            sent_messages: Vec<String>,
        }

        impl MockMessenger {
            fn new() -> Self {
                Self {
                    sent_messages: vec![],
                }
            }
        }

        impl Messenger for MockMessenger {
            // 改変
            fn send(&mut self, message: &str) {
                self.sent_messages.push(String::from(message));
            }
        }

        // 再実装
        trait Messenger {
            fn send(&mut self, msg: &str);
        }

        // 再実装
        struct LimitTracker<'a, T: Messenger> {
            messenger: &'a mut T,
            value: usize,
            max: usize,
        }

        // 再実装
        impl<'a, T> LimitTracker<'a, T>
        where
            T: Messenger,
        {
            pub fn new(messenger: &'a mut T, max: usize) -> LimitTracker<'a, T> {
                LimitTracker {
                    messenger,
                    value: 0,
                    max,
                }
            }

            pub fn set_value(&mut self, value: usize) {
                self.value = value;
                let percentage_of_max = self.value as f64 / self.max as f64;
                if percentage_of_max >= 1.0 {
                    self.messenger.send("Error: You are over your quota!");
                } else if percentage_of_max >= 0.9 {
                    self.messenger
                        .send("Urgent warning: You've used up over 90% of your quota!");
                } else if percentage_of_max >= 0.75 {
                    self.messenger
                        .send("Warning: You've used up over 75% of your quota!");
                }
            }
        }

        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mut mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mut mock_messenger, 100);
            limit_tracker.set_value(80);
            assert_eq!(mock_messenger.sent_messages.len(), 1);
        }
    }

    // RefCell を使って不変値への可変参照を得る模範解答
    mod refcell_messenger {
        use crate::tracker::{LimitTracker, Messenger};
        use std::cell::RefCell;

        struct MockMessenger {
            sent_messages: RefCell<Vec<String>>,
        }

        impl MockMessenger {
            fn new() -> Self {
                Self {
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }

        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                self.sent_messages.borrow_mut().push(String::from(message));
            }
        }

        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
            limit_tracker.set_value(80);
            assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        }
    }
}
