fn main() {
    define_struct();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn define_struct() {
    {
        // 構造体定義、インスタンス化
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        println!("user1.username = {}", user1.username);

        // フィールドを変更可能にしたい場合、全体を可変にする必要がある
        let mut user2 = build_user(
            String::from("unknown@example.com"),
            String::from("unknownusername"),
        );
        user2.email = String::from("anotheremail@example.com");
        println!("user2.email = {}", user2.email);
        // 可変なので丸ごと入れ替えもできる。構造体更新記法 `..` を使用
        user2 = User {
            email: String::from("anotheremail@example.com"),
            username: String::from("anotherusername567"),
            ..user1 // 明示的にセットされていない残りのフィールドが、与えられたインスタンスのフィールドと同じ値になるように指定する
        };
        println!("user2.sign_in_count = {}", user2.sign_in_count);

        let user3 = user1; // ムーブ
        // println!("user1.username = {}", user1.username); // ムーブ後は使用不可
        println!("user3.active = {}", user3.active);
    }
    {
        // タプル構造体
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);
        let _tuple = (0, 0, 0);
        let black = Color(0, 0, 0);
        let point = Point(0, 0, 0);
        println!("rgb({} {} {})", black.0, black.1, black.2);
        println!("(x, y, z) = ({}, {}, {})", point.0, point.1, point.2);
        // let black: Color = point; // Color と Point のシグネチャは同一だが異なる型のためムーブ等は不可
        // let point: Point = tuple; // 当然、タプルとも異なる
    }
    {
        // 構造体のフィールドに他の何かに所有されたデータへの参照を保持させるには、ライフライムを使う必要がある
        // struct User {
        //     username: &str,
        //     email: &str,
        //     sign_in_count: u64,
        //     active: bool,
        // }

        // let user1 = User {
        //     email: "someone@example.com",
        //     username: "someusername123",
        //     active: true,
        //     sign_in_count: 1,
        // };
    }
}

// User インスタンスを作成する
fn build_user(email: String, username: String) -> User {
    // フィールド初期化省略記号使用
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
