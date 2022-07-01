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
        let number: usize = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //Print fib(n)
        println!("Fibonacci({}) -> {}", number, fib(number));
    }

}
//Recursive fibonacci
fn fib(n: usize) -> usize{
    if n <=0{
        return 0;
    }
    else if n == 1 {
        return 1;
    } 
    else {
        return fib(n-1) + fib(n-2)
    }
}