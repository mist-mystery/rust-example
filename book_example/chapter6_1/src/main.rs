//! Enum

fn main() {}

#[cfg(test)]
mod tests {
    /// 列挙型（日本語版 The book だとそのまま enum と書いてあることが多い）の定義。V4, V6 は列挙子(variants)。
    enum IpAddrKind {
        V4,
        V6,
    }

    fn route(_ip_type: IpAddrKind) {}

    #[test]
    // IpAddrKind を引数に取る関数は、どの列挙子に対しても呼び出せる。
    fn enum_arg() {
        // IpAddrKind の各列挙子のインスタンスを生成
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        route(four);
        route(six);
    }

    #[test]
    /// struct を使って IpAddrKind の列挙子を保持してみる方法。
    fn struct_variants() {
        #[derive(PartialEq)]
        enum IpAddrKind {
            V4,
            V6,
        }

        struct IpAddrStruct {
            kind: IpAddrKind,
            address: String,
        }

        let home = IpAddrStruct {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };
        let loopback = IpAddrStruct {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };

        // enum に PartialEq を実装しないと列挙子を == で比較することができない。
        if home.kind == IpAddrKind::V4 {
            assert_eq!(home.address, "127.0.0.1");
        }

        // match によるパターンマッチは PartialEq を実装していなくても使える（chapter6_2）
        match loopback.kind {
            IpAddrKind::V4 => (),
            IpAddrKind::V6 => assert_eq!(loopback.address, "::1"),
        }
    }

    #[test]
    /// 列挙子に直接データを紐づける場合、IpAddrStruct のような構造体は必要なく、より簡潔に表現できる。
    /// また、列挙子にそれぞれ異なる型のデータを持たせることができる。
    fn data_in_variants() {
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        // enum だけを使って IP アドレスを保持する。
        // V4とV6のアドレスの型が違っていてもいい。
        let home = IpAddr::V4(127, 0, 0, 1);
        let _loopback = IpAddr::V6(String::from("::1"));

        // match で取り得るパターンを列挙する（chapter6_2）
        match home {
            IpAddr::V4(a, b, c, d) => assert_eq!(format!("{a}.{b}.{c}.{d}"), "127.0.0.1"),
            IpAddr::V6(ip) => assert_eq!(ip, "::1"),
        }
    }

    #[test]
    /// 列挙子各々が異なる型を持つ enum
    fn variety_enum() {
        #[allow(dead_code)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        // 構造体と同様、impl を使って enum にメソッドや関連関数を定義できる
        impl Message {
            fn call(&self) {
                // method body
            }

            // 関連関数の例
            fn new() -> Self {
                Message::Quit
            }
        }

        let _m = Message::Quit;
        let _m = Message::Move { x: 0, y: 1 };
        let _m = Message::ChangeColor(0, 0, 0);
        let m = Message::Write(String::from("hello"));

        m.call();
        Message::new();
    }

    #[test]
    /// Message 列挙体と似たような struct を定義
    fn variety_struct() {
        // Message enum の定義は、異なる種類の構造体定義を並べるのと類似
        struct QuitMessage; // ユニット構造体
        struct MoveMessage {
            x: i32,
            y: i32,
        }
        struct WriteMessage(String); // タプル構造体
        struct ChangeColorMessage(i32, i32, i32); // タプル構造体

        let _qm = QuitMessage;
        let MoveMessage { x: _x, y: _y } = MoveMessage { x: 0, y: 1 };
        let WriteMessage(_wm) = WriteMessage(String::from("hello"));
        let ChangeColorMessage(_cm0, _cm1, _cm2) = ChangeColorMessage(0, 0, 0);
    }

    #[test]
    #[allow(clippy::unnecessary_literal_unwrap)] // unwrap() 説明用にわざと Some(5) を作成
    /// enum の中で非常に一般的に使われる Option<T> を使う。
    fn option() {
        // Option enum
        let some_number = Some(5);
        let _some_char = Some('e');
        let _absent_number: Option<i32> = None; // None を束縛するときは Option<T> の型が何になるか明示する必要がある

        let x = 5;
        // let sum = x + some_number; // i32 と Option<i32> が異なる型であるため足し合わせることはできない
        assert_eq!(x + some_number.unwrap(), 10); // unwrap メソッドで Some の中身を取り出せる（None なら panic）
    }
}
