use std::collections::HashMap;

fn main() {
    new_hashmap();
    hashmap_ownership();
    hashmap_loop();
    hashmap_insert();
    hashmap_overwrite();
}

// 新しい HashMap を作成。use が必要。
fn new_hashmap() {
    // HashMap::new() を利用。作成後に insert メソッドで追加。
    // key, value とも所有権は HashMap が持つ。
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);

    // タプルの Vec から iter().zip(...).collect() を利用して HashMap を作成。
    // こちらは key, value ともに不変参照となる（数値であろうとコピーが起きず、HashMap に所有権がない）。
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // get メソッドで取得するのは不変参照の Option 型
    assert_eq!(scores1.get(&String::from("Blue")), Some(&10));
    // 元々 HashMap の値が i32 の不変参照であるから、get メソッドで取得するのは i32 の不変参照の不変参照の Option 型
    assert_eq!(scores2.get(&String::from("Blue")), Some(&&10));
    // HashMap<String, _> で get メソッドに指定する key は &str でも問題ない。
    assert_eq!(scores1.get("Blue"), Some(&10));
    // こちらはダメ。
    // assert_eq!(scores2.get("Blue"), Some(&&10));
}

// 所有権のある値を HashMap に入れるとムーブが発生する。
fn hashmap_ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();

    map.insert(field_name, field_value);

    // assert_eq!(field_name, "Favorite color"); // first_name はムーブされているためこれはできない
    assert_eq!(map.get("Favorite color"), Some(&String::from("Blue")));
    assert_eq!(map.get(""), None);
}

// HashMap の走査に for ループを使用
fn hashmap_loop() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, &value) in &scores {
        match key.as_str() {
            "Blue" => assert_eq!(value, 10),
            "Yellow" => assert_eq!(value, 50),
            _ => panic!(),
        }
    }
}

fn hashmap_insert() {
    let mut scores = HashMap::new();

    // insert で同じキーを挿入すれば、後からの値で置き換えられる。
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    for (key, &value) in &scores {
        match key.as_str() {
            "Blue" => assert_eq!(value, 25),
            _ => panic!(),
        }
    }

    // entry メソッドを使って、存在しない可能性のある値を表す Entry enum を生成する。
    // 存在しない場合のみ、値を挿入する。存在する場合は何もしない。
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    for (key, &value) in &scores {
        match key.as_str() {
            "Blue" => assert_eq!(value, 25),
            "Yellow" => assert_eq!(value, 50),
            _ => panic!(),
        }
    }
}

// or_insert メソッドで可変参照を返し、それを利用して値を変更する。
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
