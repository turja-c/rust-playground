fn main() {
    // println!("Hello, world!");
    // another_function(5, 'h');
    // total();
    // cond();
    // simple_counter()
    // second_simple_counter()
    // third_simple_counter()
    fourth_simple_counter();
}

fn another_function(value: i32, label: char) {
    println!("the value of x is: {value}{label}");
}

fn total() {
    let x = plus_one(5);
    println!("the value of x: {x}");
}

fn plus_one(x: i32) -> i32{
    x + 1
}

fn cond() {
    let condition = true;
    let num = if condition {5} else {6};

    println!("value of number is {num}")
}

fn simple_counter() {
    let mut counter = 0;

    let res = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is {res}");
}

fn second_simple_counter() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            } if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}")
}

fn third_simple_counter() {

    let mut num = 5;

    while num != 0 {
        println!("{num}");
    }
    num -= 1;

}

fn fourth_simple_counter() {
    let a = [90, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}