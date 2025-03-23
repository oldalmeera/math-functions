use num_complex::Complex;

fn main() {
    let a = 2.5 + 3.0i;
    let b = 4.0 - 1.0i;

    // Perform complex addition and subtraction
    let c = a.add(&b);
    let d = a.sub(&b);

    println!("a + b: {:?}", c);
    println!("a - b: {:?}", d);
}
