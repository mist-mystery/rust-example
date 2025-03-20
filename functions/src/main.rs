fn main() {
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
    println!("The value of five() is: {}", five());
    println!("The value of plus_one(5) is: {}", plus_one(5));

    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", modify_array(arr));
    println!("{:?}", arr);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// 元の配列は変更されない
fn modify_array(mut arr: [i32; 5]) -> [i32; 5] {
    arr[0] = 0;
    arr
}
