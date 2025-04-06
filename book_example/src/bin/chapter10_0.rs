fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    assert_eq!(result, Some(&100));

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    assert_eq!(result, Some(&6000));

    let result = largest(&[]);
    assert_eq!(result, None);
}

// スライスから最大値を探すコードを関数に抽出
fn largest(list: &[i32]) -> Option<&i32> {
    // 0番目の要素がなければ、?演算子により即座に None を返す。
    let mut largest = list.get(0)?;

    for item in list.iter() {
        // &i32 同士での比較が可能（Ord トレイト）
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}
