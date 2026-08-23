use std::{cmp::Ordering, io::Read};
use rand::{Rng, random};
use std::io;

pub fn a_plus_b(a: i32, b: i32) -> i32 {
    return a + b;
}

pub fn string_conversion_to_types(input_string: String) -> i32 {
    let output_number : i32 = input_string.trim().parse::<i32>().expect("Please type a numeric string in the form of number");
    return output_number;
}

pub fn easy_guessing_game () {
    println!("Guess the number!");
    // rand::random_range -> tu convert sang kieu
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {guess}");

    // guess.trim().parse::<dtype>().unwrap();
    let final_number = secret_number + guess.trim().parse::<i32>().unwrap();
    let final_number_2 = a_plus_b(secret_number, guess.trim().parse::<i32>().unwrap());
    assert_eq!(final_number, final_number_2);

    println!("Oh I got those string {}", string_conversion_to_types(guess));

    // match a.cmp(&b) {} -> must borrow b. 
    match final_number.cmp(&secret_number) {
        Ordering::Greater => println!(">"),
        Ordering::Equal => println!("="),
        Ordering::Less => println!("<")
    }
}

pub fn matching_guess () {
    let random_number: u64 = rand::thread_rng().gen_range(1..=100);
    println!("The randomly generated random number is {random_number}");
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("This is my new guess");
    let guess : u64 = guess.trim().parse::<u64>().expect("Expect the guess to be transfered to unsigned int 64 bit");

    match guess.cmp(&random_number) {
        Ordering::Equal => {
            println!("Equal!")
        },
        Ordering::Greater => {
            println!("Greater")
        },
        Ordering::Less => println!("Less"),
    }

    let mut new_guess = String::new();
    println!("Time to loop");
    loop {
        io::stdin().read_line(&mut new_guess).expect("Failed to read line");
        
        let new_guess: u32 = match new_guess.trim().parse::<u32>() {
            Ok(new_guess) => break,
            Err(_) => continue,
        };
    }
}