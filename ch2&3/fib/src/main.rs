use std::io;

fn main(){
    println!("{}", fib(1));
    println!("{}", fib(3));
    println!("{}", fib(5));


    println!("fibonacci generator:");

    loop {
        let mut num = String::new();

        println!("\nEnter a positive number:");

        io::stdin()
            .read_line(&mut num)
            .expect("failed to read number");
        
        if num.trim() == "quit" {
            break;
        }

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", fib(num))
    }
}

fn fib(num: u32) -> u32 {
    match num {
        0 => 1,
        1 => 1,
        _ => fib(num - 1) + fib(num - 2)
    }
}
