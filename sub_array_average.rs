fn main() {
    let mut arr1: [f64;5]=[1.5,3.6,2.6,3.1,4.7];
    let tup=(1,4);
    println!("{}", sub_array_average(&mut arr1, tup));
}

fn sub_array_average(arr: &mut [f64], bounds: (usize, usize)) -> f64 {
    let first = bounds.0;
    let second = bounds.1;
    let mut total: f64=0.0;
    for x in first..second{
        total = total + arr[x as usize];
    }
    let val = second as f64 - first as f64;
    let avg = total/val;
    return avg;
}