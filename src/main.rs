use std::io;
use rand::Rng;

fn main() {
    let max_guess = 100; //outside of tutorial scope at the moment
    let min_guess = 1; //but figures I'd want to limit range at a later point

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(min_guess..=max_guess);

    println!("The secret number is {}",secret_number);

    println!("Please input your guess between {min_guess}-{} inclusive",max_guess);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

}
