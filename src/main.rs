use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let max_guess = 100; //outside of tutorial scope at the moment
    let min_guess = 1; //but figures I'd want to limit range at a later point

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(min_guess..=max_guess);

    loop {
    println!("Please input your guess between {min_guess}-{max_guess} inclusive");
    
    let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        //let guess: u32 = guess.trim().parse().expect("Please type a number!"); //Crashed on error
        let guess: u32 = match guess.trim().parse() {   //Handle the error
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }

}
