pub fn calculate_pi(accuracy: u32) -> f64 {
    let mut sum = 0f64;
    for i in 1..=accuracy as usize {
        sum += (i as f64) / ((i * 2 + 1) as f64);
    }
    return 4.0 * sum;
}
