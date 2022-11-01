use std::io; // input/output library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    // a macro that prints a string to the string
    println!("guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("secret number is: {secret_number}");

    loop {
    println!("please input your guess");
    
    // mutable (mut) variable
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");


    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("you guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => {
            println!("you win!");
            break;
        }
    }
}
}
