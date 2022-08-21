fn main() {
    let first = "firstword";
    let second = "secondword";
    println!("{}", string_concat(first, second));
}

fn string_concat(first: &str, second: &str) ->String {
    let concat = format!("{}{}", first, second);
    return concat;
}