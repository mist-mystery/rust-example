//! ライフタイムとは、参照が有効でなければならない領域のことである。
//! 参照を保持する変数のライフタイムが長引くと、借用期間も長くなる（ことが多く、借用規則に違反しがちである）ため、
//! コンパイラはできるだけこのライフタイムを最小限にしようとする。
//! ほとんどの場合暗黙的に推論されるが、関数の境界をまたぐ場合など、明示的に指定が必要になる場合もある。
//!
//! ライフタイムにより dangling reference の発生を回避できる。
//!
//! # keywords
//! - lifetime  
//!   dangling reference 回避が目的。
//!   ライフタイム引数により、関数の引数と戻り値のライフタイムの接続に関する情報を提供する。
//! - lifetime elision
//! - non-lexical lifetime (NLL)
//!
//! # Reference
//! - https://doc.rust-lang.org/nomicon/lifetimes.html
//! - https://doc.rust-lang.org/reference/lifetime-elision.html

mod lifetime {
    use std::cmp::min;

    #[rustfmt::skip]
    // 変数のライフタイムを明示。
    // r は 'a のライフタイムであるが、println! で 'b のライフタイムのメモリを参照しようとしているため、コンパイルエラーになる。
    fn simple()    {
        let r;          // ─────────┬─ 'a
        {                     //          │
            let x = 5;   // ─┬── 'b  │
            r = &x;           //  │       │
            assert_eq!(r, &5);//  │       │
        }                     // ─┘       │
        // println!("r: {r}");//          │
                              // ─────────┘
    }

    // 戻り値のライフタイム外で戻り値を使おう（参照やムーブ）とするとコンパイルエラー
    fn call_func() {
        let string_literal = "abcd";
        let result;
        {
            let xyz = "xyz".to_string();
            let xyz_ref = xyz.as_str();
            // 戻り値のライフタイムは string_literal, xyz_ref が両方とも生存している期間（ブロック内）となる。
            result = longest(string_literal, xyz_ref);
            assert_eq!(result, "abcd");
        }
        // result の参照先のライフタイムはスコープ内に限られるが、その外で result を参照しようとしているため、コンパイルエラー。
        // assert_eq!(result, "abcd");
    }

    // 文字列リテラルは静的ライフタイム &'static str を持つため、ブロックの外で戻り値を使っても問題ない。
    fn call_func_static() {
        let string_literal1 = "abcd";
        let result;
        {
            let string_literal2 = "xyz";
            result = longest(string_literal1, string_literal2);
        }
        assert_eq!(result, "abcd");
    }

    #[rustfmt::skip]
    // 引数に使用した変数をムーブするとそこで寿命が切れるため、ムーブ後に戻り値を使うことはできない。
    // コメントアウトを外すと cannot move out of `string1` because it is borrowed. と出る。
    fn call_move() {
        let string1 = "abcd".to_string();           // ─────────┬─ 'a
        let result;                                   //          │
        {                                                   //          │
            let string2 = "xyz".to_string();        // ─┬── 'b  │
            result = longest(&string1, &string2);      //  │       │
            assert_eq!(result, "abcd");                     //  │       │
            let _s = string1; // move               // ─┼───────┘
            // assert_eq!(result, "abcd");                  // ─┘
        }
    }

    // https://doc.rust-lang.org/nomicon/lifetimes.html#example-aliasing-a-mutable-reference
    fn take_over_ref() {
        // 'a: xyz の生存期間
        // 具体的には `assert_eq!(xyz, "xyz");` まで

        let mut xyz = "xyz".to_string();
        {
            // 'b: xyz_ref の生存期間
            // 具体的には `assert_eq!(xyz_ref, "xyz");` まで

            // frozen のシグネチャは引数と戻り値のライフタイムが同じとしているため、
            // 引数(&mut xyz)は 'b の間有効でなければならない。
            // でないと xyz_ref が 'b の間有効でなければならないというルールを満たせないからである。
            // その結果、xyz は 'b の間、すなわち xyz_ref が最後に使用されるまでの間ずっと可変借用を続けなければならない。
            let xyz_ref = frozen(&mut xyz);
            // assert_eq!(xyz, "xyz"); // 可変借用中に不変借用しようとするためエラーになる
            assert_eq!(xyz_ref, "xyz");
        }
        // 'b を抜ければ xyz の可変借用も終了するため、ここで xyz を不変借用するのは問題ない。
        assert_eq!(xyz, "xyz");
    }

    pub fn run() {
        simple();
        call_func();
        call_func_static();
        call_move();
        take_over_ref();

        assert_eq!(small_slice("abcd", "xyz"), "abc");
    }

    // x, y のライフタイムを 'a で指定しているので、
    // 'a は x として渡したもののライフタイムと y として渡したもののライフタイムの共通部分、またはそれより狭い範囲になる。
    // 戻り値の参照は 'a の指定をしているため、それと同じか狭い期間でしか有効でない。
    // 逆に言えば、x と y が生きている間、戻り値も生きられることは保証される。
    // このシグネチャの場合、戻り値のライフタイムが x と y のどちらと等しいかコンパイラは判断できないため、
    // ライフタイムを明示的に指定しないとコンパイルエラーとなる。
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    #[allow(dead_code)]
    #[allow(unreachable_code)]
    // 関数内で生成した値は、関数を抜けるとスコープを抜けてしまうため、参照を返すことはできない。
    // この場合、所有されたデータ型(String)を返すのが適切。
    fn longest_inner<'a>(_x: &str, _y: &str) -> &'a str {
        unimplemented!();

        // result のライフタイム('b とする)を返そうとしているが、返り値は &'a で指定しているため、'b: 'a である必要がある。
        // 'b は関数内までなので 'a より狭く、制約を満たせないためコンパイルエラーになる。
        let result = String::from("really long string");
        result.as_str()
    }

    // ライフタイム省略規則を満たす場合は、ライフタイムを省略できる。
    // - 1引数関数は一つのライフタイム引数、2引数関数は二つのライフタイム引数、…を得る。
    // - 入力ライフタイム引数が一つだけであれば、そのライフタイムが全ての出力ライフタイム引数に代入される。
    // - メソッドであり入力ライフタイム引数の一つが &self や &mut self であれば、self のライフタイム引数が全ての出力ライフタイム引数に代入される。
    fn frozen(x: &mut String) -> &String {
        x
    }

    // 文字列を加工して返す場合も、同様にライフタイムを設定する必要あり（index メソッドが &Self::Output を返すため）。
    fn small_slice<'a>(x: &'a str, y: &'a str) -> &'a str {
        let len = min(x.len(), y.len());
        if x < y { &x[..len] } else { &y[..len] }
    }
}

// the book の構造体のライフタイム注釈の例。
// とりあえず書いておいたが、余計な情報が多くて分かりにくいので、lifetime_struct にシンプルな例を記載する。
mod lifetime_book_struct {

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    #[allow(clippy::needless_lifetimes)] // このケースでは impl ImportantExcerpt<'_> とした方が簡潔。
    impl<'a> ImportantExcerpt<'a> {
        // ライフタイムの明示がない、かつ &self が引数にあるため、出力ライフタイム引数は self のライフタイムと同じになる。
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {announcement}");
            self.part
        }
    }

    pub fn run() {
        let i;

        // novel の寿命は i と同じか長くなければならない。
        let novel = String::from("Call me Ishmael. Some years ago...");
        {
            // ImportantExcerpt インスタンスは novel に所有される String への参照を持つため、
            // novel の寿命がインスタンスより先に切れるとコンパイルエラーとなる。
            // let novel = String::from("Call me Ishmael. Some years ago...");

            // first_sentence は novel に所有される String への参照
            let first_sentence = novel.split('.').next().expect("Could not find a '.'");
            i = ImportantExcerpt {
                part: first_sentence, // こちらも novel に所有される String への参照
            };
        }
        assert_eq!(i.announce_and_return_part("Test!"), "Call me Ishmael");
    }
}

mod lifetime_struct {
    // 構造体フィールドに参照を使う場合はライフタイムの明示が必要。
    // 構造体インスタンスはライフタイム 'a を超えて生存することはできない。
    // つまり、この構造体インスタンスは、（全ての）フィールドに保持している参照より長生きできない。
    struct X<'a>(&'a i32);

    // 構造体フィールド用のライフタイム名は impl キーワードの後に宣言する。
    impl<'a> X<'a> {
        fn get(&self) -> &i32 {
            self.0
        }

        // 戻り値のライフタイムを明示することで、&mut self のライフタイムと関係がなくなる。
        fn replace(&mut self, value: &'a i32) -> &'a i32 {
            let last_value = self.0;
            self.0 = value;
            last_value
        }

        // ライフタイム省略規則により、戻り値の &i32 のライフタイムは &mut self のライフタイムと同じになる。
        fn replace_borrow(&mut self, value: &'a i32) -> &i32 {
            let last_value = self.0;
            self.0 = value;
            last_value
        }

        // ライフタイム省略規則により、戻り値の &i32 のライフタイムは &self のライフタイムと同じになる。
        fn more_than(&self, value: &i32) -> Option<&i32> {
            if self.0 > value { Some(self.0) } else { None }
        }

        // メソッドでないため省略規則3は適用されない。こちらはライフタイム指定が必須。
        fn wrap_more_than<'b>(wrap: &'b X, value: &i32) -> Option<&'b i32> {
            if wrap.0 > value { Some(wrap.0) } else { None }
        }
    }

    // 構造体のライフタイムはフィールドの生存期間だけであるため、スコープの外側で構造体の参照は不可。
    fn struct_scope() {
        let x;
        {
            let value = 42;
            x = X(&value);
            assert_eq!(x.0, &42);
        }
        // assert_eq!(x.0, &42);
    }

    // replace の引数と戻り値のライフタイムは関係ないため、メソッド呼び出し時の x の可変借用は即座に解放される。
    fn replace() {
        let mut x = X(&42);
        let old = x.replace(&1);
        assert_eq!(x.get(), &1);
        assert_eq!(old, &42);
    }

    // replace_borrow の引数と戻り値のライフタイムは同じであるため、old 生存中は x を可変借用したままになってしまう。
    fn replace_borrow() {
        let mut x = X(&42);
        let old = x.replace_borrow(&1);
        // assert_eq!(x.get(), &1); // cannot borrow `x` as immutable because it is also borrowed as mutable
        assert_eq!(old, &42);
    }

    // ライフタイム省略規則3が有効なメソッドと、有効でない関連関数
    fn elision() {
        let value = 42;
        let x = X(&value);

        let large_value = 100;
        assert_eq!(x.more_than(&large_value), None);

        let small_value = 1;
        assert_eq!(X::wrap_more_than(&x, &small_value), Some(&42));
    }

    pub fn run() {
        struct_scope();
        replace();
        replace_borrow();
        elision();
    }
}

// ジェネリックな型引数、トレイト境界を組み合わせる。特筆すべきこともないため the book の例そのまま。
mod gen_trait_lifetime {
    use std::fmt::Display;

    pub fn run() {
        let x = String::from("Penguin");
        let y = String::from("Duck");
        let ann = "Bird";
        assert_eq!(
            longest_with_an_announcement(x.as_str(), y.as_str(), ann),
            "Penguin"
        );
    }

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {ann}");
        if x.len() > y.len() { x } else { y }
    }
}

mod lifetime_difference {
    struct Reader<'a> {
        book: &'a str,
    }

    impl<'a> Reader<'a> {
        // self と戻り値のライフタイムが一致する。
        fn read1_book(&mut self, _content: &str) -> &str {
            self.book
        }

        // self 及び戻り値のライフタイムは構造体フィールド book のライフタイムと一致する。
        // すなわち、このメソッドを呼んだ瞬間、インスタンスは寿命まで可変借用を続けるため二度と使えなくなる。
        fn read2_book(&'a mut self, _content: &str) -> &'a str {
            self.book
        }

        // content と戻り値 &str のライフタイムは同じになる。
        fn read3_content<'b>(&self, content: &'b str) -> &'b str {
            content
        }

        // &self と戻り値 &str のライフタイムは同じになる。
        #[allow(dead_code)]
        fn read4_content<'b>(&'b self, content: &'b str) -> &'b str {
            content
        }
    }

    fn exec_read1() {
        let mut reader = Reader { book: "the book" };
        {
            reader.read1_book("value");
        }
        println!("{}", reader.book);
    }

    // exec_read1 と比べて、関数の中身は read2_content メソッドを使っているの以外同じだが、
    // read2_content のライフタイム 'a の指定により、self、すなわち reader は
    // reader が生きている間ずっと可変借用することになってしまう。
    // （'a のライフタイムは既に決定されているため、戻り値を束縛する変数のライフタイムによって 'a すなわち reader の借用期間が変わるということもない）
    fn exec_read2() {
        let mut reader = Reader { book: "the book" };
        {
            reader.read2_book("value");
        }
        // println!("{}", reader.book); // cannot borrow `reader.book` as immutable because it is also borrowed as mutable
    }

    #[allow(unused_mut)]
    // read2 メソッドの戻り値 r は、引数で渡した値(value)の生存期間より長く生きれない。
    fn exec_read3() {
        // ここでは特に mutable にする意味はないが、exec_read4 と条件を同じにするために mutable にしている。
        let mut r;
        let value = String::from("value");
        {
            let reader = Reader { book: "the book" };
            r = reader.read3_content(&value);
        }
        assert_eq!(r, "value");
    }

    #[allow(unused_mut)]
    #[allow(unused_variables)]
    // exec_read3 と比べて、関数の中身は read4_content メソッドを使っているのと r を初期化してるかどうか以外同じだが、
    // read4_content メソッドのライフタイムの指定によって、戻り値 r が reader の生存期間（ブロック内）より長く生きれない。
    // しかし、ブロック外で r を使おうとしているため、コンパイルエラーになる。
    fn exec_read4() {
        let mut r = "value";
        let value = String::from("value");
        {
            let reader = Reader { book: "the book" };
            // r = reader.read4_content(&value); // ここでライフタイムのエラーとなる。
        }
        assert_eq!(r, "value");
    }

    pub fn run() {
        exec_read1();
        exec_read2();
        exec_read3();
        exec_read4();
    }
}

// Drop を実装していると、スコープを抜ける際に構造体が借用されたままであると、
// 可変借用するメソッドの drop が呼べなくてコンパイルエラーになる。
mod lifetime_drop {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct X<'a>(&'a i32);

    impl Drop for X<'_> {
        fn drop(&mut self) {
            println!("droppped: {:?}", self);
        }
    }

    pub fn run() {
        let mut data = vec![1, 2, 3];
        data.push(4);
        let x = X(&data[0]); // data の不変借用（x が drop するまで継続）
        println!("{x:?}");

        // x はスコープを抜けたときに可変借用メソッドの drop が呼ばれる。
        // data の不変借用が続いているままであるため、ここで data の可変借用メソッドは呼べない。
        // data.push(5);
    }
}

fn main() {
    lifetime::run();
    lifetime_book_struct::run();
    lifetime_struct::run();
    gen_trait_lifetime::run();

    lifetime_difference::run();
    lifetime_drop::run();
}
