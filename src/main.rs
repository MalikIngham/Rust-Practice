// use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the guessing game");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        let answer = rand::thread_rng().gen_range(1..=1000);

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        println!("You guessed: {}", &guess);
        println!("ans: {}", &answer);

        match guess.cmp(&answer){
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                print!("You win!");
                break;
            },
        }

    }
    
}