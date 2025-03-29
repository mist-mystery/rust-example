// 列挙型（日本語版 The book だとそのまま enum と書いてあることが多い）の定義。V4, V6 は列挙子(variants)。
enum IpAddrKind {
    V4,
    V6,
}

pub fn main() {
    {
        // IpAddrKind の各列挙子のインスタンスを生成
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        // IpAddrKind を引数に取る関数は、どちらの列挙子に対しても呼び出せる。
        route(four);
        route(six);
    }
    {
        // 列挙子に値を紐づけない場合、例えば struct を使って IpAddrKind の列挙子を保持する。
        struct IpAddrStruct {
            kind: IpAddrKind,
            address: String,
        }

        let home = IpAddrStruct {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };
        let _loopback = IpAddrStruct {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };

        // match で取り得るパターンを列挙する（chapter6_2）
        match home.kind {
            IpAddrKind::V4 => (),
            IpAddrKind::V6 => assert_eq!(home.address, "127.0.0.1"),
        }
    }
    {
        // 列挙子に直接データを紐づける場合、IpAddrStruct のような構造体は必要なく、より簡潔に表現できる。
        // また、列挙子にそれぞれ異なる型を持たせることができる。
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
    {
        // 列挙子各々が異なる型を持つ enum
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        // 構造体と同様、impl を使って enum にメソッドや関連関数を定義できる
        impl Message {
            fn call(&self) {
                match self {
                    Self::Quit => (),
                    Self::Move { x, y } => println!("{:?}", (x, y)),
                    Self::Write(str) => println!("{str:?}"),
                    Self::ChangeColor(r, g, b) => println!("rgb({r}, {g}, {b})"),
                }
            }
            fn new() {}
        }

        let _m = Message::Quit;
        let _m = Message::Move { x: 0, y: 1 };
        let _m = Message::ChangeColor(0, 0, 0);
        let m = Message::Write(String::from("hello"));

        m.call();
        Message::new();
    }
    {
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
    {
        // Option enum
        let some_number = Some(5);
        let _some_char = Some('e');
        let _absent_number: Option<i32> = None; // None を使うときは Option<T> の型が何になるか明示する必要がある

        let x = 5;
        // let sum = x + some_number; // i32 と Option<i32> が異なる型であるため足し合わせることはできない
        assert_eq!(x + some_number.unwrap(), 10); // unwrap メソッドで Some の中身を取り出せる（None なら panic）
    }
}

fn route(_ip_type: IpAddrKind) {}
