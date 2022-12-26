fn main() {
    let x = 3;

    if x > 5 {
        println!("{x} is bigger than 5");
    } else {
        println!("{x} is less than 5");
    }
    let a = [1, 2, 3, 4];
    for element in a {
        println!("{element}");
    }

    for i in (1..10).rev() {
        println!("{i}");
    }
    println!("a {}", f2t(100.0));
    println!("b {}", t2f(100.0));
    println!("c {}", fb(1));
    println!("c {}", fb(2));
    println!("c {}", fb(3));
    println!("c {}", fb(4));
    println!("c {}", fb(5));
    println!("c {}", fb(6));
    let mut s = String::from("Hello");
    simple_print(&mut s);
    simple_print(&mut s);
    let a = &mut s;
    println!("{} {}", a, a);
    let w = String::from("Hello world");
    println!("{}", first_word("&w"));
}

fn simple_print(s: &String) {
    println!("{}", s);
}

fn f2t(f: f32) -> f32 {
    (f - 32.0) / 1.8
}
fn t2f(t: f32) -> f32 {
    1.8 * t + 32.0
}

fn fb(i: i32) -> i32 {
    if i == 0 || i == 1 {
        return 1;
    }
    fb(i - 1) + fb(i - 2)
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
