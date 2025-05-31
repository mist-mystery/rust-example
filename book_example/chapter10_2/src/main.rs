//! keywords  
//! - trait
//! - orphan rule  
//!   トレイトの実装の重複を防ぐための規則
//! - default implementation
//! - trait boundary
//! - blanket implementation  
//!   `impt<T: Display> ToString for T { ... }`

const POST_SUMMARY: &str = "horse_ebooks: of course, as you probably already know, people";
const ARTICLE_SUMMARY: &str =
    "Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)";

/// トレイトを使わず、SocialPost 構造体にメソッドを直接実装する。
mod struct_implement {
    struct SocialPost {
        username: String,
        content: String,
        reply: bool,
        repost: bool,
    }

    impl SocialPost {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }

        fn is_popular(&self) -> bool {
            self.reply && self.repost
        }
    }

    pub fn run() {
        let post = SocialPost {
            username: "horse_ebooks".to_string(),
            content: "of course, as you probably already know, people".to_string(),
            reply: false,
            repost: false,
        };
        assert_eq!(post.summarize(), super::POST_SUMMARY);
        assert!(!post.is_popular());
    }
}

/// Summary トレイト、SocialPost 構造体、NewsArticle 構造体を定義し、Summary トレイトを SocialPost と NewsArticle に実装する。
/// ここでのトレイトの使い方は、他の言語でのインターフェースに近い。
mod trait_implement {
    // 実装するメソッドを外に公開したいなら pub をつける。
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct SocialPost {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub repost: bool,
    }

    // impl Summary はあくまでも Summary トレイトのメソッドを定義するものであり、
    // ここでトレイトに定義されているメソッド以外を定義することはできない。
    impl Summary for SocialPost {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }

        // ここに is_popular メソッドを書くとエラー。
        // `fn is_popular` is not a member of trait `Summary`
        // fn is_popular(&self) -> bool {
        //     self.reply && self.repost
        // }
    }

    // トレイトに定義されてないメソッドは、構造体に対して直接実装する。
    impl SocialPost {
        pub fn is_popular(&self) -> bool {
            self.reply && self.repost
        }
    }

    #[derive(Debug)]
    /// Summary を実装する別の構造体を定義
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        #[allow(dead_code)]
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }
}

/// trait_implement で定義した SocialPost と Summary を use して使う。
mod trait_import {
    use crate::trait_implement::{SocialPost, Summary};

    pub fn run() {
        let post = SocialPost {
            username: "horse_ebooks".to_string(),
            content: "of course, as you probably already know, people".to_string(),
            reply: false,
            repost: false,
        };
        // summarize メソッドを使うには、SocialPost 本体だけでなく Summary も use する必要がある。
        assert_eq!(post.summarize(), super::POST_SUMMARY);

        // 他言語のインターフェースであればこういうことが可能なはずだが、Rust ではできない。
        // これは、コンパイル時に型が決定している必要があるが、インターフェースだけだと決まらないため。
        // let summary: Summary = post;

        // これであれば通る。詳しくは英語版の chapter18 で扱うはず。
        let summary: &dyn Summary = &post;
        assert_eq!(summary.summarize(), super::POST_SUMMARY);

        // let summary: Summary は許されないが、引数をトレイトとする関数の定義は可能（コンパイル時には具体的な型に解決される）。
        arg_trait(&post);
        assert!(!post.is_popular());
    }

    // 引数に `impl トレイト` と指定することで、トレイトを実装する何らかの型を受け付ける。
    fn arg_trait(item: &impl Summary) {
        assert_eq!(item.summarize(), super::POST_SUMMARY);
    }
}

/// orphan rule
mod orphan {
    use crate::trait_implement::SocialPost;
    use std::fmt::Display;

    trait Summary {
        fn summarize(&self) -> String;
    }

    // orphan rule により、外部のトレイトを外部の型に対して実装することはできない
    // impl Display for Vec<i32> {
    //     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    //         write!(f, "Custom Vec formatting")
    //     }
    // }

    // 外部のトレイトをローカルの型に対して実装するのはOK
    impl Display for SocialPost {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "{}: {}\nreplay: {}, repost: {}",
                self.username, self.content, self.reply, self.repost
            )
        }
    }

    // 逆に、ローカルのトレイトに対して外部の型を実装するのもOK
    impl Summary for i32 {
        fn summarize(&self) -> String {
            self.to_string()
        }
    }

    pub fn run() {
        let post = SocialPost {
            username: "horse_ebooks".to_string(),
            content: "of course, as you probably already know, people".to_string(),
            reply: false,
            repost: false,
        };

        // SocialPost に Display を実装したため、to_string メソッドを呼べる。
        assert_eq!(
            post.to_string(),
            format!("{}\nreplay: false, repost: false", super::POST_SUMMARY)
        );

        // i32 に Summary を実装したため、summarize メソッドを呼べる。
        assert_eq!(1.summarize(), "1");
    }
}

/// デフォルト実装
mod default_implementation {
    use crate::trait_implement::{NewsArticle, SocialPost};

    // トレイトにはデフォルト実装を与えることもできる。
    // なお、trait_implement では SocialPost 等に impl Summary しているが、
    // ここで trait_implement::Summary を use しなければ、summarize 等の同名メソッドをローカルで定義することは可能。
    trait Summary {
        // こちらは実装必須。
        fn summarize_author(&self) -> String;

        // デフォルト実装は、通常のメソッド定義のように実装を記述する。
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    // SocialPost では summarize メソッドのデフォルト実装を上書き（通常のトレイト実装と記述は変わらない）。
    impl Summary for SocialPost {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }

        // デフォルト実装を上書き。なお、上書きしている実装の中でデフォルト実装を呼ぶことはできない。
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // NewsArticle では summarize メソッドはデフォルト実装を使う。
    impl Summary for NewsArticle {
        // 実装が必須な summarize_author だけ実装
        fn summarize_author(&self) -> String {
            format!("Author: {}", self.author)
        }
    }

    // post でも article でも Summary を実装しているため summarize メソッドを呼べる。
    pub fn run() {
        let post = SocialPost {
            username: "horse_ebooks".to_string(),
            content: "of course, as you probably already know, people".to_string(),
            reply: false,
            repost: false,
        };
        assert_eq!(post.summarize(), super::POST_SUMMARY);
        assert_eq!(post.summarize_author(), "@horse_ebooks");

        let article = NewsArticle {
            headline: "Penguins win the Stanley Cup Championship!".to_string(),
            location: "Pittsburgh, PA, USA".to_string(),
            author: "Iceburgh".to_string(),
            content: "The Pittsburgh Penguins once again are the best hockey team in the NHL."
                .to_string(),
        };
        assert_eq!(article.summarize(), "(Read more...)");
    }
}

/// トレイト境界構文
mod trait_boundary {
    use crate::trait_implement::{NewsArticle, SocialPost, Summary};
    use std::fmt::{Debug, Display};

    /// impl Trait 構文及びトレイト境界構文
    fn simple_trait_boundary(post: &SocialPost) {
        // トレイト境界構文
        fn notify_bound<T: Summary>(item: &T) {
            assert_eq!(item.summarize(), super::POST_SUMMARY);
        }

        // `item: &impl Trait` はトレイト境界構文の糖衣構文。
        // ジェネリクスを使わなくていいため単純なケースならこちらのほうが簡潔。
        fn notify_impl(item: &impl Summary) {
            assert_eq!(item.summarize(), super::POST_SUMMARY);
        }

        notify_bound(post);
        notify_impl(post);
    }

    /// 多少複雑なケースでは impl Trait 構文では対応できないケースもある。
    fn complex_trait_boundary(post: &SocialPost, post2: &SocialPost, article: &NewsArticle) {
        // item1 と item2 は Summary を実装し、かつ両者が同じ型でなければコンパイルが通らない。
        // これは impl Trait 構文では実現できない。
        fn notify_bound_double<T: Summary>(item1: &T, item2: &T) {
            assert_eq!(item1.summarize(), super::POST_SUMMARY);
            assert_eq!(item2.summarize(), "ferris: I'm not Kani");
        }

        // Summary を実装する型であれば item1 と item2 の型は違っていてもいい、というのであれば impl Trait 構文も使える。
        fn notify_impl_double(item1: &impl Summary, item2: &impl Summary) {
            assert_eq!(item1.summarize(), super::POST_SUMMARY);
            assert_eq!(item2.summarize(), super::ARTICLE_SUMMARY);
        }

        notify_impl_double(post, article); // 引数は両方とも Summary を実装していれば型は違っていい
        // notify_bound_double(post, article); // これはコンパイルエラー mismatched types
        notify_bound_double(post, post2); // 引数は両方とも同じ型（実体は違くてもいい）
    }

    // 複数のトレイト境界
    fn multiple_trait_boundary(article: &NewsArticle, _post: &SocialPost) {
        // 複数のトレイト境界を `+` で指定する。引数は両方を実装しなくてはならなくなる。
        fn notify_bound_multi<T: Summary + Debug>(item: &T) {
            assert_eq!(item.summarize(), super::ARTICLE_SUMMARY);
        }

        // 糖衣構文ver
        fn notify_impl_multi(item: &(impl Summary + Debug)) {
            assert_eq!(item.summarize(), super::ARTICLE_SUMMARY);
        }

        // NewsArticle は Debug を実装しているが、SocialPost は実装していない。
        // notify_bound_multi(_post); // `trait_implement::SocialPost` doesn't implement `std::fmt::Debug`
        notify_bound_multi(article);
        notify_impl_multi(article);
    }

    // 複雑なトレイト境界には where 句を使うとスッキリする。
    fn where_trait_boundary() {
        // トレイト境界が複雑だと、関数名と引数リストの間が長くなって読みづらい。
        fn some_function_boundary<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
            let _ = u.clone();
            assert_eq!(t.to_string(), "123");
        }

        // where 句を使うと、複雑なジェネリック型の引数を持つ関数のシグネチャが読みやすくなる。
        fn some_function_where<T, U>(t: &T, u: &U)
        where
            T: Display + Clone,
            U: Clone + Debug,
        {
            let _ = u.clone();
            assert_eq!(t.to_string(), "123");
        }

        some_function_boundary(&123, &"123");
        some_function_where(&123, &"123");
    }

    pub fn run() {
        let post = SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            repost: false,
        };
        let post2 = SocialPost {
            username: String::from("ferris"),
            content: String::from("I'm not Kani"),
            reply: false,
            repost: false,
        };
        let article = NewsArticle {
            headline: "Penguins win the Stanley Cup Championship!".to_string(),
            location: "Pittsburgh, PA, USA".to_string(),
            author: "Iceburgh".to_string(),
            content: "The Pittsburgh Penguins once again are the best hockey team in the NHL."
                .to_string(),
        };

        simple_trait_boundary(&post);
        complex_trait_boundary(&post, &post2, &article);
        multiple_trait_boundary(&article, &post);
        where_trait_boundary();
    }
}

/// 戻り値が impl Trait
mod return_trait {
    use crate::trait_implement::{NewsArticle, SocialPost, Summary};

    // トレイトを実装する型を返す。
    fn returns_summarizable_p() -> impl Summary {
        SocialPost {
            username: "horse_ebooks".to_string(),
            content: "of course, as you probably already know, people".to_string(),
            reply: false,
            repost: false,
        }
    }

    fn _returns_summarizable_a() -> impl Summary {
        NewsArticle {
            headline: "Penguins win the Stanley Cup Championship!".to_string(),
            location: "Pittsburgh, PA, USA".to_string(),
            author: "Iceburgh".to_string(),
            content: "The Pittsburgh Penguins once again are the best hockey team in the NHL."
                .to_string(),
        }
    }

    enum SummaryWrapper {
        NewsArticle(NewsArticle),
        SocialPost(SocialPost),
    }

    // トレイトを実装している型を返す場合でも、2種類以上の型を返す場合はエラー
    // これを行うにはトレイトオブジェクトを使った動的ディスパッチが必要（chapter17_2）。
    // fn returns_summarizable_switch(switch: bool) -> impl Summary {
    //     if switch {
    //         _returns_summarizable_a()
    //     } else {
    //         returns_summarizable_p()
    //     }
    // }

    // 同じ trait を実装する異なる型を impl Trait で返すことはできないが、
    // 列挙体を使えば「どちらかを返す」というのだけは表現できる。
    fn returns_summarizable_wrap(switch: bool) -> SummaryWrapper {
        if switch {
            SummaryWrapper::NewsArticle(NewsArticle {
                headline: "Penguins win the Stanley Cup Championship!".to_string(),
                location: "Pittsburgh, PA, USA".to_string(),
                author: "Iceburgh".to_string(),
                content: "The Pittsburgh Penguins once again are the best hockey team in the NHL."
                    .to_string(),
            })
        } else {
            SummaryWrapper::SocialPost(SocialPost {
                username: "horse_ebooks".to_string(),
                content: "of course, as you probably already know, people".to_string(),
                reply: false,
                repost: false,
            })
        }
    }

    pub fn run() {
        // Summary で定義されている summarize メソッドは使えるが、元の SocialPost のフィールド（usernameなど）へのアクセスはできない
        let summary = returns_summarizable_p();
        assert_eq!(summary.summarize(), super::POST_SUMMARY);

        // 列挙体を使えば型に応じた処理の切替は可能だが、ダックタイピングのようなことはできない。
        let summary = returns_summarizable_wrap(false);
        match summary {
            SummaryWrapper::NewsArticle(article) => {
                assert_eq!(article.summarize(), super::ARTICLE_SUMMARY)
            }
            SummaryWrapper::SocialPost(post) => {
                assert_eq!(post.summarize(), super::POST_SUMMARY)
            }
        }
    }
}

// トレイト境界で PartialOrd を使用
mod trait_ord {
    fn use_largest() {
        // トレイト境界に PartialOrd を含めて、大なり演算子(>)や小なり演算子(<)での比較をできるようにする。
        // 値を返そうとすると Copy または Clone も必要になる。
        fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest = list[0];
            for &item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        // 参照を返すのであれば Copy も Clone も不要
        fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
            let mut largest = &list[0];
            for item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];
        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&number_list);
        assert_eq!(result, 100);
        let result = largest(&char_list);
        assert_eq!(result, 'y');

        let result = largest_ref(&number_list);
        assert_eq!(*result, 100);
        let result = largest_ref(&char_list);
        assert_eq!(*result, 'y');
    }

    // トレイト境界を使ってメソッド実装を条件分け
    fn conditionally_implement() {
        struct Pair<T> {
            x: T,
            y: T,
        }

        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }

        // T が PartialOrd のトレイト境界を満たす場合のみ、Pair<T> に largest メソッドを実装する。
        impl<T: PartialOrd> Pair<T> {
            fn largest(&self) -> &T {
                if self.x >= self.y { &self.x } else { &self.y }
            }
        }

        // i32 は PartialOrd を実装しているため、largest メソッドが実装されている。
        let int_pair = Pair::new(1, 2);
        assert_eq!(int_pair.largest(), &2);

        // &Pair<i32> は PartialOrd を実装していないため、largest メソッドは実装されていない。
        let _mix_pair = Pair::new(&int_pair, &int_pair);
        // assert_eq!(_mix_pair.largest(), &int_pair); // the method `largest` exists for struct `Pair<&Pair<{integer}>>`, but its trait bounds were not satisfied
    }

    pub fn run() {
        use_largest();
        conditionally_implement();
    }
}

/// ブランケット実装
mod blanket_implementation {
    use std::fmt::{Display, Formatter, Result};

    struct Wrapper<T>(T);

    trait HelloTrait {
        fn say_hello(&self) -> &str;
    }

    // トレイト境界を満たすあらゆる型にトレイトを実装することを blanket implementation という。
    // 標準ライブラリで広く使用されている。
    // HelloTrait は Clone を実装している型全てに対して実装され、say_hello メソッドを呼べるようになる。
    impl<T: Clone> HelloTrait for T {
        fn say_hello(&self) -> &str {
            "Hello from a Clone-able type!"
        }
    }

    // orphan rule によりこれはできない（T が外部の型も含むため）。
    // impl<T: HelloTrait> Display for T {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         write!(f, "format: {}", self.say_hello())
    //     }
    // }

    // orphan rule 回避のためには、ローカルの型を定義して、それに対して外部の型を実装する。
    impl<T: HelloTrait> Display for Wrapper<T> {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "format: {}", self.say_hello())
        }
    }

    pub fn run() {
        // プリミティブ型をレシーバとして say_hello メソッドを呼べる。
        assert_eq!(1.say_hello(), "Hello from a Clone-able type!");

        assert_eq!(
            Wrapper(1).to_string(),
            "format: Hello from a Clone-able type!"
        );
    }
}

fn main() {
    struct_implement::run();
    trait_import::run();
    orphan::run();
    default_implementation::run();
    trait_boundary::run();
    return_trait::run();
    trait_ord::run();
    blanket_implementation::run();
}
