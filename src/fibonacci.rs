use std::io;
// Task 2. Generate the nth Fibonacci number.

fn main() {
    loop {
        println!("Enter the value of n: ");
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Exiting Application...");
                break;
            }
        };
        println!("The {}th Fibonacci number is: {}", n, fibonacci(n));
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}