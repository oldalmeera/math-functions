// This is an example of how you can use Rust to define a simple mathematical function.
// We will define a function called `add`, which takes two i32 parameters and returns their sum.

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// To call the function and print its result:
let result = add(5, 10);
println!("The sum is: {}", result);
