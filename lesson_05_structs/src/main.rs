#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        return self.width * self.height;
    }
}

fn main() {
    let rect = Rectangle {
        height: 40,
        width: 30,
    };

    println!("{}", &rect.calculate_area());
}
