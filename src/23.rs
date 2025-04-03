fn sum(a: isize, b: isize) -> isize {
    a + b
}

fn product(x: isize, y: isize) -> isize {
    x * y
}

fn difference(x: isize, y: isize) -> isize {
    if x > y { return x - y } else { return x + y }
}
