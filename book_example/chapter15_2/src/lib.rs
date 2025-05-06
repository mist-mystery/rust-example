//! Deref
//! - Deref coercion

/// 参照及びBoxに対して参照外し演算子を使用。
pub fn deref() {
    #[derive(Debug, PartialEq)]
    /// 独自型（== 比較できるように Debug と PartialEq を実装）
    struct Data<T>(T);

    {
        // 参照外し演算子(dereference operator)を使ってデータまで参照をたどる
        let x = 5;
        let y = &x;
        let _z = *y;
        assert_eq!(5, x);
        assert_eq!(5, *y);
        // assert_eq!(5, y, "integer と &integer の比較はできない。");

        // 独自型に対しても同じことはできる。
        // 参照をデリファレンスしようとしたときに値に Copy が実装されていない場合、基本的にはエラーになる（参照は所有権を持たないためムーブはできない）。
        // ただし、Deref 実装で構造体が保持する値を返すようにしたり、Box のようにコンパイラによる特殊扱いを受けていたりする場合、例外的にムーブができることはある。
        let d1 = Data(42);
        let d2 = &d1;
        assert_eq!(Data(42), d1);
        assert_eq!(Data(42), *d2); // デリファレンス直後に消費するなら問題ない
        // let d3: Data<i32> = *d2; // cannot move out of `*d2` which is behind a shared reference
    }
    {
        // y は x の参照でなく、xの値を指すボックスのインスタンス。
        let x = 5;
        let y = Box::new(x);
        let _z = *y;
        assert_eq!(5, x);
        assert_eq!(5, *y); // &i32 と同様、参照外し演算子を使ってボックスのポインタを辿ることができる。

        // Box::new により d はムーブされる。e に対して参照外し演算子は同様に使用可能。
        let d1 = Data(42);
        let d2 = Box::new(d1); // d をムーブ
        // assert_eq!(Data(42), d, "d はムーブされているためもう d は使えない。");
        assert_eq!(Data(42), *d2);
        let _d3 = *d2; // Box は例外的にデリファレンスによってムーブできる。
        // assert_eq!(Data(42), *d2); // borrow of moved value: `*d2`
    }
}

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> Self {
        Self(x)
    }
}

fn is_hello(greet: &str) {
    assert_eq!("Hello", greet);
}

fn is_hello_mut(greet: &mut str) {
    assert_eq!("Hello", greet);
}

pub mod immutable {
    use super::MyBox;
    use std::ops::Deref;

    /// MyBox で参照外し演算子を使えるように、Deref トレイトを実装する。
    /// これにより &MyBox<T> だけでなく MyBox<T> に対しても参照外し演算子を使えるようになる。
    /// Deref トレイトを実装する構造体に対して参照外し演算子を使うと、*y は *(y.deref()) のように、
    /// deref メソッドの呼び出しと * 演算子の呼び出しに置き換えられる。
    ///
    /// ```
    /// use std::ops::Deref;
    ///
    /// let x = 5;
    /// let y = chapter15_2::MyBox::new(x);
    /// assert_eq!(5, *y);
    /// assert_eq!(5, *(y.deref()));
    /// ```
    impl<T> Deref for MyBox<T> {
        type Target = T;

        // 不変参照に対して * をオーバーライドする。
        // deref が &self を引数に取り &T を返すのは、構造体が持つ値を self からムーブしたくないため。
        // なお、&self は参照であり所有権を持たないため、T を返すのは不可能。
        fn deref(&self) -> &T {
            &self.0
        }
    }

    #[derive(Debug, PartialEq)]
    struct Data<T>(T);

    /// 独自型に対してDerefトレイトを実装し、参照外し演算子を使用。
    pub fn mybox() {
        let b1 = MyBox::new(5);
        let b1_deref = *b1; // 独自型に対して*演算子で参照外しするためには Deref トレイトを実装する。
        assert_eq!(b1_deref, 5);

        let b2 = MyBox::new(Data(5));
        // fn deref(&self) -> &T というシグネチャ、つまり self は参照であって所有権がないことから、構造体が保持する値をムーブするのは不可能。
        // let b2_deref = *b2; // cannot move out of dereference of `MyBox<immutable::Data<i32>>`
        let b2_ref = &*b2;
        assert_eq!(b2_ref, &Data(5));

        // Box と MyBox の比較。Box の完全な模倣は不可能。
        let bx = Box::new(Data(5));
        let bx_deref = *bx;
        assert_eq!(bx_deref, Data(5));
    }

    /// 独自型の Deref coercion (参照外し型強制)。
    /// Deref coercion は、特定の型の値への参照を関数やメソッド定義の引数型と一致しない引数として関数やメソッドに渡すときに自動的に発生する。
    /// 実引数として渡した値を、deref メソッドを必要な回数呼び出すことで、引数が必要とする型に変換する。
    pub fn implicit_deref_coercions() {
        let m = MyBox::new("Hello".to_string());

        // Deref coercion により、Deref トレイトを実装している MyBox<T> は、deref メソッドを呼び出すことで、
        // &MyBox<String> は &String に、&String は &str に変換される。
        // deref を呼ぶ回数はコンパイル時に自動解決されるためコストはかからない。
        super::is_hello(&m);

        // &MyBox<String> -> &String -> &str の変換を行って、それを is_hello に渡す。
        let m_deref = &**m;
        super::is_hello(m_deref);

        // Deref coercion がもしなければ、(*m) で MyBox<String> を String に参照外しし、& と [..] で String 全体の文字列スライスを取る必要がある。
        let m_str = &(*m)[..];
        super::is_hello(m_str);
    }
}

pub mod mutable {
    use super::MyBox;
    use std::ops::DerefMut;

    /// 可変参照に対して * をオーバーライドする。
    /// なお、DerefMut を実装するには Deref を実装しておく必要がある。
    /// Target は Deref で設定したものが使われるため、deref_mut の戻り値の型を deref のものから変えることはできないと思われる。
    impl<T> DerefMut for MyBox<T> {
        fn deref_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    pub fn implicit_deref_coercions() {
        let mut m = MyBox::new("Hello".to_string());
        let m_mref = &mut m;
        super::is_hello_mut(m_mref); // (MyBox<String> の)可変参照を(deref_mut メソッドで str の)可変参照に型強制。
        super::is_hello(m_mref); // 可変参照を(deref メソッドで)不変参照に型強制することもできる。
    }
}
