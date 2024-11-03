use std::io;

fn main() {
    let max_guess = 9;
    let min_guess = 0;

    println!("Guess the number!");

    println!("Please input your guess between {}-{} inclusive",min_guess,max_guess);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

}
