fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    let result = add(5, 3);
    println!("The result of 5 + 3 is {}", result);

    let result = subtract(10, 5);
    println!("The result of 10 - 5 is {}", result);

    let result = multiply(4, 6);
    println!("The result of 4 * 6 is {}", result);

    let result = divide(12, 3);
    println!("The result of 12 / 3 is {}", result);
}
