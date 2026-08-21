use std::cmp::Ordering;
use std::io;
mod function_a;

use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number: i32 = rand::random_range(0..100);
    println!("The secret number is {secret_number}");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {guess}");

    let mut final_number = secret_number + guess.trim().parse::<i32>().unwrap();
    let mut final_number_2 = function_a::a_plus_b(secret_number, guess.trim().parse::<i32>().unwrap());
    assert_ne!(final_number, final_number_2);
}

