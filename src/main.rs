use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    loop {
        println!("\nEnter the number: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line! You are not to write any number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}