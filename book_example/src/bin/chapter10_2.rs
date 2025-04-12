//! keywords  
//! - trait
//! - orphan rule  
//!   トレイトの実装の重複を防ぐための規則
//! - default implementation
//! - trait boundary
//! - blanket implementation  
//!   `impt<T: Display> ToString for T { ... }`

const TWEET_SUMMARY: &str = "horse_ebooks: of course, as you probably already know, people";
const ARTICLE_SUMMARY: &str =
    "Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)";

/// トレイトを使わず、Tweet 構造体にメソッドを直接実装する。
mod struct_implement {
    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    impl Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }

        fn is_popular(&self) -> bool {
            self.reply && self.retweet
        }
    }

    pub fn main() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        assert_eq!(tweet.summarize(), super::TWEET_SUMMARY);
        assert!(!tweet.is_popular());
    }
}

/// Summary トレイトを Tweet 構造体に実装する。
/// ここでのトレイトの使い方は、他の言語でのインターフェースに近い。
mod trait_implement {
    use std::fmt::Display;

    trait Summary {
        fn summarize(&self) -> String;
    }

    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    // impl Summary はあくまでも Summary トレイトのメソッドを定義するものであり、
    // ここでトレイトに定義されているメソッド以外を定義することはできない。
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }

        // ここに is_popular メソッドを書くと method `is_popular` is not a member of trait `Summary` のようなエラーとなる。
        // fn is_popular(&self) -> bool {
        //     self.reply && self.retweet
        // }
    }

    // トレイトに定義されてないメソッドは、構造体に対して直接実装する。
    impl Tweet {
        fn is_popular(&self) -> bool {
            self.reply && self.retweet
        }
    }

    // orphan rule により、外部のトレイトを外部の型に対して実装することはできない
    // impl Display for Vec<i32> {
    //     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    //         write!(f, "Custom Vec formatting")
    //     }
    // }

    // 外部のトレイトをローカルの型に対して実装するのはOK
    impl Display for Tweet {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "{}: {}\nreplay: {}, retweet: {}",
                self.username, self.content, self.reply, self.retweet
            )
        }
    }

    // 逆に、ローカルのトレイトに対して外部の型を実装するのもOK
    impl Summary for i32 {
        fn summarize(&self) -> String {
            self.to_string()
        }
    }

    pub fn main() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        assert_eq!(tweet.summarize(), super::TWEET_SUMMARY);

        // 他言語のインターフェースであればこういうことが可能なはずだが、Rust ではできない。
        // これは、コンパイル時に型が決定している必要があるが、インターフェースだけだと決まらないため。
        // let summary: Summary = tweet;

        // これであれば通る。詳しくは英語版の chapter18 で扱うはず。
        let summary: &dyn Summary = &tweet;
        assert_eq!(summary.summarize(), super::TWEET_SUMMARY);

        // let summary: Summary は許されないが、引数をトレイトとする関数の定義は可能（コンパイル時には具体的な型に解決される）。
        arg_trait(&tweet);
        assert!(!tweet.is_popular());

        // Tweet に Display を実装したため、to_string メソッドを呼べる。
        assert_eq!(
            tweet.to_string(),
            format!("{}\nreplay: false, retweet: false", super::TWEET_SUMMARY)
        );

        assert_eq!(1.summarize(), "1");
    }

    // 引数に `impl トレイト` と指定することで、トレイトを実装する何らかの型を受け付ける。
    fn arg_trait(item: &impl Summary) {
        assert_eq!(item.summarize(), super::TWEET_SUMMARY);
    }
}

/// デフォルト実装
mod default_implementation {
    // トレイトにはデフォルト実装を与えることもできる。
    trait SummaryDefault {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    #[allow(unused)]
    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }

    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    impl Tweet {
        fn is_popular(&self) -> bool {
            self.reply && self.retweet
        }
    }

    // Tweet ではデフォルト実装を上書き。
    impl SummaryDefault for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // NewsArticle ではデフォルト実装を使う。
    impl SummaryDefault for NewsArticle {}

    // tweet でも article でも SummaryDefault を実装しているため summarize メソッドを呼べる。
    pub fn main() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        assert_eq!(tweet.summarize(), super::TWEET_SUMMARY);
        assert!(!tweet.is_popular());

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        };
        assert_eq!(article.summarize(), "(Read more...)");
    }
}

mod trait_boundary {
    use std::fmt::{Debug, Display};

    trait Summary {
        fn summarize(&self) -> String;
    }

    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    impl Tweet {
        fn is_popular(&self) -> bool {
            self.reply && self.retweet
        }
    }

    #[derive(Debug, Clone)]
    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub fn main() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        let tweet2 = Tweet {
            username: String::from("ferris"),
            content: String::from("I'm not Kani"),
            reply: false,
            retweet: false,
        };

        {
            // トレイト境界構文
            fn notify_bound<T: Summary>(item: &T) {
                assert_eq!(item.summarize(), super::TWEET_SUMMARY);
            }

            // `item: &impl Trait` はトレイト境界構文の糖衣構文。
            // 単純なケースでならジェネリクスを使わなくていいためこちらのほうが簡潔。
            fn notify_impl(item: &impl Summary) {
                assert_eq!(item.summarize(), super::TWEET_SUMMARY);
            }

            notify_bound(&tweet);
            notify_impl(&tweet);
            assert!(!tweet.is_popular());
        }

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
        };

        {
            // item1 と item2 は Summary を実装し、かつ両者が同じ型でなければコンパイルが通らない。
            // これは impl Trait 構文では実現できない。
            fn notify_bound_double<T: Summary>(item1: &T, item2: &T) {
                assert_eq!(item1.summarize(), super::TWEET_SUMMARY);
                assert_eq!(item2.summarize(), "ferris: I'm not Kani");
            }

            // Summary を実装する型であれば item1 と item2 の型は違っていてもいい、というのであれば impl Trait 構文も使える。
            fn notify_impl_double(item1: &impl Summary, item2: &impl Summary) {
                assert_eq!(item1.summarize(), super::TWEET_SUMMARY);
                assert_eq!(item2.summarize(), super::ARTICLE_SUMMARY);
            }

            notify_impl_double(&tweet, &article); // 引数は両方とも Summary を実装していれば型は違っていい
            // notify_bound_double(&tweet, &article); // これはコンパイルエラー
            notify_bound_double(&tweet, &tweet2); // 引数は両方とも同じ型（実体は違くてもいい）
        }

        {
            // 複数のトレイト境界を `+` で指定する。引数は両方を実装しなくてはならなくなる。
            fn notify_bound_multi<T: Summary + Debug>(item: &T) {
                assert_eq!(item.summarize(), super::ARTICLE_SUMMARY);
            }

            // 糖衣構文ver
            fn notify_impl_multi(item: &(impl Summary + Debug)) {
                assert_eq!(item.summarize(), super::ARTICLE_SUMMARY);
            }

            // notify_bound_multi(&tweet); // tweet は Debug を実装していないためこれはコンパイルエラー
            notify_bound_multi(&article);
            notify_impl_multi(&article);
        }
        {
            // 複雑なトレイト境界
            fn some_function_boundary<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
                let _ = u.clone();
                assert_eq!(format!("{t}"), "123");
            }

            // where 句を使うと、複雑なジェネリック型の引数を持つ関数のシグネチャが読みやすくなる。
            fn some_function_where<T, U>(t: &T, u: &U)
            where
                T: Display + Clone,
                U: Clone + Debug,
            {
                let _ = u.clone();
                assert_eq!(format!("{t}"), "123");
            }

            some_function_boundary(&123, &"123");
            some_function_where(&123, &"123");
        }
    }
}

mod return_trait {
    trait Summary {
        fn summarize(&self) -> String;
    }

    struct Tweet {
        username: String,
        content: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    // トレイトを実装する型を返す。
    fn returns_summarizable_t() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
        }
    }

    fn _returns_summarizable_a() -> impl Summary {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
        }
    }

    // トレイトを実装している型を返す場合でも、2種類以上の型を返す場合はエラー
    // これを行うにはトレイトオブジェクトを使った動的ディスパッチが必要（chapter17_2）。
    // fn returns_summarizable_switch(switch: bool) -> impl Summary {
    //     if switch {
    //         returns_summarizable_a()
    //     } else {
    //         returns_summarizable_t()
    //     }
    // }

    enum SummaryWrapper {
        NewsArticle(NewsArticle),
        Tweet(Tweet),
    }

    // Summerize を実装した列挙子のみを持つ列挙体を返す
    fn returns_summarizable_wrap(switch: bool) -> SummaryWrapper {
        if switch {
            SummaryWrapper::NewsArticle(NewsArticle {
                headline: String::from("Penguins win the Stanley Cup Championship!"),
                location: String::from("Pittsburgh, PA, USA"),
                author: String::from("Iceburgh"),
            })
        } else {
            SummaryWrapper::Tweet(Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
            })
        }
    }

    pub fn main() {
        // summarize メソッドは使えるが、元の Tweet のフィールド（usernameなど）へのアクセスはできない
        let summary = returns_summarizable_t();
        assert_eq!(summary.summarize(), super::TWEET_SUMMARY);

        // 列挙体を使えば型の切替自体は可能だが、ダックタイピングのようなことはできない。
        let summary = returns_summarizable_wrap(false);
        match summary {
            SummaryWrapper::NewsArticle(article) => {
                assert_eq!(article.summarize(), super::ARTICLE_SUMMARY)
            }
            SummaryWrapper::Tweet(tweet) => assert_eq!(tweet.summarize(), super::TWEET_SUMMARY),
        }
    }
}

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

    fn conditionally_implement() {
        // トレイト境界を使ってメソッド実装を条件分け
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
        // assert_eq!(_mix_pair.largest(), &int_pair);
    }

    pub fn main() {
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

    pub fn main() {
        // プリミティブ型をレシーバとして say_hello メソッドを呼べる。
        assert_eq!(1.say_hello(), "Hello from a Clone-able type!");

        assert_eq!(
            Wrapper(1).to_string(),
            "format: Hello from a Clone-able type!"
        );
    }
}

fn main() {
    trait_implement::main();
    struct_implement::main();
    default_implementation::main();
    trait_boundary::main();
    return_trait::main();
    trait_ord::main();
    blanket_implementation::main();
}
