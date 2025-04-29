struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // メソッドの最初の引数は必ず self で、これはメソッドが呼び出されている構造体インスタンス。
    // `&self` は実際は `self: &Self` の短縮記法で、Self は impl ブロックの対象となる型（今回は Rectangle）のエイリアスである。
    // &self とすることで不変借用している。通常の関数と同様、&mut self であれば可変借用、& をつけなければ所有権を奪うことになる。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 構造体フィールドと同名のメソッドを定義できる。
    // 多くの場合（常にではない）、フィールドと同名のメソッドは getter として定義する。
    fn width(&self) -> bool {
        self.width > 0
    }

    // 別の Rectangle インスタンスを引数として取るメソッド
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Functions (関連関数)
    // 最初のパラメータに self を持たない（ゆえにメソッドではない）。
    // コンストラクタとして使う場合は `new` が使われることが多いが、new は特別な名前ではない。
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// 同じ構造体に対して impl ブロックは複数あっても問題ない。
impl Rectangle {
    // 可変参照
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    #[allow(clippy::needless_borrow)]
    {
        let rect = Rectangle {
            width: 30,
            height: 50,
        };

        // receiver に & をつけなくても、自動参照及び参照外しによって、
        // メソッドのシグニチャに一致するようにコンパイラが自動で `&`, `&mut`, `*` のいずれかを付与する。
        assert_eq!(rect.area(), 1500);
        assert_eq!((&rect).area(), 1500); // つけても問題ない

        // 冗長だが、メソッド記法を使わず関連関数で書いても問題はない。
        // この場合、自動参照及び参照外しは行われない。
        assert_eq!(Rectangle::area(&rect), 1500);
    }
    {
        let mut rect_m = Rectangle {
            width: 30,
            height: 50,
        };

        rect_m.set_width(100);

        // 括弧ありなしで同名（width）のフィールドとメソッドを区別
        if rect_m.width() {
            assert_eq!(rect_m.width, 100);
            assert_eq!(rect_m.area(), 5000);
        }
    }
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle::square(60);

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }
}
