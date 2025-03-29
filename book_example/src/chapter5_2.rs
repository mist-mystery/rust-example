pub fn main() {
    variables();
    tuple();
    rectangle();
    name();
    rect_debug();
}

// 変数に幅と高さを持たせる
fn variables() {
    let width1 = 30;
    let height1 = 50;

    assert_eq!(area_variables(width1, height1), 1500);
}

fn area_variables(width: u32, height: u32) -> u32 {
    width * height
}

// タプルに幅と高さを持たせる。タプルにどちらが幅でどちらが高さという情報がないという問題点がある。
fn tuple() {
    let rect1 = (30, 50);

    assert_eq!(area_tuple(rect1), 1500);
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
    println!("rect1 is {rect1:?}");

    assert_eq!(area_rectangle(&rect1), 1500);
}

// Rectangle インスタンスの不変借用。
// この関数は Rectangle と緊密に結びついているため、メソッドに変形したほうがよさそう（chapter5_3）
fn area_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Name {
    first: String,
    last: String,
}

fn name() {
    let name = Name {
        first: String::from("Alice"),
        last: String::from("Howk"),
    };

    assert_eq!(fullname(&name), "Alice Howk"); // 借用
    assert_eq!(fullname_move(name), "Alice Howk"); // ムーブ
    // println!("{:?}", name); // ムーブ後は使えない
}

// Copyt トレイトを持たないフィールドがある構造体の参照を引数で渡す
fn fullname(name: &Name) -> String {
    // let _first = name.first; // 借用している構造体フィールドのムーブはできない
    let _first = &name.first; // これならOK
    let Name {
        first: _first,
        last: _last,
    } = name; // これでもOK

    format!("{} {}", name.first, name.last)
}

fn fullname_move(name: Name) -> String {
    let first = name.first; // ムーブ
    let last = name.last; // ムーブ

    // return format!("{} {}", name.first, name.last); // フィールドのムーブ後にアクセス不可
    format!("{first} {last}")
}

// dbg! マクロを利用して標準エラー出力にデバッグ情報を出力
fn rect_debug() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let _rect2 = dbg!(&rect1); // 所有権を移動しない
    let _rect2 = dbg!(rect1); // 所有権を移動
}
