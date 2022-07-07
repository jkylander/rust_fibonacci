use std::io;
mod recursive;
mod binet;
use binet::fib_binet;
use recursive::fib_recurvsive;
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
        println!("Binet fast Fibonacci({}) -> {}", number, fib_binet(number));
        println!("Fibonacci({}) -> {}", number, fib_recurvsive(number));

    }

}