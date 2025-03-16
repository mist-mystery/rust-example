fn main() {
    ownership();
    println!();
    references();
}

fn ownership() {
    {
        let mut mut_s = String::from("hello");
        mut_s.push_str(", world!");
        println!("{}", mut_s);
    }
    {
        let s1 = String::from("hello");
        let s2 = s1;
        let s3 = s2.clone();
        // println!("s1 = {}", s1); // s1 はムーブ後に使用できない
        println!("s2 = {}, s3= {}", s2, s3);
    }
    {
        let s = String::from("hello");
        takes_ownership(s);
        // println!("{}", s); sの値が関数にムーブされ、ここではもう有効でない

        let x = 5;
        makes_copy(x);
        println!("x = {}", x);
    }
    {
        let s1 = gives_ownership(); // gives_ownershipは、戻り値をs1にムーブする
        let s2 = String::from("hello"); // s2がスコープに入る
        let s3 = takes_and_gives_back(s2); // s2はtakes_and_gives_backにムーブされ戻り値もs3にムーブされる
        println!("s1 = {}, s3 = {}", s1, s3);
        // println!("s2 = {}", s2);
    }
    {
        let s1 = String::from("hello");
        let (s1, len) = calculate_length_tuple(s1);
        println!("The length of '{}' is {}.", s1, len);
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("some_integer = {}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // some_stringが返され、呼び出し元関数にムーブされる
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_stringが返され、呼び出し元関数にムーブされる
}

fn calculate_length_tuple(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返します
    (s, length)
}

fn references() {
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1); // 参照を渡すため所有権が移らない
        println!("The length of '{}' is {}.", s1, len);
    }
    {
        let mut s = String::from("hello");
        change(&mut s);
        println!("change_s = {}", s);
    }
    {
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            // let r2 = &mut s; // sを可変として2回以上借用することはできない
            r1.push_str(", ");
        } // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる
        println!("s = {}", s); // "hello, "

        let r2 = &mut s;
        r2.push_str("world");
        // println!("s = {}", s); // sは可変参照として借用されているため、借用元のsにアクセスすることはできない
        println!("r2 = {}", r2); // "hello, world"
    }
    {
        let s = String::from("hello");
        {
            let m = s;
            println!("m = {}", m); // "hello, "
        }
        // println!("s = {}", s); // sの値は（スコープ外でだが）ムーブされているため使用不可
    }
}

// sはStringへの参照
fn calculate_length(s: &String) -> usize {
    s.len()
} // ここで、sはスコープ外になる。けど、参照しているものの所有権を持っているわけではないので何も起こらない

// 可変参照を受け取ることで、借用した値を変更することが可能
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
