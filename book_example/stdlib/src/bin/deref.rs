//! [Rust Doc](https://doc.rust-lang.org/std/ops/trait.Deref.html) には "Used for immutable dereferencing operations, like `*v.`" とある。
//! 他の std::ops と同様に、基本的には * 演算子使用時に自動で呼ばれるものなので、defer メソッドを明示的に呼び出すのはあまり行わない。
//! 独自型に Deref を実装すると、Deref Coercion によりコンパイラが暗黙的に deref メソッドを何度も呼ぶことがあるため注意が必要。
//! Rc や Box では `&*`, as_ref は結果は同じになるが、使い分けを意識した方が良いと言える。

mod rc {
    use std::{ops::Deref, rc::Rc};

    #[derive(Debug, PartialEq)]
    struct I32Tuple(i32);

    #[derive(Debug, PartialEq)]
    struct I32TupleTuple(I32Tuple);

    pub fn run() {
        rc_ref();
        rc_struct_field();
    }

    fn rc_ref() {
        let rc_t = Rc::new(I32Tuple(1));
        // let i32_t: Rc<I32Tuple> = *rc_t; // Rc<T> の T が Copy を実装していないと、Tをデリファレンスで取り出すことはできない。

        let rc_t = Rc::new(Rc::clone(&rc_t));

        // - &rc_t は &Rc<Rc<I32Tuple>> であるが、変数の型の明示で &I32Tuple に変換することが可能。
        //   とはいえ Deref coercion によりメソッドを呼び出したり関数の引数で渡したりする場面では（コンパイル時に）自動変換されるため、
        //   実用上ここで変換する意味はあまりないと言える。
        // - &*rc_t という書き方は *rc_t がダメでも可能。Deref coercion により Rc<T> の T を取り出す。
        // - Rc や Box に対する as_ref メソッドと deref メソッドはシグネチャも結果も同一となる。
        //   なお、as_ref は意味論的には「参照から（別の）参照への変換」を表し、必ずしも「デリファレンス」を表すとは限らない。
        //   deref は意味論的にも正しいが、std::ops::Deref を use する必要があるのと、基本的に deref はコンパイラが自動で呼ぶものであるという前提がある。
        let rc_ref_t = &rc_t;
        let ref_t: &I32Tuple = &rc_t;
        let ref_deref_t = &*rc_t;
        let asref_t = rc_t.as_ref();
        let deref_t = rc_t.deref();

        assert_tuple(rc_ref_t);
        assert_tuple(ref_t);
        assert_tuple(ref_deref_t);
        assert_tuple(asref_t);
        assert_tuple(deref_t);

        // match 式を使う場合は Deref coercion は行われないため、適宜デリファレンスが必要になる。
        match &**rc_t {
            I32Tuple(i) => println!("{i}"),
        }
    }

    fn rc_struct_field() {
        let rc_t = Rc::new(I32Tuple(1));
        let rc_t_t = Rc::new(I32TupleTuple(I32Tuple(1)));

        let i = rc_t.0; // Rc<T> の T が構造体で、フィールドが Copy を実装しているなら、そのフィールドを直接指定して透過的に値をコピーして取り出すことができる。
        // let i32_t: I32Tuple = rc_t_t.0; // フィールドが Copy を実装していないなら、透過的な値の取り出しはできない。
        let i32_t_t = rc_t_t.0.0; // これはOK
        assert_eq!(i, 1);
        assert_eq!(i32_t_t, 1);
    }

    fn assert_tuple(r: &I32Tuple) {
        assert_eq!(r, &I32Tuple(1));
    }
}

mod custom {
    use std::ops::Deref;

    struct DerefExample<T> {
        value: T,
    }

    impl<T> Deref for DerefExample<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    struct AsRefExample<T> {
        value: T,
    }

    impl<T> AsRef<T> for AsRefExample<T> {
        fn as_ref(&self) -> &T {
            &self.value
        }
    }

    pub fn run() {
        let deref_ex = DerefExample { value: "a" };
        let asref_ex = AsRefExample { value: "b" };

        assert_eq!("a", *deref_ex);
        assert_eq!(&"a", deref_ex.deref());
        // assert_eq!("b", *asref_ex); // AsRef を実装していても Deref を実装していなければデリファレンスは不可
        assert_eq!(&"b", asref_ex.as_ref());
    }
}

fn main() {
    rc::run();
    custom::run();
}
