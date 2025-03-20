fn main() {
    variables();
    tuple();
    rectangle();
    method();
}

// 変数に幅と高さを持たせる
fn variables() {
    let width1 = 30;
    let height1 = 50;

    println!(
        // 長方形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels.",
        area_variables(width1, height1)
    );
}

fn area_variables(width: u32, height: u32) -> u32 {
    width * height
}

// タプルに幅と高さを持たせる。タプルにどちらが幅でどちらが高さという情報がないという問題点がある。
fn tuple() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn rectangle() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Rectangle 構造体に #[derive(Debug)] の注釈が必要
    println!("rect1 is {:?}", rect1);

    // area_rectangle が Rectangle を引数とするなら、参照（借用）でなくムーブされる
    println!(
        "The area of the rectangle is {} square pixels.",
        area_rectangle(&rect1)
    );
}

fn area_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    // 不変借用。& をつけなければ所有権を奪うことになる。
    // receiver に & をつけなくても(`(&rect1).width` のような呼び出しをしなくても)コンパイラが自動で判断する。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 構造体フィールドと同名のメソッドを定義できる。
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Functions
    // コンストラクタとして使う場合は `new` が使われることが多いが、それ以外でも問題ない。
    // impl キーワードの後の型を表すのに `Self` キーワードが使われる。
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    // 可変参照
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn method() {
    {
        let mut rect_m = Rectangle {
            width: 30,
            height: 50,
        };
        let rect = &rect_m;

        println!(
            "The area of the rectangle is {} square pixels.",
            rect.area()
        );
        rect_m.set_width(100);

        if rect_m.width() {
            println!("The rectangle has a nonzero width; it is {}", rect_m.width);
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
    {}
}
