use std::io;
use rand::Rng;

fn main() {
    println!("Number guessing game!");
    let random_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut guess = String::new();
        println!("Enter your guess");
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Greater => println!("Too large"),
            std::cmp::Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
