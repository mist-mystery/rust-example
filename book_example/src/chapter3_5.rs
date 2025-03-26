pub fn main() {
    if_expression();
    println!("The result is {}", return_from_loops());
    loop_label();
    loop_with_while();
    loop_with_for();
}

fn if_expression() {
    let number = 6;

    // if の後の式の評価結果は bool 型でなければならない
    // if 式の条件に紐づけられる一連のコードはアームと呼ばれる
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // アームの評価結果は同じ型である必要がある。let で束縛しない場合でも同様
    let number = if true { 5 } else { 6 };
    println!("The value of number is: {number}");
}

// `loop` は `break` で値を返すことができる（while や for でのループではできない）
fn return_from_loops() -> i32 {
    // counter をループの外に保持し、条件を満たしたら `break` でループを抜け、値を返す
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
            // result を使用せず条件を満たしたときに関数から値を返すようにしても実質同じこと
            // return counter * 2;
        }
    };
    result
}

fn loop_label() {
    let mut count = 0;
    // ループラベルをつけることでループを区別し、ネストしたループを一気に抜けることができる。
    // この場合でも `break` で値を返すことができる
    let result = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up count * 100;
            }
            remaining -= 1;
        }

        count += 1;
    };

    println!("End result = {result}");
}

fn loop_with_while() {
    let mut number = 3;

    // `loop` と同様にループラベルを付けることも可能
    #[allow(unused_labels)]
    'counting_up: while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!\n");
}

// Rust では for i = 0; i < 10; i++ { ... } のような構文はなく、イテレーターを使う
// Range 型を使えば、一定回数ループするようなコードを簡潔に書ける
fn loop_with_for() {
    let values = [10, 20, 30, 40, 50];

    // `loop` と同様にループラベルを付けることも可能
    #[allow(unused_labels)]
    'print_array: for element in values {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
