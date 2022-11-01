use std::io;

// this file does not work

fn main() {
    println!("multiply two numbers!");

    let mut first = String::new();
    let mut second = String::new();

    let mut third = (first*second);

    io::stdin()
        .read_line(&mut first)
        .read_line(&mut second)
        .expect("this is an error");

    printlin!("the answer is {third}");
}
