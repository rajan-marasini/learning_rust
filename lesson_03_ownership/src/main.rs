fn main() {
    let s = String::from("hello world");
    let arr = [1, 2, 3, 4, 5, 6];
    let slice = &arr[1..3];
    assert_eq!(slice, &[2, 3, 4]);

    let first_word = find_first_word(&s);

    println!("{first_word}");
}

fn find_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
