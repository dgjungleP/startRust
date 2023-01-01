use std::{collections::HashMap, thread, time::Duration};

fn main() {
    let simulate_user_soecialed_value = 10;
    let simulate_random_numver = 7;
    generate_workout(simulate_user_soecialed_value, simulate_random_numver);
}

fn generate_workout(intensity: i32, random_number: i32) {
    let mut expensive_closure = Cacher::new(|intensity| {
        println!("Calculate slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });
    if intensity < 25 {
        println!("Today, do {} pushups", expensive_closure.value(intensity));

        println!("Next, do {} situps", expensive_closure.value(intensity + 1));
        println!("Next, do {} situps", expensive_closure.value(intensity + 2));
        println!("Next, do {} situps", expensive_closure.value(intensity + 1));
        println!("Next, do {} situps", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        } else {
            println!(
                "Today, run for {} mimutes",
                expensive_closure.value(intensity)
            );
        }
    }
}
fn sumulated_expensive_calculation(intensity: i32) -> i32 {
    println!("Calculate slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
where
    T: Fn(i32) -> i32,
{
    caculation: T,
    value: HashMap<i32, i32>,
}

impl<T> Cacher<T>
where
    T: Fn(i32) -> i32,
{
    fn new(caculation: T) -> Cacher<T> {
        Cacher {
            caculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: i32) -> i32 {
        *self
            .value
            .entry(arg)
            .or_insert_with_key(|key| (self.caculation)(*key))
    }
}
