//! keywords
//! - closure

use std::env;

use rand::Rng;

mod closure {
    // 関数とクロージャ記法の比較。全て同じ意味。
    // クロージャでは型推論が効くため、引数にも戻り値にも注釈が不要なことが多い。
    fn use_closure() {
        // 通常の関数
        fn _add_one_v1(x: u32) -> u32 {
            x + 1
        }
        let _add_one_v2 = |x: u32| -> u32 { x + 1 }; // 引数と戻り値に注釈
        let _add_one_v3 = |x: u32| x + 1; // 引数だけ注釈。{} は不要。
        let _add_one_v4 = |x| -> u32 { x + 1 }; // 戻り値だけ注釈
        let _add_one_v5 = |x| x + 1; // 注釈なしで {} もなし
        _add_one_v4(1u32);
        _add_one_v5(1u32);
    }

    // クロージャを最初に呼び出した時点で、コンパイラが型を推論する。一度決定すると同じクロージャを異なる型で使用することはできない。
    fn closure_infer() {
        let id = |x| x;
        assert_eq!(id("hello"), "hello");
        // id(5); // これはコンパイルエラー
    }

    pub fn main() {
        use_closure();
        closure_infer();
    }
}

mod expensive {
    use std::{thread, time::Duration};

    pub fn main(simulated_user_specified_value: u32, simulated_random_number: u32) {
        if simulated_random_number < 3 {
            println!("workout initial");
            generate_workout_initial(simulated_user_specified_value, simulated_random_number);
        } else {
            println!("workout closure");
            generate_workout_closure(simulated_user_specified_value, simulated_random_number);
        }
    }

    // 時間のかかる計算
    fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    }

    // ユーザーが選択するトレーニングの強度を高強度か低強度かによって処理を分ける。
    // 動作はするが、以下の問題がある。
    // - 低強度時に simulated_expensive_calculation(intensity) を2回呼び出している。
    // - ↑を解消するために intensity < 25 のブロックで1回だけ呼ぶようにして結果を変数に持つとしても、
    //   高強度のブロックで simulated_expensive_calculation(intensity) を呼んでおり、
    //   仮に simulated_expensive_calculation に変更がある場合、両方変更しなければならなくなる。
    fn generate_workout_initial(intensity: u32, random_number: u32) {
        // 強度25未満かそれ以上かで分岐
        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                simulated_expensive_calculation(intensity)
            );
            println!(
                "Next, do {} situps!",
                simulated_expensive_calculation(intensity)
            );
        } else if random_number == 3 {
            // 乱数値が3であれば休憩を勧める。
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }

    // simulated_expensive_calculation 関数をクロージャに書き換える。
    fn generate_workout_closure(intensity: u32, random_number: u32) {
        // simulated_expensive_calculation の重複した呼び出しをクロージャに抽出する。
        // クロージャでは引数や戻り値の型を注釈する必要はない。
        let expensive_closure = |num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        };

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_closure(intensity));
            println!("Next, do {} situps!", expensive_closure(intensity));
        } else if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

mod lazy {
    use std::{thread, time::Duration};

    // クロージャと Option の結果値を保持する構造体
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        // 引数は value を取得するためのクロージャ。
        // 最初は値がないので value フィールドは None になる。
        fn new(calculation: T) -> Self {
            Self {
                calculation,
                value: None,
            }
        }

        // value フィールドに値があればそれを返し、なければクロージャを実行して値を value フィールドに保持し、それを返す。
        fn value(&mut self, arg: u32) -> u32 {
            self.value.unwrap_or_else(|| {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            })
        }
    }

    // メモ化（あるいは遅延評価）によるリファクタ。
    // 高価な計算は最大1回だけで、必要なければ全く行われなくなる。
    pub fn generate_workout(intensity: u32, random_number: u32) {
        // Cacher インスタンス（value フィールドは None）を作成する。
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

        if intensity < 25 {
            // value メソッドで高価な計算がされ、その値を返す。
            println!("Today, do {} pushups!", expensive_result.value(intensity));
            // value メソッド2回目の呼び出しは、キャッシュした value の値が即座に返される。
            println!("Next, do {} situps!", expensive_result.value(intensity));
        } else if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

mod general {
    use std::{collections::HashMap, hash::Hash};

    // lazy::Cacher を一般化。
    // calculation は1引数の任意のクロージャ、stores は calculation の引数を key、戻り値を value とする HashMap.
    // クロージャの引数や HashMap のキーとする K は、所有権を奪ったり Clone をしないように、参照型にする。
    // V はクロージャによって生成された値で、クロージャからはムーブされて HashMap が所有権を持つ。
    struct Cacher<'a, F, K, V>
    where
        F: Fn(&K) -> V,
        K: Eq + Hash,
    {
        calculation: F,
        stores: HashMap<&'a K, V>,
    }

    impl<'a, F, K, V> Cacher<'a, F, K, V>
    where
        F: Fn(&K) -> V,
        K: Eq + Hash,
    {
        fn new(calculation: F) -> Self {
            Self {
                calculation,
                stores: HashMap::new(),
            }
        }

        fn value(&mut self, arg: &'a K) -> &V {
            if !self.stores.contains_key(arg) {
                let v = (self.calculation)(arg);
                self.stores.insert(arg, v);
            }
            self.stores.get(arg).unwrap()
        }
    }

    pub fn int_cache() {
        // キーの値を2倍した値が戻り値となり、それを HashMap に保持する関数。
        let mut cacher = Cacher::new(|key| {
            println!("key: {key}");
            *key * 2
        });

        assert_eq!(cacher.value(&1), &2);
        println!("Q1 end");
        assert_eq!(cacher.value(&2), &4);
        println!("Q2 end");
        assert_eq!(cacher.value(&1), &2);
        println!("Q3 end");
    }

    pub fn string_cache() {
        // キーの長さが戻り値となり、それを HashMap に保持する関数。
        let mut cacher = Cacher::new(|key: &String| {
            println!("key: {key}");
            key.len()
        });

        let key1 = String::from("foo");
        let key2 = String::from("foobar");
        assert_eq!(cacher.value(&key1), &3);
        println!("Q1 end");
        assert_eq!(cacher.value(&key1), &3);
        println!("Q2 end");
        assert_eq!(cacher.value(&key2), &6);
        println!("Q3 end");
    }

    pub fn str_cache() {
        // &str そのままを key に使えず、&&str とする必要がある。
        let mut cacher = Cacher::new(|key: &&str| {
            println!("key: {key}");
            key.len()
        });

        assert_eq!(cacher.value(&"rust"), &4);
        println!("Q1 end");
        assert_eq!(cacher.value(&"go"), &2);
        println!("Q2 end");
        assert_eq!(cacher.value(&"go"), &2);
        println!("Q3 end");
    }
}

mod capture {
    pub fn run() {
        immutable();
        mutable();
        once();
    }

    fn immutable() {
        let x = 4;
        // x の値をキャプチャする（不変借用）
        let equal_to_x = |z| z == x;

        let y = 4;
        assert!(equal_to_x(y));
    }

    fn mutable() {
        let mut x = 4;
        // x の値をキャプチャ（可変借用）。クロージャも可変にする必要がある。
        let mut increment_x = || x += 1;
        increment_x();
        assert_eq!(x, 5);
    }

    fn once() {
        let x = vec![1, 2, 3];
        // move キーワードを使用することで x をムーブすることを強制
        let equal_to_x = move || x;
        // assert_eq!(x, vec![1, 2, 3]); // x はムーブされたためこれはできない。

        assert_eq!(equal_to_x(), vec![1, 2, 3]);
        // assert_eq!(equal_to_x(), vec![1, 2, 3]); // 2回以上 equal_to_x を呼ぶことはできない。
    }
}

// https://doc.rust-lang.org/nomicon/subtyping.html
// https://doc.rust-lang.org/nomicon/phantom-data.html
mod phantom {
    use std::marker::PhantomData;

    /// 構造体フィールドで型引数Tを使っていないとコンパイルエラーになる。
    /// これは、型引数がフィールドにないと変性(variance)が定まらず、型安全性が保証できなくなるためらしい。
    /// それを避けるために PhantomData と呼ばれる特殊な型を用いて、強制的に T を消費する。
    /// なお、PhantomData<T> とすると、T を所有しているとみなされる。特に所有を明示する必要がないのであれば PhantomData<fn() -> T> のほうがいいらしい。
    struct Wrap<F, T>
    where
        F: Fn(T),
    {
        callback: F,
        _marker: PhantomData<fn() -> T>,
    }

    pub fn run() {
        let w1 = Wrap {
            callback: |s| println!("{s}"),
            _marker: PhantomData,
        };
        let w2 = Wrap {
            callback: |s| println!("{s}"),
            _marker: PhantomData,
        };

        let local = "local";

        (w1.callback)(local);
        (w2.callback)(String::from(local));
    }
}

fn main() {
    closure::main();

    // フロントエンドからアプリへの入力値（ここではハードコード）
    let simulated_user_specified_value = 10;
    // アプリが生成する乱数（ここでは適当に0～5）
    let simulated_random_number = rand::rng().random_range(0..=5);

    let args: Vec<_> = env::args().collect();
    let kind = args.get(1).map_or("expensive", |s| s.as_str());

    match kind {
        "expensive" => expensive::main(simulated_user_specified_value, simulated_random_number),
        "lazy" => lazy::generate_workout(simulated_user_specified_value, simulated_random_number),
        "general" => {
            general::int_cache();
            general::string_cache();
            general::str_cache();
        }
        _ => eprintln!("Invalid argument."),
    }

    capture::run();
    phantom::run();
}
