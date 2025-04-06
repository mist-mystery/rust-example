fn main() {
    {
        // 空のベクタを作成。何も挿入しないなら、どんなデータを保持させるかの型注釈が必須。
        let _v: Vec<i32> = Vec::new();
    }
    {
        // 可変にして push メソッドで値を追加
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
    }
    {
        // vec! マクロで初期値を持つ新しいベクタを作成。中身があれば型は推論される。
        let v = vec![1, 2, 3, 4, 5];

        // 添え字記法（indexing syntax）で値を取得。型は &i32
        // 添え字記法が使えるのは、Vec が Index trait (`fn index(&self, index: I)`)を実装しているため。
        let third_indexing = &v[2];
        println!("The third element is {third_indexing}");

        // get メソッドで値を取得。型は Option<&i32>
        let third_get = v.get(2);
        match third_get {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }

        // let does_not_exist = &v[100]; // [] で存在しない要素にアクセスしようとするとパニック
        let does_not_exist = v.get(100); // get メソッドの場合は None を返す
        assert_eq!(does_not_exist, None);
    }
    {
        // 不変参照取得後に可変借用した場合、可変借用後は以前の不変参照は使えなくなる。
        let mut v = vec![1, 2, 3, 4, 5];
        let _first = &v[0];
        v.push(6); // これより下で _first を参照すると、不変参照中に可変参照することになるため、借用規則に違反することになる。
        // println!("The first element is: {_first}");
    }
    {
        // 可変借用を先にした後、不変参照を取得するのは問題ない。
        let mut v = vec![1, 2, 3, 4, 5];
        v.push(6); // ここで可変借用は終了しているため、後の不変借用は問題ない
        let first = &v[0];
        println!("The first element is: {first}");
    }
    {
        spreadsheet();
    }
    {
        struct_vec_example();
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn spreadsheet() {
    // enum で（実質的に）異なる型の要素を保持するベクタを作成。
    // vector が取り得る全ての値をコンパイル時に網羅できない場合はトレイトオブジェクトを使えるらしい(chapter17)
    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // 要素を順番にアクセスするために不変参照を得る。
    for c in &row {
        println!("{c:?}");
    }

    // 要素を変更するために可変参照を得る。
    for c in &mut row {
        // 可変参照が参照している値を参照したり変更したりするには、参照外し演算子(*)を使用する必要がある。
        match c {
            SpreadsheetCell::Int(cell_val) => *c = SpreadsheetCell::Float(*cell_val as f64 * 2.0), // 要素に直接代入
            SpreadsheetCell::Float(cell_val) => *c = SpreadsheetCell::Int(*cell_val as i32 * 10), // 要素に直接代入
            SpreadsheetCell::Text(cell_val) => *cell_val = format!("{} {}", cell_val, cell_val), // enum の値だけを変更
        }

        // &c とする場合は、cell_val は不変参照になる。
        if let SpreadsheetCell::Int(cell_val) = &c {
            *c = SpreadsheetCell::Int(cell_val * 2)
        }
    }
    println!("{row:?}");
}

// リファレンスでの Vec の例
fn struct_vec_example() {
    {
        // 初期化は Vec::from でも行うことができる。
        let mut vec1 = vec![1, 2, 3];
        vec1.push(4);
        let vec2 = Vec::from([1, 2, 3, 4]);
        assert_eq!(vec1, vec2);
    }
    {
        // 予め0埋めしておくなら、resize メソッドを使うよりこちらのほうが効率的。
        let vec = vec![0; 5];
        assert_eq!(vec, [0, 0, 0, 0, 0]);
    }
    {
        // Vec をスタックとして扱う例。
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{top}");
        }
        println!("{stack:?}");
    }
}
