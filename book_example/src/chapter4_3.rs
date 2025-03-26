pub fn main() {
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
