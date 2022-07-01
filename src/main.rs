use std::io;
fn main() {

    loop{
        //Input
        println!("Input n: ");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line.");

        //Exit the program
        if number.trim() == "quit" || number.trim() == "exit"{
            break;
        }
        //Make sure it's a number
        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //Print fib(n)
        //println!("Binet fast Fibonacci({}) -> {}", number, fib_binet(number));
        println!("Fibonacci({}) -> {}", number, fib_recurvsive(number));

    }

}
//Recursive fibonacci
fn fib_recurvsive(n: i32) -> i32{
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
//Binet algorithm
fn fib_binet(n: i32) {
    
    
}