fn main() {
    use_iter_for();
    iter_next();
    iterator_sum();
    iterator_adaptors();
    iterator_closure();
    my_iterator::calling_next_directly();
    my_iterator::using_other_iterator_trait_methods();
}

// for ループでイテレータを使用
fn use_iter_for() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // Vec をループ
    for val in &v1 {
        println!("Got: {val}");
    }

    // 配列から生成したイテレータをループ
    // 内部的に v1_iter の所有権を奪い、可変にしてループする。
    for val in v1_iter {
        println!("Got: {val}");
    }

    // イテレータを for ループで使用するとムーブされるため、以降は使用不可
    // assert_eq!(v1_iter.sum::<i32>(), 6);
    // 元の Vec があればもう1回イテレータを生成するのはできる
    let v1_iter2 = v1.iter();
    assert_eq!(v1_iter2.sum::<i32>(), 6);
}

// イテレータの next メソッドを使用
fn iter_next() {
    {
        let v1 = [1, 2, 3];
        let mut v1_iter = v1.iter();

        // next メソッドはイテレータの内部状態を変化されるため可変である必要がある。
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
        assert_eq!(v1_iter.next(), None);
    }
    {
        // 可変なイテレータを作成
        let mut v1 = [1, 2, 3];
        let mut v1_iter_mut = v1.iter_mut();
        let next = v1_iter_mut.next();
        if let Some(val) = next {
            *val += 10
        }
        // イテレータから取り出した値の変更が、元となった配列に反映される。
        assert_eq!(v1.first(), Some(&11));
    }
    {
        let v1 = vec![1, 2, 3];
        let mut v1_iter_into = v1.into_iter();

        // into_iter で値の所有権がイテレータに移ったため、v1 はもう使えない。
        // assert_eq!(v1.first(), Some(&1));

        assert_eq!(v1_iter_into.next(), Some(1));
    }
}

// 消費アダプタ（next メソッドを呼び出すイテレータのメソッド）
fn iterator_sum() {
    let v1 = [1, 2, 3];
    let v1_iter = v1.iter();

    // sum メソッドはイテレータの所有権を奪う
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
    // v1_iter はもう使用できない
    // assert_eq!(v1_iter.sum::<i32>(), 6);
}

// イテレータアダプタ（イテレータを別の種類のイテレータに変更する）
fn iterator_adaptors() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // 生成したイテレータは collect() などで消費する必要がある。
    let v2: Vec<_> = v1_iter.map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    // map メソッドで v1_iter はムーブされる（わざわざ v1_iter を定義せず v1.iter().map(...) のようにするのが普通）
    // assert_eq!(v1_iter.sum::<i32>(), 6);
}

// filter イテレータアダプタを使って環境をキャプチャする。
fn iterator_closure() {
    #[derive(PartialEq, Debug)]
    // assert_eq! で比較するのに PartialEq と Debug の注釈が必要。
    struct Shoe {
        size: u32,
        style: String,
    }

    // shoes はムーブされる。
    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        // shoes の所有権を奪ってイテレータを作成。
        // shoe_size をキャプチャし、size が shoe_size と等しいもののみ残す。型は戻り値の型となる。
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    // ここで shoes の所有権は奪われる
    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}

// 独自イテレータを作成する。
mod my_iterator {
    // count フィールドは非公開にして、Counter の実装にその値を管理させるようにする。
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Self {
            Self { count: 0 }
        }
    }

    // Counter 型に対して Iterator を実装する。
    // Item 関連型の設定及び next メソッドの実装が必要となる。
    impl Iterator for Counter {
        // Item 関連型を u32 に設定する。
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    pub fn calling_next_directly() {
        let mut counter = Counter::new();

        // next メソッドを呼び出して、順番に値を返す。
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None); // 6以上の値は返さず、None を返す。
    }

    // イテレータを実装すれば、他の Iterator トレイトメソッド(デフォルト実装)は全て呼び出し可能になる。
    pub fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}
