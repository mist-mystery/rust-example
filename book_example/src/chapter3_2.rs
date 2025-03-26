use std::io;

// 定数はグローバルスコープでも定義できる。値の方は必ず注釈する必要がある。
const MAX_POINTS: u32 = 100_000;

fn variable() {
    let spaces = "   ";
    let spaces = spaces.len(); // shadowing. 型が違ってても問題ない
    {
        let spaces = spaces * 2; // shadowing （このスコープ内でのみ有効）
        println!("The value of spaces in the inner scope is: {}", spaces);
    }
    println!("The value of spaces is: {}", spaces); // {} を抜けると shadowing が終了し、その前の値に戻る

    println!("Max Point is: {}", MAX_POINTS);
}

fn data_type() {
    // 数値演算。i32 及び f64 が基準型
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // i32 同士の除算は i32、すなわち結果は0になる。
    // let _floored = 2 / 3.0; // 異なる型での除算は許されていない
    let _remainder = 43 % 5;

    // 論理値型
    let _t = true;
    let _f = false;

    // 文字型、文字列型
    let _c = '😻'; // char型リテラルはシングルクォートで囲む。Unicodeのスカラー値。
    let _str = "";

    // タプル型
    let tup = (500, 6.4, 1);
    let (_x, _y, _) = tup; // パターンマッチング
    let _five_hundred = tup.0; // タプルの要素に直接アクセス

    // 配列。全要素は同じ型で、固定長。
    let _a = [3; 5]; // 角括弧内に初期値と長さをセミコロンを挟んで与えると、各要素に初期値が入って初期化される。 
    let a = [1, 2, 3, 4, 5];
    let _first = a[0]; // 配列に添え字アクセスできる
    // let _over = a[5]; // 配列の要素を超えてアクセスしようとしてもコンパイルエラーにはならないが、実行時エラー

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

pub fn main() {
    variable();
    data_type();
}
