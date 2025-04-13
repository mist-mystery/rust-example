use std::env;

fn main() {
    println!("option");
    args_get();
    println!("\nindex");
    args_index();
}

// コマンドライン引数をベクタにして、1番目と2番目のコマンドライン引数を表示。
// 引数が足りなければ範囲外アクセスとなり panic する
fn args_index() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Binary name: {}", &args[0]);
    println!("Searching for {query}");
    println!("In file {filename}");
}

// arg_index と同様だが、引数が足りない場合は早期 return で何もしない。
fn args_get() -> Option<()> {
    let args: Vec<String> = env::args().collect();

    let query = args.get(1)?;
    let filename = args.get(2)?;

    println!("Searching for {query}");
    println!("In file {filename}");
    Some(())
}
