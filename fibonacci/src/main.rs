fn main() {
    println!("Hello, world!");
    println!("{}", fib(8));
    println!("{}", fib_recur(8));

}

fn fib(num: u32) -> u64 {
   
   if num <= 0 {
        panic!("number is negative");
   } else if num == 0 {
        panic!("number is equal to zero");
   } else if num == 1 {
        return 1;
   }
   
   let mut sum = 0;
   let mut first = 0;
   let mut second = 1;

   for _a in 1..num {
        sum = first + second;
        first = second;
        second = sum;
   }
   return sum;

}

fn fib_recur(num: u32) -> u32 {
    if num <= 0 {
        return 0;
    } else if num ==1 {
        return 1;
    } else {
        return fib_recur(num - 1) + fib_recur(num - 2);
    }
}