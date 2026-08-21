use std::cmp::Ordering;
use std::io;
mod function_a;

use rand::Rng;

fn main() {
    println!("Guess the number!");
    // rand::random_range -> tu convert sang kieu
    let secret_number: i32 = rand::random_range(0..100);
    println!("The secret number is {secret_number}");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {guess}");

    // guess.trim().parse::<dtype>().unwrap();
    let final_number = secret_number + guess.trim().parse::<i32>().unwrap();
    let final_number_2 = function_a::a_plus_b(secret_number, guess.trim().parse::<i32>().unwrap());
    assert_eq!(final_number, final_number_2);

    // match a.cmp(&b) {} -> must borrow b. 
    match final_number.cmp(&secret_number) {
        Ordering::Greater => println!(">"),
        Ordering::Equal => println!("="),
        Ordering::Less => println!("<")
    }
}

