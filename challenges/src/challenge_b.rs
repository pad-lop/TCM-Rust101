/* Build a simple calculator that takes two user inputs
     then calculates the addition, subtraction, multiplication, and division
     of those two inputs.
  */

use std::io;

pub fn execute_challenge(){
    println!("Give me number A: ");
    let mut input_a: String = String::new();
    io::stdin().read_line(&mut input_a);

    println!("Give me number B: ");
    let mut input_b: String = String::new();
    io::stdin().read_line(&mut input_b);

    let x: i32 = input_a.trim().parse().expect("Numero A no es un entero!");
    let float_x = x as f64;
    let y: i32 = input_b.trim().parse().expect("Numero B no es un entero!");
    let float_y = y as f64;

    println!("The result of {} + {} is {}", x, y, x+y);
    println!("The result of {} - {} is {}", x, y, x-y);
    println!("The result of {} * {} is {}", x, y, x*y);
    println!("The result of {} / {} is {}", x, y, float_x/float_y);
}
