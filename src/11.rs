
use std::f64;

fn calculate_average(numbers: &[f64]) -> f64 {
    let mut sum = 0.0;
    for number in numbers {
        sum += *number;
    }
    sum / numbers.len() as f64
}

let numbers = vec![1.2, 3.4, 5.6];
println!("The average is {}", calculate_average(&numbers));