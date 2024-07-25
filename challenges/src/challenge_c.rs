// Create a calculator that takes three user inputs (x, y, and operator)
// Create functions for +, -, *, /
// Use if/else or Match for operator
// Might take a little research!


use std::io;

pub fn execute_challenge() {
    println!("Give me number A: ");
    let mut input_a: String = String::new();
    io::stdin().read_line(&mut input_a);

    println!("Give me number B: ");
    let mut input_b: String = String::new();
    io::stdin().read_line(&mut input_b);

    let x: i32 = input_a.trim().parse().expect("Numero A no es un entero!");
    let y: i32 = input_b.trim().parse().expect("Numero B no es un entero!");

    //Receive an operator
    println!("Choose an operator: +, -, *, /");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator);
    let operator_slice = operator.trim();

    perform_operation(x, y, operator_slice);
}

fn perform_operation(x: i32, y: i32, operator: &str) {
    let float_x = x as f64;
    let float_y = y as f64;

    match operator {
        "+" => { sum(x, y) }
        "-" => { rest(x, y) }
        "*" => { multiply(x, y) }
        "/" => { divide(float_x, float_y) }
        _ => { println!("Invalid Operator") }
    }
}

fn sum(x: i32, y: i32) { println!("{} + {} = {}", x, y, x + y) }

fn rest(x: i32, y: i32) { println!("{} + {} = {}", x, y, x - y) }

fn multiply(x: i32, y: i32) { println!("{} + {} = {}", x, y, x * y) }

fn divide(x: f64, y: f64) { println!("{} + {} = {}", x, y, x / y) }
