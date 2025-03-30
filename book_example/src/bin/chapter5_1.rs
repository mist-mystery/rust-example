// 構造体定義
// 一つ一つのデータ片はフィールドと呼ばれる
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // インスタンス化。構造体で宣言した通りの順番に指定する必要はない。
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // dot notation で値を取得
    assert_eq!(user1.username, "someusername123");

    {
        // フィールドを変更可能にしたい場合、全体を可変にする必要がある（一部のフィールドのみ可変にはできない）
        let mut user2 = build_user(
            String::from("unknown@example.com"),
            String::from("unknownusername"),
        );
        user2.email = String::from("anotheremail@example.com");
        assert_eq!(user2.email, "anotheremail@example.com");
    }
    {
        // 可変なので丸ごと入れ替えもできる。構造体更新記法 `..` を使用
        // `..` を最後以外で使うと "cannot use a comma after the base struct" のコンパイルエラーが発生する
        let user2 = User {
            email: String::from("another@example.com"),
            ..user1 // 明示的にセットされていない残りのフィールドが、与えられたインスタンスのフィールドと同じ値になるように指定する
        };
        // assert_eq!(user1.username, "someusername123"); // user1.username はムーブされるため使えなくなる
        assert_eq!(user1.email, "someone@example.com"); // user1.email はムーブされないため引き続き使える
        assert_eq!(user1.sign_in_count, 1); // 変数への `=` と同様、Copy トレイトを実装している型なら問題ない
        assert_eq!(user1.active, true);

        assert_eq!(user2.username, "someusername123");
    }
    {
        // タプル構造体
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let tuple = (0, 0, 0);
        assert_eq!(tuple, (0, 0, 0));

        let black = Color(0, 0, 0);
        let point = Point(0, 0, 0);
        let new_point = Point { 1: 1, ..point }; // タプル構造体でも構造体更新記法を使える
        assert_eq!(new_point.1, 1);
        // let black: Color = point; // Color と Point のシグネチャは同一だが異なる型のためムーブ等は不可
        // let point: Point = tuple; // 当然、タプルとも異なる

        let Point(x, y, z) = point; // タプル構造体を分配するには型名の明示が必要
        // let (x, y, z) = point; // これは型不一致でコンパイルエラー

        println!("rgb({} {} {})", black.0, black.1, black.2);
        println!("(x, y, z) = ({x}, {y}, {z})");
    }
    {
        // フィールドを持たない構造体を unit-like 構造体という
        struct AlwaysEqual;
        let _subject = AlwaysEqual; // インスタンス生成に {} は不要
    }
    {
        // 構造体に他の何かに所有されたデータへの参照を保持させるには、ライフタイムを使う必要がある。
        // String であれば構造体フィールドが所有権を持つため、ライフタイム明示の必要はない。
        struct User<'a> {
            _username: &'a str,
            _email: &'a str,
            _sign_in_count: u64,
            _active: bool,
        }

        let _user1 = User {
            _email: "someone@example.com",
            _username: "someusername123",
            _active: true,
            _sign_in_count: 1,
        };
    }
}

// User インスタンスを作成する
fn build_user(email: String, username: String) -> User {
    // フィールド初期化省略記法を使用
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
