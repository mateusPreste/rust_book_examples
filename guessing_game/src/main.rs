use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess : String = String::new(); 

    let size = io::stdin()
        .read_line(&mut guess) // read_line APPEND the input to the string
        .expect("msg");

    let guess: i32 = guess.trim().parse().expect("expecting a integer");

    println!("You guess is {guess} and the size {:?}", size);

}
