fn main() {}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    /// 新しい HashMap を作成。use が必要。
    /// vec! のようにマクロで生成することはできない。
    fn new_hashmap() {
        // HashMap::new() を利用。作成後に insert メソッドで追加。
        // key, value とも所有権は HashMap が持つ。
        let mut scores_new = HashMap::new();
        scores_new.insert("Blue".to_string(), 10);
        scores_new.insert("Yellow".to_string(), 50);

        // 要素が分かっていて初期化するなら from を使うのが一番楽。
        let scores_from = HashMap::from([("Blue".to_string(), 10), ("Yellow".to_string(), 20)]);

        // into_iter().zip(...).collect() を利用して HashMap を作成。teams, initial_scores の所有権を奪う。
        // into_iter の代わりに iter を使うと、key が不変参照となる（HashMap に所有権がない）。
        let teams = ["Blue".to_string(), "Yellow".to_string()];
        let initial_scores = [10, 50];
        let scores_zip: HashMap<_, _> = teams.into_iter().zip(initial_scores).collect();

        // get メソッドで取得するのは不変参照の Option 型
        assert_eq!(scores_new.get(&String::from("Blue")), Some(&10));
        // Option::copied(self) で Option<T> を取得
        assert_eq!(scores_from.get(&String::from("Blue")).copied(), Some(10));
        // Option::unwrap_or(self, default: T) で &T を取得
        assert_eq!(scores_zip.get(&String::from("Blue")).unwrap_or(&0), &10);

        // HashMap<String, _> で get メソッドに指定する key は &str でも問題ない。
        assert_eq!(scores_new.get("Blue"), Some(&10));
        assert_eq!(scores_from.get("Blue").copied().unwrap_or_default(), 10);
        assert_eq!(scores_zip.get("Blue").ok_or(0), Ok(&10));
    }

    #[test]
    /// 所有権のある値を HashMap に入れるとムーブが発生する。
    fn hashmap_ownership() {
        let field_name = String::from("Favorite color");
        let field_value = "Blue".to_string();
        let mut map = HashMap::new();

        map.insert(field_name, field_value);

        // assert_eq!(field_name, "Favorite color"); // first_name はムーブされているためこれはできない
        assert_eq!(map.get("Favorite color"), Some(&"Blue".to_string()));
        assert_eq!(map.get(""), None);
    }

    #[test]
    /// HashMap の走査に for ループを使用
    fn hashmap_loop() {
        let mut scores = HashMap::new();
        scores.insert("Blue".to_string(), 10);
        scores.insert("Yellow".to_string(), 50);

        // 各要素を (key, value) のタプルで取り出せる
        for (key, &value) in &scores {
            match key.as_str() {
                "Blue" => assert_eq!(value, 10),
                "Yellow" => assert_eq!(value, 50),
                _ => panic!(),
            }
        }
    }

    #[test]
    /// 挿入等でキーが被った場合
    fn hashmap_duplicate_key() {
        let mut scores = HashMap::new();

        // insert で同じキーを挿入すれば、後からの値で置き換えられる。
        scores.insert("Blue".to_string(), 10);
        scores.insert("Blue".to_string(), 25);
        assert_eq!(*scores.get("Blue").unwrap(), 25);
        assert_eq!(scores.len(), 1);

        // entry メソッドを使って、存在しない可能性のある値を表す Entry enum を生成する。
        // 存在しない場合のみ、値を挿入する。存在する場合は何もしない。
        scores.entry("Yellow".to_string()).or_insert(50);
        scores.entry("Blue".to_string()).or_insert(50);
        assert_eq!(*scores.get("Yellow").unwrap(), 50);
        assert_eq!(*scores.get("Blue").unwrap(), 25); // 50 に変わっておらず、元の値のまま
        assert_eq!(scores.len(), 2);
    }

    #[test]
    /// or_insert メソッドで可変参照を返し、それを利用して値を変更する。
    fn hashmap_overwrite() {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        for (&key, &value) in &map {
            match key {
                "hello" => assert_eq!(value, 1),
                "wonderful" => assert_eq!(value, 1),
                "world" => assert_eq!(value, 2),
                _ => panic!(),
            }
        }
    }
}
