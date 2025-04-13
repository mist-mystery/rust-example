use std::{
    env,
    fs::{self, File},
    io::Read,
};

fn main() {
    read_file();
    read_file_simple();
}

// src/public/poem.txt を読み込む想定。
fn read_file() -> Option<()> {
    let args: Vec<String> = env::args().collect();
    let _query = args.get(1)?;
    let filename = args.get(2)?;

    println!("In file {filename}");
    let mut f = File::open(filename).unwrap();

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("Write text:\n{contents}");
    Some(())
}

// ファイルの中身を全取得するだけならこっちの方が簡潔。
fn read_file_simple() -> Option<()> {
    let args: Vec<String> = env::args().collect();
    let _query = args.get(1)?;
    let filename = args.get(2)?;

    println!("In file {filename}");
    let contents = fs::read_to_string(filename).expect("something went wrong reading the file");

    println!("Write text:\n{contents}");
    Some(())
}
