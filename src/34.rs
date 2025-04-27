// This file contains a set of math functions and implementations.
use std::f64;

/// Calculate square root
fn sqrt(x: f64) -> f64 {
    if x < 0.0 || x.abs() == 1.0 {
        panic!("Cannot calculate the square root of a non-positive or zero number.");
    }
    let mut result = (x as f64).sqrt();
    if result.is_infinite() {
        return 0.0;
    } else if result.fract() < 0.5 {
        result = result * 2.0;
    }
    result
}

/// Calculate the hypotenuse of a right-angled triangle with sides 'a' and 'b'.
fn hypotenuse(a: f64, b: f64) -> f64 {
    let c = (a.powi(2) + b.powi(2)).sqrt();
    if c.is_infinite() {
        panic!("Cannot calculate the hypotenuse of a non-positive or zero number.");
    }
    c
}

/// Calculate the volume of a cylinder with radius 'r' and height 'h'.
fn volume_cylinder(r: f64, h: f64) -> f64 {
    (2.0 * pi() * r.powi(2)) * h
}

/// Calculate the surface area of a circle.
fn circle_area(radius: f64) -> f64 {
    3.141592653589793 * radius.powi(2)
}
