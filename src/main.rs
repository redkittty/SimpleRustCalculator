use std::io;
fn main() {
    println!("SimpleRustCalculator v3.0");
    println!("Licensed under GNU General Public License v3.0 (GPLv3)");
    println!("Modes = (1) = Addition, (2) = Subtraction, (3) = Multiplication, (4) = Division, (5) = Squared, (6) = Sq Root, (7) = Pythagrean Theorum");
    let mut operator = String::new();
    // Allows user to input stuff
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");
    let operator: i8 = operator.trim().parse().expect("Please insert a number");
    if operator == 1 || operator == 2 || operator == 3 || operator == 4 || operator == 5 || operator == 6 || operator == 7 {
        // Valid operator
        println!("-----");
        // First Number
        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line");
        let num1: f64 = num1.trim().parse().expect("Please insert a number");
        if operator == 5 || operator == 6 {
            // Squared
            if operator == 5 {
                let sqred = num1 * num1;
                println!("= {sqred}");
            }
            // Sq Root
            if operator == 6 {
                let sqrt = f64::sqrt(num1);
                println!("= {sqrt}");
            }
        }
        else {
            // Second Number
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
            if operator == 3 {
                let mul = num1 * num2;
                println!("= {mul}");
            }
            if operator == 4 {
                let div = num1 / num2;
                println!("= {div}");
            }
            // Pythagrean Theorum
            if operator == 7 {
                let sq1 = num1 * num1;
                let sq2 = num2 * num2;
                println!("Is there a missing leg?");
                println!("(1) = yes, (2) = no,");
                let mut a = String::new();
                io::stdin()
                    .read_line(&mut a)
                    .expect("Failed to Read Line");
                let a: i8 = a.trim().parse().expect("Please insert a number");
                if a == 1 || a == 2 {
                    if a == 1 {
                        let pyt1 = sq1 - sq2;
                        let pyt2 = f64::sqrt(pyt1);
                        println!("= {pyt2}");
                    }
                    if a == 2 {
                        let pyt1 = sq1 + sq2;
                        let pyt2 = f64::sqrt(pyt1);
                        println!("= {pyt2}");
                    }
                }
                // Invalid Option
                else {
                    println!("Invalid Option, Please try again");
                }
            }
        }
    }
    else {
        // Invalid Operator
        println!("Invalid Operator, Please try again");
    }
    // Exit
    println!("---------------");
    println!("Press the ENTER key to exit!");
    let mut exit = String::new();
    io::stdin()
        .read_line(&mut exit)
        .expect("Closing with Failed to read line Error");

}
