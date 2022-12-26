use std::fs::File;

fn main() {
    // let v = vec![1, 2, 3];
    // v[99];
    // println!("Hello, world!");
    // panic!("crash and burn")

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(ie) => panic!("Problem creating the file:{:?}", ie),
            },
            other => panic!("Problem opening the file:{:?}", other),
        },
    };
    let p = File::open("world.txt").expect("Failed ti open world.txt");
}
