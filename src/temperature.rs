use std::io;

// Task 1. Convert temperatures between Fahrenheit and Celsius.
fn main() {
    println!("Select what do you want to do: ");
    println!("1. Convert temperatures between Fahrenheit and Celsius.");
    println!("2. Convert temperatures between Celsius and Fahrenheit.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: i32 = input.trim().parse().unwrap();
    match input {
        1 => {
            println!("Enter the temperature in Fahrenheit: ");
            let mut f = String::new();
            io::stdin().read_line(&mut f).unwrap();
            let f: f64 = f.trim().parse().unwrap();
            println!("The temperature in Celsius is: {}", fahrenheit_to_celsius(f));
        },
        2 => {
            println!("Enter the temperature in Celsius: ");
            let mut c = String::new();
            io::stdin().read_line(&mut c).unwrap();
            let c: f64 = c.trim().parse().unwrap();
            println!("The temperature in Fahrenheit is: {}", celsius_to_fahrenheit(c));
        },
        _ => println!("Invalid input!"),
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    ((f - 32.0) * 5.0 / 9.0).round()
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    ((c * 9.0 / 5.0) + 32.0).round()
}