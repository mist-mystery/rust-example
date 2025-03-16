use std::io;

const MAX_POINTS: u32 = 100_000;
fn main() {
    variable();
    data_type();
}

fn variable() {
    let spaces = "   ";
    let spaces = spaces.len();
    {
        let spaces = spaces * 2;
        println!("The value of spaces in the inner scope is: {}", spaces);
    }
    println!("The value of spaces is: {}", spaces);

    println!("Max Point is: {}", MAX_POINTS);
}

fn data_type() {
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3;
    let _remainder = 43 % 5;

    let _t = true;
    let _f = false;

    let _c = '😻';
    let _str = "";

    let tup = (500, 6.4, 1);
    let (_x, _y, _) = tup;
    let _five_hundred = tup.0;

    let _a = [3; 5];
    let a = [1, 2, 3, 4, 5];
    let _first = a[0];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
