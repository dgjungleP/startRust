fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;
    loop {
        do_fib(&mut a, &mut b);
        i += 1;
        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);
    while i < n {
        do_fib(&mut a, &mut b);
        i += 1;
    }
}
fn do_fib(a: &mut i32, b: &mut i32) {
    let c = *a + *b;
    *a = *b;
    *b = c;
    println!("Next val is {}", b);
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);
    for _i in 2..n {
        do_fib(&mut a, &mut b);
    }
}
fn main() {
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}
