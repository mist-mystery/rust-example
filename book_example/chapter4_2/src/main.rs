//! keywords
//! - several immutable references (&T): aliasing
//! - one mutable reference (&mut T): mutability

fn main() {
    {
        // （複数の）不変な参照
        let s1 = String::from("hello");
        let len = calculate_length(&s1); // 参照を渡すため所有権が移らない。String のまま渡そう（ムーブしよう）とすると型エラー
        println!("The length of '{s1}' is {len}.");

        let s1 = String::from("hello");
        let s2 = return_ref(&s1); // 参照をそのまま返すこともできる
        let s3 = &s1; // 不変参照は複数可能
        println!("immutable references {s1} {s2} {s3}"); // s2, s3 は自動でデリファレンスされる（format! の機能）
    }
    {
        // 一つの可変な参照
        let mut s = String::from("hello");
        change(&mut s);
        println!("change_s = {s}");

        let mut s1 = String::from("hello");
        let s2 = change_return_ref(&mut s1);
        // println!("s1 = {s1}"); // s1 は可変として借用されてるのでここで immutable に借用することはできない（後述）
        println!("change_return_s2 = {s2}");
    }
    {
        // 借用規則
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            // let r2 = &mut s; // sを可変として2回以上借用することはできない
            r1.push_str(", ");
        } // r1はここでスコープを抜けるので、問題なく s の新しい参照を作ることができる
        println!("s = {s}"); // "hello, "
        let r2 = &s;
        let r3 = &s;
        println!("{r2} {r3}");

        // r2, r3 で不変借用しているが、これ以降 r2 と r3 を使用していないので、ここで可変借用しても問題ない
        let message = &mut s;
        message.push_str("world");
        println!("message = {message}"); // "hello, world"

        // sは可変として借用されているため、可変借用が Drop されるまで s を読み取りも再借用もできない。
        // ※ println! は内部的に s を不変借用しているため、可変借用されている変数を println! で表示するのはできない
        // println!("s = {}", s);

        let moved: String = s;
        println!("moved = {moved}");
        // println!("moved message = {message}"); // 借用元変数をムーブすると、借用している側も使えなくなる
    }
    {
        // Dangling References
        // println!("{}", dungle()); // ダングリング参照を返す関数はコンパイルエラーとなる
        println!("{}", no_dungle());
    }
}

#[allow(clippy::ptr_arg)] // ここでの引数は &str のほうが受け入れ範囲が広い
// sはStringへの参照。
fn calculate_length(s: &String) -> usize {
    s.len()
} // ここで、sはスコープ外になる。けど、参照しているものの所有権を持っているわけではないので何も起こらない

fn return_ref(s: &String) -> &String {
    s
}

// 可変参照を受け取ることで、借用した値を変更することが可能
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn change_return_ref(s: &mut String) -> &String {
    s.push_str(", world");
    s
}

// dangleはStringへの参照を返す（実際はコンパイルエラーとなる）
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // String sへの参照を返す
// } // ここで、sはスコープを抜け、ドロップされる。そのメモリは消される。

// String を直接返すとムーブされ、メモリは解放されない
fn no_dungle() -> String {
    let s = String::from("no_dungle");
    #[allow(clippy::let_and_return)]
    s
}
