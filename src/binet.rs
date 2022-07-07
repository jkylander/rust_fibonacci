use std::f64;

pub fn fib_binet(n: i32) -> i32 {
    let phi: f64 = (1.0 + 5.0_f64.sqrt())/2.0;
    let fib: f64 = (phi.powi(n))/5.0_f64.sqrt();
    return fib.round() as i32;
}
