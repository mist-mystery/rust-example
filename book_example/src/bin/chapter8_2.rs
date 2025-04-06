fn main() {}
#[cfg(test)]
mod tests {
    #[test]
    // String は非常によく使われるため、たくさんの選択肢がある
    fn new_string() {
        let s1 = "initial contents".to_string();
        let s2 = String::from("initial contents");
        let mut s3 = String::from("initial");
        s3.push_str(" content"); // 可変借用
        s3.push('s'); // 可変借用
        assert_eq!(s1, s2);
        assert_eq!(s1, s3);
    }

    #[test]
    // + 演算子を使用すると `add` メソッド（Add トレイトを実装している）が呼ばれる。
    // add メソッドのシグニチャは、1番目の引数の型が self, 2番目の引数の型が &str となっている。
    // s2 で実際に渡している型は &String であるにも関わらず使えるのは、参照外し型強制を使用しているからである。
    // 1番目の引数の型は String となり、所有権を奪う。
    // つまり、s1 + &s2 で、コピーが発生するのは s2 の中身を s1 の後ろに追記するときのみ。
    fn string_concat() {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // s1はムーブされ、もう使用できないことに注意
        assert_eq!(s3, "Hello, world!");
    }

    #[test]
    // 複数の文字列を連結しようとすると複雑になる。
    fn string_concat_complex() {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = format!("{s1}-{s2}-{s3}"); // format! マクロを使う方が分かりやすく、かつ s1 の所有権を奪わない。
        assert_eq!(s, "tic-tac-toe");
        let s = s1 + "-" + &s2 + "-" + &s3; // これだと s1 の所有権を奪うため、これ以降 s1 は使えない。
        assert_eq!(s, "tic-tac-toe");
    }

    #[test]
    #[should_panic(expected = "byte index 1 is not a char boundary")]
    fn char_boundary() {
        let hello = "Здравствуйте";
        let _hello = String::from(hello);
        // 文字列に添え字記法でのアクセスはできない（Index トレイトは実装されていない）。
        // let h = hello[0];
        // let h = _hello[0];

        // 文字列スライスを作ることはできるが、char boundary でないところで区切ろうとすると panic
        assert_eq!(&hello[0..4], "Зд");
        let _s = &hello[0..1];
    }

    #[test]
    // 文字列の要素アクセスには chars メソッド、bytes メソッドを使えばエラーは起きない。
    fn string_methods() {
        let hello = "नमस्ते";
        for c in hello.chars() {
            println!("{c}");
        }
        // 添え字アクセスのようなことは chars().nth() で実現できる。
        assert_eq!(hello.chars().nth(1), Some('म'));

        for b in hello.bytes() {
            println!("{b}");
        }
    }
}
