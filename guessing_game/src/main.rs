use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess : String = String::new(); // muttable variable that contains the guess value

    let result = io::stdin()
        .read_line(&mut guess)
        .expect("msg");

    println!("You guess is {guess} and the size {result}");
}
