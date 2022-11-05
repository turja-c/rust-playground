fn main() {
//     let mut x = 5;
//     println!("value of x is: {x}");
//     x = 6;
//     println!("value of x is: {x}");


// -------------------------------------
    // let x = 5;
    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("the value of x in the inner scope is: {x}");
    // }

    // println!("the value of x is: {x}");

// -------------------------------------

    // let guess: f64 = "42".parse().expect("not a number!");
    // let _tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}