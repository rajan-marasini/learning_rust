#![allow(unused)]

fn main() {
    use std::collections::HashMap;
    let mut map = HashMap::new();

    let text = "hello world wonderful world";

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}
