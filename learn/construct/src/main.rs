use std::collections::HashMap;

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
impl User {
    fn say_hello(&self) {
        println!("Hello {}", self.username);
    }
    fn build_user(username: String, email: String) -> User {
        let user = Self {
            active: true,
            username,
            email,
            sign_in_count: 1,
        };
        user.say_hello();
        user
    }
}

struct Color(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    println!("Hello, world!");

    let mut jungle = User {
        email: String::from("dgjungle_c@163.com"),
        username: String::from("Jungle"),
        active: true,
        sign_in_count: 1,
    };
    jungle.username = String::from("Dgjungle");
    let jungle_2 = User::build_user(String::from("Jungle"), String::from("dgjungle_c@163.com"));
    println!("{:#?}", jungle);
    println!("{:#?}", jungle_2);

    // dbg!(jungle);
    dbg!(&jungle);

    let mut v: Vec<i32> = Vec::new();
    let m = vec![1, 2, 3];
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
}
