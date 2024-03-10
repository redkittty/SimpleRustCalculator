use std::io;
fn main() {
    println!("SimpleRustCalculator");
    println!("Modes = (1) = Addition, (2) = Subtraction, (3) = Multiplication, (4) = Division");
    let mut operator = String::new();
    // Allows user to input stuff
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");
    let operator: i8 = operator.trim().parse().expect("Please insert a number");
    if operator == 1 || operator == 2 || operator == 3 || operator == 4 {
        // Valid operator
        println!("-----");
        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line");
        let num1: f64 = num1.trim().parse().expect("Please insert a number");
        let mut num2 = String::new();
        io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read line");
        let num2: f64 = num2.trim().parse().expect("Please insert a number");
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
        // Multiplication
        if operator == 3 {
            let mul = num1 * num2;
            println!("= {mul}");
        }
        // Division
        if operator == 4 {
            let div = num1 / num2;
            println!("= {div}");
        }
    }
    else {
        // Invalid Operator
        println!("Invalid Operator, Please try again");
    }
}
