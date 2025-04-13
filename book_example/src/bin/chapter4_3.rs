fn main() {
    let hello_world = "hello world";
    {
        // 単語の終端の添え字を返す場合
        let mut s = String::from(hello_world);
        let first_size = first_word_length(&s); // 空白までのバイト数
        println!("first size = {}", first_size);
        s.clear();
        // ここで s を空と（""と等しく）しても、当然だが first_size に変更はない
        // そのため first_size は「sの空白までのバイト数」という意味としてはもう使えない。
        println!("first size after clear != {}", first_size);
    }
    {
        // 文字列スライス
        let s = String::from(hello_world);

        // 両者は同じ
        let hello_both = &s[0..5];
        let hello_end = &s[..5];
        assert_eq!(hello_both, hello_end);

        let len = s.len();
        // 両者は同じ
        let world_both = &s[6..len];
        let world_start = &s[6..];
        assert_eq!(world_both, world_start);

        // 全て同じ
        let hello_world_both = &s[0..len];
        let hello_world_slice = &s[..];
        assert_eq!(hello_world_both, hello_world_slice);
        assert_eq!(hello_world, hello_world_slice);
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
        #[allow(clippy::redundant_slicing)]
        let _word2 = first_word_str(&hello_world[..]);
        let _word3 = first_word_str(hello_world);
        #[allow(clippy::needless_borrow)] // 本来なら引数の型が異なる(&&str)が、エラーにはならない
        let _word4 = first_word_str(&hello_world);
    }
    {
        // 文字列スライス以外のスライス
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(return_slice(slice), [2, 3]);
    }
}

// String 引数へのバイト数で表された添え字を返す
fn first_word_length(s: &String) -> usize {
    // String をバイト配列に変換
    let bytes = s.as_bytes();

    // バイト配列のイテレータを生成し、enumerate メソッドでタプルを返す。タプルはパターンを使って分配。
    // 2番目の要素はコレクションの参照となる。& がなければ型は &u8 となる。
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// 文字列を受け取って、その文字列中の最初の単語を返す関数
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 引数は &str とするとより一般的にできる。
// これは implicit deref coercion （暗黙的な参照外し型強制）という機能を使っている。
fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}

fn return_slice(a: &[i32]) -> &[i32] {
    a
}
