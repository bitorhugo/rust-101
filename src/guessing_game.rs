use std::io;
use std::cmp::Ordering;
use rand::Rng; // random number generator

fn main() {
    println! ("Guess the number");
    loop {
        let secret = rand::thread_rng().gen_range(0..=100);
        println! ("Input a number");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("expecting guess input");
        let guess : u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; 

        match guess.cmp(&secret) {
            Ordering::Less => println!("Guess is lower than secret"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
            Ordering::Greater => println!("Guess is higher than secret"),
        }
        println!("secret = {}", secret);
    }
}
