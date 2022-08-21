fn main() {
    let p1 = Poly::implementPoly([35, -32, 23, -24, 77, -7, 8, -1, 10, -15]);
    println!("{}", p1.evaluatePoly(6));}

struct Poly {
    polyCoefficients: [i32;10],
    polyDegree: usize
}

impl Poly {
    fn implementPoly(polyCoefficients: [i32;10]) -> Poly {
        let mut polyDegree = 0;
        for x in 0..polyCoefficients.len() {
            if polyCoefficients[x]!=0{
                polyDegree = 9-x;
            }
        }
        Poly {
            polyDegree: polyDegree, 
            polyCoefficients,
        }
    }

    fn evaluatePoly(self, var: i32) -> i32 {
        let mut value = 0;
        for x in 0..self.polyCoefficients.len() {
            value = value + i32::pow(var, 9-x as u32) * self.polyCoefficients[x];
        }
        return value;
    }
}