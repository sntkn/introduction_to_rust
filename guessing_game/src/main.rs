use std::io;
use rand::Rng;

fn main() {
    println!("Guessing number!");

    let random_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", random_number);

    println!("Please input your guess.");
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to read line");

    println!("You guessed {}", guess);

}
