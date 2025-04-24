use pyo3::prelude::*;

#[pyfunction]
#[allow(unsafe_op_in_unsafe_fn)]
pub fn summarize(numbers: Vec<f64>) -> (f64, f64) {
    let sum: f64 = numbers.iter().sum();
    let count = numbers.len() as f64;
    let mean = if count > 0.0 { sum / count } else { 0.0 };
    (sum, mean)
}

#[pymodule]
fn csv_cruncher(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(summarize, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summarize() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0];
        let (sum, mean) = summarize(numbers);
        assert_eq!(sum, 10.0);
        assert_eq!(mean, 2.5);
    }

    #[test]
    fn test_summarize_empty() {
        let numbers: Vec<f64> = vec![];
        let (sum, mean) = summarize(numbers);
        assert_eq!(sum, 0.0);
        assert_eq!(mean, 0.0);
    }
}
