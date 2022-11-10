fn main() {
    // clone()
    // clone2()
    ownership_copy()
}


fn clone() {
    let s = String::from("hello");
    let s2 = s.clone();

    println!("s = {}, s2 = {}", s, s2);
}

fn clone2() {
    let s = 5;
    let s2 = s;

    println!("s = {}, s2 = {}", s, s2);
}

fn ownership_copy() {
    let s = String::from("hello");

    takes_ownership(s);
    
    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}