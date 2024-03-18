use std::io;
fn main() {
    println!("SimpleRustCalculator v2.0");
    println!("Licensed under GNU General Public License v3.0 (GPLv3)");
    println!("Modes = (1) = Addition, (2) = Subtraction, (3) = Multiplication, (4) = Division, (5) = Squared");
    let mut operator = String::new();
    // Allows user to input stuff
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");
    let operator: i8 = operator.trim().parse().expect("Please insert a number");
    if operator == 1 || operator == 2 || operator == 3 || operator == 4 || operator == 5 {
        // Valid operator
        println!("-----");
        // First Number
        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line");
        let num1: f64 = num1.trim().parse().expect("Please insert a number");
        // Squared
        if operator == 5 {
            let sqred = num1 * num1;
            println!("= {sqred}");
        }
        else {
            // Second Number
            let mut num2 = String::new();
            io::stdin()
                .read_line(&mut num2)
                .expect("Failed to read line");
            let num2: f64 = num2.trim().parse().expect("Please insert a number");
            // Operations
            // Addition
            if operator == 1 {
                let sum = num1 + num2;
                println!("= {sum}");
            }
            // Subtraction
            if operator == 2 {
                let dif = num1 - num2;
                println!("= {dif}");
            }
            if operator == 3 {
                let mul = num1 * num2;
                println!("= {mul}");
            }
            if operator == 4 {
                let div = num1 / num2;
                println!("= {div}");
            }
        }
    }
    else {
        // Invalid Operator
        println!("Invalid Operator, Please try again");
    }
}
