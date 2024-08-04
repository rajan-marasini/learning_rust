use rand::Rng;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    let _random_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
}
