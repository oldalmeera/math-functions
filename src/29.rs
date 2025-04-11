// Example Rust code for mathematical operations: +, -, *, /.
fn main() {
    let x = 5.0;
    let y = 3.0;

    // Addition
    let result1 = x + y;
    println!("The sum of {} and {} is {}", x, y, result1);

    // Subtraction
    let result2 = x - y;
    println!("The difference between {} and {} is {}", x, y, result2);

    // Multiplication
    let result3 = x * y;
    println!("The product of {} and {} is {}", x, y, result3);

    // Division
    if y != 0.0 {
        let result4 = x / y;
        println!(
            "The quotient between {} and {} is {:.2}",
            x, y, result4
        );
    } else {
        println!("Error: Division by zero");
    }
}
