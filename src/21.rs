// This is a simple example of a function in Rust that takes two integers as input.
fn main() {
    let num1 = 5;
    let num2 = 3;

    // Example usage
    assert_eq!(sum(num1, num2), 8); // num1 + num2 should be equal to 8

    println!("The sum of {} and {} is {}", num1, num2, sum(num1, num2));
}

// This function calculates the sum of two integers using Rust's built-in functions.
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
