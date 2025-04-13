fn main() {
    let hello = "hello";
    {
        // String型はヒープにメモリを確保するため、mutable にできる
        // （文字列リテラルは中身がコンパイル時に確定するためできない）
        let mut message = String::from(hello);
        message.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える
        println!("{message}");
    }
    {
        // 変数のムーブ・コピー
        let s1 = String::from(hello);
        let s2 = s1; // ムーブ
        let s3 = s2.clone();
        // println!("s1 = {}", s1); // s1 はムーブ後に使用しようとするとコンパイルエラー
        println!("s2 = {s2}, s3= {s3}");

        let s = String::from(hello);
        {
            let m = s; // ムーブ
            println!("m = {m}"); // "hello"
        }
        // println!("s = {s}"); // sの値は（スコープ外でだが）ムーブされているため使用不可
    }
    {
        // 関数の引数のムーブ
        let s = String::from(hello);
        takes_ownership(s);
        // println!("{s}"); // sの値が関数にムーブされ、ここではもう有効でない

        let x = 5;
        makes_copy(x); // x も関数にムーブされるが、i32はCopyなので、この後にxを使っても大丈夫
        println!("x = {x}");
    }
    {
        // 戻り値のムーブ
        let s1 = gives_ownership(); // gives_ownershipは、戻り値をs1にムーブする
        let s2 = String::from(hello); // s2がスコープに入る
        let s3 = takes_and_gives_back(s2); // s2はtakes_and_gives_backにムーブされ戻り値もs3にムーブされる
        println!("s1 = {s1}, s3 = {s3}");
        // println!("s2 = {}", s2); // s2 はムーブ後に使用できない
    }
    {
        // タプルで複数の値を返す
        let s1 = String::from(hello);
        let (s2, len) = calculate_length_tuple(s1);
        // println!("s1 = {}", s1);                 // s1 はムーブ後に使用できないが、
        println!("The length of '{s2}' is {len}."); // s1 で渡したものを s2 として受け取ることで再利用はできる。ただし、借用(4-2)を使う方がよい。
    }
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) {
    println!("some_integer = {some_integer}");
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

// gives_ownershipは、戻り値を呼び出した関数にムーブする
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    #[allow(clippy::let_and_return)]
    some_string // some_stringが返され、呼び出し元関数にムーブされる
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_stringが返され、呼び出し元関数にムーブされる
}

// 引数の s と String の長さ length を一緒に返す
fn calculate_length_tuple(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返します
    (s, length)
}
