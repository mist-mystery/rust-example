//! keywords
//! - lifetime  
//!   dangling reference 回避が目的。具体的には、関数の引数と戻り値のライフライムを接続するもの。

mod lifetime {
    use std::cmp::min;

    pub fn main() {
        let string1_literal = "abcd";
        let result;
        let literal_result;
        {
            let string2 = String::from("xyz");
            let string2 = string2.as_str();
            // 戻り値のライフタイムは string1, string2 が両方とも生存している期間となる。
            result = longest(string1_literal, string2);
            assert_eq!(result, "abcd");

            // 文字列リテラルは静的ライフタイムを持ち、実際は &'static str となる。
            let string2_literal = "xyz";
            literal_result = longest(string1_literal, string2_literal);
            assert_eq!(literal_result, "abcd");
        }

        // ここのコメントアウトを外すと、借用元（string2）の生存期間より長く（スコープの外まで）借用することになってしまうため、
        // ライフタイム規則違反でコンパイルエラー。
        // assert_eq!(result, "abcd");

        assert_eq!(literal_result, "abcd");

        assert_eq!(small_slice("abcd", "xyz"), "abc");
    }

    // x, y 及び戻り値の文字列スライスは、少なくともライフタイム 'a と同じだけ生きる文字列スライスである、と指定。
    // 戻り値のライフタイムが x と y のどちらと等しいか判断できないため、ライフタイムを明示的に指定しないとコンパイルエラー。
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    // 関数内で生成した値は、関数を抜けるとスコープを抜けてしまうため、参照を返すことはできない。
    // この場合、所有されたデータ型(String)を返すのが適切。
    // fn longest_inner<'a>(_x: &str, _y: &str) -> &'a str {
    //     let result = String::from("really long string");
    //     result.as_str()
    // }

    // ライフタイム省略規則を満たす場合は、ライフタイムを省略できる。
    // - 1引数関数は一つのライフタイム引数、2引数関数は二つのライフタイム引数、…を得る。
    // - 入力ライフタイム引数が一つだけであれば、そのライフタイムが全ての出力ライフタイム引数に代入される。
    // - メソッドであり入力ライフタイム引数の一つが &self や &mut self であれば、self のライフタイム引数が全ての出力ライフタイム引数に代入される。
    fn _id(x: &str) -> &str {
        x
    }

    // 文字列を加工して返す場合も、同様にライフタイムを設定する必要あり。
    fn small_slice<'a>(x: &'a str, y: &'a str) -> &'a str {
        let len = min(x.len(), y.len());
        if x < y { &x[..len] } else { &y[..len] }
    }
}

mod lifetime_struct {
    // この構造体インスタンスは、part フィールドに保持している参照より長生きしない。
    // 言い換えれば、part の所有者の寿命が先に切れるようだとコンパイルエラーになる。
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    // 構造体フィールド用のライフタイム名は impl キーワードの後に宣言する。
    impl<'a> ImportantExcerpt<'a> {
        // &self が引数にあるため、出力ライフタイム引数は self のライフタイムと同じになる。
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {announcement}");
            self.part
        }
    }

    pub fn main() {
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

mod gen_trait_lifetime {
    use std::fmt::Display;

    pub fn main() {
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

fn main() {
    lifetime::main();
    lifetime_struct::main();
    gen_trait_lifetime::main();
}
