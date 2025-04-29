fn summarize(numbers: &[f64]) -> (f64, f64) {
    let sum: f64 = numbers.iter().sum();
    let mean = if numbers.is_empty() {
        0.0
    } else {
        sum / numbers.len() as f64
    };
    (sum, mean)
}

fn main() {
    let data = vec![1.0; 1_000_000];
    let (_sum, _mean) = summarize(&data);
}
