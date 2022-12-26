fn main() {
    let mut x = 5;

    println!("The value is:{x}");
    x = 6;
    println!("The value is:{x}");

    other_funtion();
    print_number(2);
    let value = caculate(5);
    println!("The value is {value}")
}

fn other_funtion() {
    println!("Other function")
}

fn print_number(i: i32) {
    println!("The number is {i}");
}

fn caculate(x: i32) -> i32 {
    x + 3
}
