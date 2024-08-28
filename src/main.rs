//AI Response: The user's code contains mistakes as it returns incorrect values and lacks the correct
// implementation of Fibonacci sequence recursive function when compared to the reference code. Incorrect base
// case and missing addition of the results are the main problems.

use std::io;
enum Operation{
    Add(f64,f64),
Subtract(f64,f64),
Multiply(f64,f64),
Divide(f64,f64),
}

fn calculate(operation: Operation)->f64{
    match operation {
        Operation::Add(n1,n2)=>{
            n1+n2
        }
        Operation::Multiply(n1,n2)=>{
            n1*n2
        }
        Operation::Divide(n1,n2)=>{
            n1/n2
        }
        Operation::Subtract(n1,n2)=>{
            n1-n2
        }
    }
}
fn main() {
    let op :Operation;
    println!("Enter the first number:");
    let mut first_number = String::new();
    io::stdin().read_line(&mut first_number).expect("Failed to read input");
    let first_number: f64 = first_number.trim().parse().expect("Please enter a valid number");

    // Prompt the user for the operation
    println!("Enter the operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read input");
    let operation = operation.trim();

    // Prompt the user for the second number
    println!("Enter the second number:");
    let mut second_number = String::new();
    io::stdin().read_line(&mut second_number).expect("Failed to read input");
    let second_number: f64 = second_number.trim().parse().expect("Please enter a valid number");

    match operation {
        "+"=>{
          op=  Operation::Add(first_number,second_number)
        }
        "-"=>{
            op=  Operation::Subtract(first_number,second_number)
        }
        "*"=>{
            op=  Operation::Multiply(first_number,second_number)
        }
        "/"=>{
            op= Operation::Divide(first_number,second_number)
        }
        _ => {
            println!("Invalid operation entered!");
            return;
        }
    }
    let result =calculate(op);
    println!("The result is: {}", result);
}
        