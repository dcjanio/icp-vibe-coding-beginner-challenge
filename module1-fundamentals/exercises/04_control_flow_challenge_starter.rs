use std::io;
use std::io::Write;

fn main() {
    // Part 1: FizzBuzz Implementation
    println!("=== FizzBuzz Challenge ===");
    
    for i in 1..=20 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
    
    // Part 2: Menu-driven Calculator
    println!("\n=== Calculator ===");
    
    // TODO: Create a variable to control the calculator loop
    let mut running = true;
    
    // TODO: Implement the calculator loop
    while running {
        // TODO: Show the menu options
        println!("Choose an operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");
        
        // TODO: Get the user's choice
        let mut choice = String::new();
        // TODO: Read user input
        print!("Enter your choice: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        
        // TODO: Convert choice to a number (with error handling)
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        
        // TODO: Exit if the user chose option 5
        if choice == 5 {
            // TODO: Set running to false to exit the loop
            running = false;
            continue;
        }
        
        // TODO: Get the two input numbers from the user
        // First number
        let mut num1 = String::new();
        print!("Enter first number: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line");
        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        
        // Second number
        let mut num2 = String::new();
        print!("Enter second number: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read line");
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        
        // TODO: Perform the selected operation using match or if statements
        match choice {
            1 => println!("Result: {} + {} = {}", num1, num2, num1 + num2),
            2 => println!("Result: {} - {} = {}", num1, num2, num1 - num2),
            3 => println!("Result: {} * {} = {}", num1, num2, num1 * num2),
            4 => {
                if num2 == 0.0 {
                    println!("Error: Division by zero is not allowed.");
                } else {
                    println!("Result: {} / {} = {}", num1, num2, num1 / num2);
                }
            },
            _ => println!("Invalid option. Please try again."),
        }
        
        // TODO: Ask if the user wants to perform another calculation
        let mut continue_choice = String::new();
        print!("Do you want to perform another calculation? (y/n): ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut continue_choice)
            .expect("Failed to read line");
        
        // TODO: Set running to false if the user doesn't want to continue
        if continue_choice.trim().to_lowercase() != "y" {
            running = false;
        }
    }
    
    println!("Thank you for using the calculator!");
}