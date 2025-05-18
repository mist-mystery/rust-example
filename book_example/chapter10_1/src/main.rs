mod point1 {
    // 型引数を1つだけ使用。
    // なお、ジェネリックな型はコンパイル時に単相化(Monomorphization)され、コード中で使用している Point<i32> と Point<f64> が生成される。
    // メリットはコンパイル時に静的ディスパッチを行うことで実行時コストがかからないため速度が早くなる。
    // デメリットはバイナリサイズが肥大化する。
    struct Point<T> {
        x: T,
        y: T,
    }

    // 型 Point<T> にメソッドを実装するのであれば、impl の直後に T を宣言する。
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // Point<T> でなく、具体的な型 Point<f32> に対してのみメソッドを実装。
    // ジェネリックな型を持たないので impl の後に型宣言しない。
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    // Point<i32> に対してはこちらを呼び出すことになる。
    // メソッドのオーバーロードが可能（ad-hoc polymorphism）。
    impl Point<i32> {
        fn distance_from_origin(&self) -> i32 {
            (self.x.pow(2) + self.y.pow(2)).isqrt()
        }
    }

    pub fn generic() {
        {
            // ジェネリックな型引数を利用。
            let _integer = Point { x: 5, y: 10 }; // こちらは Point<i32>
            let Point { x: _x, y: _y } = Point { x: 1.0, y: 4.0 }; // こちらは Point<f64>

            // x と y は同じ型でなければならない。
            // let wont_work = Point { x: 5, y: 4.0 };
        }
        {
            // メソッド定義におけるジェネリクス
            let p = Point { x: 5, y: 10 };
            assert_eq!(p.x(), &5);
        }
        {
            // Point<f32> に対しては f32 で返す。
            let p = Point { x: 1.0, y: 4.0 };
            assert_eq!(p.distance_from_origin(), 4.1231055);

            // Point<i32> に対しては i32 で返す。
            let q = Point { x: 1, y: 4 };
            assert_eq!(q.distance_from_origin(), 4);

            // Point<i64> に対しては distance_from_origin メソッドは実装していないため呼べない。
            let _r = Point { x: 1_i64, y: 4_i64 };
            // assert_eq!(_r.distance_from_origin(), 4)
        }
    }
}

mod point2 {
    // 型引数を2つ使用
    struct Point<T, U> {
        x: T,
        y: U,
    }

    // 構造体定義のジェネリックな型引数が、メソッドシグニチャで使う型引数とは一致していなくてもいい。
    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    pub fn generic() {
        {
            // ジェネリックな型引数を複数利用。
            let _integer = Point { x: 5, y: 10 };
            let Point { x: _x, y: _y } = Point { x: 1.0, y: 4.0 };

            // 型引数2つなので、xとyの型が違っていてもOK
            let _wont_work = Point { x: 5, y: 4.0 };
        }
        {
            // mixup を使用
            let p1 = Point { x: 5, y: 10.4 };
            let p2 = Point { x: "Hello", y: 'c' };

            let p3 = p1.mixup(p2);
            assert_eq!(p3.x, 5);
            assert_eq!(p3.y, 'c');
        }
    }
}

fn main() {
    point1::generic();
    point2::generic();
}
