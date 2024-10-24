use std::io;

fn main() {
    println!("Welcome to the Simple Calculator!");

    loop {
        // Display menu
        println!("\nPlease select an operation:");
        println!("1. Addition (+)");
        println!("2. Subtraction (-)");
        println!("3. Multiplication (*)");
        println!("4. Division (/)");
        println!("5. Exit");

        // Read user choice
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number between 1 and 5.");
                continue;
            }
        };

        // Handle the menu selection
        match choice {
            1 => perform_operation("+"),
            2 => perform_operation("-"),
            3 => perform_operation("*"),
            4 => perform_operation("/"),
            5 => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Please enter a valid option (1-5)."),
        }
    }
}

// Function to perform selected arithmetic operation
fn perform_operation(operator: &str) {
    // Get two numbers from the user
    let num1 = get_number("Enter the first number:");
    let num2 = get_number("Enter the second number:");

    // Perform the operation based on the operator
    let result = match operator {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed.");
                None
            } else {
                Some(num1 / num2)
            }
        }
        _ => None,
    };

    // Display the result if operation was successful
    if let Some(value) = result {
        println!("Result: {} {} {} = {}", num1, operator, num2, value);
    }
}

// Function to get a number from the user
fn get_number(prompt: &str) -> f64 {
    println!("{}", prompt);

    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input, please enter a valid number.");
                input.clear();
            }
        }
    }
}
