use std::io;

fn main() {
    println!("Guessing number!");

    println!("Please input your guess.");
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to read line");

    println!("You guessed {}", guess);

}
