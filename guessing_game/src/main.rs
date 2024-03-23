//Standar library moduel io for input/output used in this case for input and String
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //Macros for printing in the terminal
    //Comes included with the prelufe
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess (between 1-100)");

        let mut guess = String::new(); //Mutable (mut) variable

        //We call stdin from the io module
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
        Err(_) => continue,
        };

        println!("You guessed: {guess}"); //Using {} as placeholders

        //We match the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
