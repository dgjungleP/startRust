use std::collections::HashMap;

use crate::garden::vegetables::Asparagus;
pub mod garden;
fn main() {
    let plant = Asparagus {};
    println!("Hello, world! {:#?}", plant);

    let v = vec![
        5, 6, 9, 1, 52, 63, 98, 14, 22, 65, 384, 65, 91, 22, 47, 6, 58, 3, 1, 4, 5, 8, 6, 3, 2, 1,
        4, 5, 8, 1, 5, 7, 4, 99, 99, 99, 99, 99,
    ];

    let mid = get_mid_number(&v);
    let most = get_most_number(&v);

    println!("Mid number is:{mid}");
    println!("Most number is:{most}");
    let pig_latin = pig_latin_str("apple");
    println!("Pig latin:{}", pig_latin);
}

fn pig_latin_str(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let first_cahr = chars[0];
    let mut result;
    if first_cahr == 'a'
        || first_cahr == 'e'
        || first_cahr == 'i'
        || first_cahr == 'o'
        || first_cahr == 'u'
        || first_cahr == 'A'
        || first_cahr == 'E'
        || first_cahr == 'I'
        || first_cahr == 'O'
        || first_cahr == 'U'
    {
        result = String::from_iter(&chars);
        result.push('-');
        result.push_str("hay");
    } else {
        result = String::from_iter(&chars[1..]);
        result.push('-');
        result.push(first_cahr);
        result.push_str("ay");
    }
    result
}

fn get_mid_number(v: &Vec<i32>) -> i32 {
    let current = v.clone();

    let mid_index = current.len() / 2;
    current[mid_index]
}

fn get_most_number(v: &Vec<i32>) -> i32 {
    let mut number_map = HashMap::new();
    let mut max_key = 0;
    let mut max_count = 0;
    for ele in v {
        let count = number_map.entry(*ele).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            max_key = *ele;
        }
    }
    max_key
}
