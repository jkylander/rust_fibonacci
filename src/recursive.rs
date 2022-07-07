pub fn fib_recurvsive(n: i32) -> i32{
    if n <=0{
        return 0;
    }
    else if n == 1 {
        return 1;
    } 
    else {
        return fib_recurvsive(n-1) + fib_recurvsive(n-2)
    }
}