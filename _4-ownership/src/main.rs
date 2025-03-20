fn main() {
    ownership();
    println!();
    references();
    println!();
    slice_type();
}

// 4.1
fn ownership() {
    let hello = "hello";
    {
        // String は mutable にできる
        let mut mut_s = String::from(hello);
        mut_s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える
        println!("{}", mut_s);
    }
    {
        // 変数のムーブ・コピー
        let s1 = String::from(hello);
        let s2 = s1; // ムーブ
        let s3 = s2.clone();
        // println!("s1 = {}", s1); // s1 はムーブ後に使用できない
        println!("s2 = {}, s3= {}", s2, s3);

        let s = String::from(hello);
        {
            let m = s;
            println!("m = {}", m); // "hello"
        }
        // println!("s = {}", s); // sの値は（スコープ外でだが）ムーブされているため使用不可
    }
    {
        // 関数の引数のムーブ
        let s = String::from(hello);
        takes_ownership(s);
        // println!("{}", s); sの値が関数にムーブされ、ここではもう有効でない

        let x = 5;
        makes_copy(x); // x も関数にムーブされるが、i32はcopyなので、この後にxを使っても大丈夫
        println!("x = {}", x);
    }
    {
        // 戻り値のムーブ
        let s1 = gives_ownership(); // gives_ownershipは、戻り値をs1にムーブする
        let s2 = String::from(hello); // s2がスコープに入る
        let s3 = takes_and_gives_back(s2); // s2はtakes_and_gives_backにムーブされ戻り値もs3にムーブされる
        println!("s1 = {}, s3 = {}", s1, s3);
        // println!("s2 = {}", s2); // s2 はムーブ後に使用できない
    }
    {
        // タプルで複数の値を返す
        let s1 = String::from(hello);
        let (s2, len) = calculate_length_tuple(s1);
        // println!("s1 = {}", s1);                     // s1 はムーブ後に使用できないが、
        println!("The length of '{}' is {}.", s2, len); // s1 で渡したものを s2 として受け取ることで再利用はできる。ただし、後述する借用を使う方がよい。
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) {
    println!("some_integer = {}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

// gives_ownershipは、戻り値を呼び出した関数にムーブする
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

// 4.2
fn references() {
    {
        // （複数の）不変な参照
        let s1 = String::from("hello");
        let len = calculate_length(&s1); // 参照を渡すため所有権が移らない
        println!("The length of '{}' is {}.", s1, len);

        let s1 = String::from("hello");
        let s2 = return_ref(&s1); // 参照をそのまま返すこともできる
        let s3 = &s1; // 不変参照は複数可能
        println!("immutable references {} {} {}", s1, s2, s3); // s1は String ,s2とs3は &String だが、"hello hello hello" と表示される
    }
    {
        // 一つの可変な参照
        let mut s = String::from("hello");
        change(&mut s);
        println!("change_s = {}", s);

        let mut s1 = String::from("hello");
        let s2 = change_return_ref(&mut s1);
        // println!("s1 = {}", s1); // s1 は mutable として借用されてるのでここで immutable に借用することはできない（後述）
        println!("change_return_s2 = {}", s2);
    }
    {
        // 借用規則
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            // let r2 = &mut s; // sを可変として2回以上借用することはできない
            r1.push_str(", ");
        } // r1はここでスコープを抜けるので、問題なく s の新しい参照を作ることができる
        println!("s = {}", s); // "hello, "

        let r2 = &mut s;
        r2.push_str("world");
        // println!("s = {}", s); // sは可変参照として借用されているため、借用元のsにアクセスすることはできない
        println!("r2 = {}", r2); // "hello, world"
    }
    {
        // Dangling References
        // println!("{}", dungle()); // ダングリング参照を返す関数はコンパイルエラーとなる
        println!("{}", no_dungle());
    }
}

// sはStringへの参照
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

fn no_dungle() -> String {
    let s = String::from("no_dungle");
    s
}

// 4.3
fn slice_type() {
    let hello_world = "hello world";
    {
        // 単語の終端の添え字を返す場合
        let mut s = String::from(hello_world);
        let first_size = first_word_length(&s); // 空白までのバイト数
        println!("first size = {}", first_size);
        s.clear();
        // ここで s を空と（""と等しく）しても、当然だが first_size に変更はない
        // そのため first_size は「sの空白までのバイト数」という意味としてはもう使えない。
        println!("first size after clear = {}", first_size);
    }
    {
        // 文字列スライス
        let s = String::from(hello_world);

        // 両者は同じ
        let _hello = &s[0..5];
        let _hello = &s[..5];

        let len = s.len();
        // 両者は同じ
        let _world = &s[6..len];
        let _world = &s[6..];

        // 全て同じ
        let _hello_world = &s[0..len];
        let _hello_world = &s[..];
        let _hello_world = hello_world;
    }
    {
        // 最初の例で first_word_length の代わりに first_word を使う
        let /* mut */ s = String::from(hello_world);
        let word = first_word(&s);
        // s.clear(); // s が mutable で定義されていても、first_word で借用しているためコンパイルエラーとなる。
        println!("the first word is: {}", word);

        let my_string = String::from(hello_world);
        // first_word_str 関数は String の部分スライスでも全体スライスでも String の参照でも受け入れる
        let _word1 = first_word_str(&my_string[..6]);
        let _word2 = first_word_str(&my_string[..]);
        let _word3 = first_word_str(&my_string);
        // let _word4 = first_word_str(my_string); // String そのものは入れられない

        // first_word_str 関数は文字列スライスの部分スライスでも全体スライスでも受け入れる。当然文字列スライスそのものでもOK
        let _word1 = first_word_str(&hello_world[..6]);
        let _word2 = first_word_str(&hello_world[..]);
        let _word3 = first_word_str(&hello_world);
        let _word4 = first_word_str(hello_world);
    }
    {
        // 文字列スライス以外のスライス
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        println!("{:?}", slice);
    }
}

fn first_word_length(s: &String) -> usize {
    let bytes = s.as_bytes();
    // 2番目の要素はコレクションの参照となる。& がなければ型は &u8 となる。
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
