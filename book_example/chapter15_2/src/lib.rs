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
        assert_eq!(5, x);
        assert_eq!(5, *y);
        // assert_eq!(5, y, "integer と &integer の比較はできない。");

        // 独自型に対しても同じことはできる。
        let d = Data(42);
        let e = &d;
        assert_eq!(Data(42), d);
        assert_eq!(Data(42), *e);
    }
    {
        // y は x の参照でなく、xの値を指すボックスのインスタンス。
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(
            5, *y,
            "&i32 と同様、参照外し演算子を使ってボックスのポインタを辿ることができる。"
        );

        // Box::new により d はムーブされる。e に対して参照外し演算子は同様に使用可能。
        let d = Data(42);
        let e = Box::new(d); // d をムーブ
        // assert_eq!(Data(42), d, "d はムーブされているためもう d は使えない。");
        assert_eq!(Data(42), *e);
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
        // deref が T でなく &T を返すのは、構造体が持つ値を self からムーブして所有権を渡すのは通常望ましい動作ではないため。
        fn deref(&self) -> &T {
            &self.0
        }
    }

    /// 独自型に対してDerefトレイトを実装し、参照外し演算子を使用。
    pub fn mybox() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(
            5, *y,
            "*演算子で参照外しするためには Deref トレイトを実装する。"
        );
    }

    /// 独自型の Deref coercion (参照外し型強制)。
    /// 参照外し型強制は、特定の型の値への参照を関数やメソッド定義の引数型と一致しない引数として関数やメソッドに渡すときに自動的に発生する。
    /// 実引数として渡した値を、deref メソッドを必要な回数呼び出すことで、引数が必要とする型に変換する。
    pub fn implicit_deref_coercions() {
        let m = MyBox::new(String::from("Hello"));

        // 参照外し型強制により、Deref トレイトを実装している MyBox<T> は、deref メソッドを呼び出すことで、
        // &MyBox<String> は &String に、&String は &str に変換される。
        // deref を呼ぶ回数はコンパイル時に自動解決されるためコストはかからない。
        super::is_hello(&m);

        // 参照外し型強制がもしなければ、(*m) で MyBox<String> を String に参照外しし、& と [..] で String 全体の文字列スライスを取る必要がある。
        let m_str = &(*m)[..];
        super::is_hello(m_str);

        // deref メソッドを2回呼び出すことでも &str に変換できる。
        let m_deref = (m.deref()).deref();
        super::is_hello(m_deref);
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
        let mut m = MyBox::new(String::from("Hello"));
        let m_mref = &mut m;
        super::is_hello_mut(m_mref); // 可変参照を(deref_mut メソッドで)可変参照に型強制。
        super::is_hello(m_mref); // 可変参照を(deref メソッドで)不変参照に型強制することもできる。
    }
}
