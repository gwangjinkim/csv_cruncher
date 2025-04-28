use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::Bound;

/// A Python class representing the summary (sum and mean) of a list of numbers.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Summary {
    #[pyo3(get)]
    pub sum: f64,
    #[pyo3(get)]
    pub mean: f64,
}

#[pymethods]
impl Summary {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<Summary sum={}, mean={}>", self.sum, self.mean))
    }
}

/// Summarizes a list of numbers.
/// 
/// # Arguments
/// * `numbers` - A vector of f64 values.
/// 
/// # Returns
/// A `Summary` object with `sum` and `mean` attributes.
/// 
/// # Example
/// ```
/// let numbers = vec![1.0, 2.0, 3.0];
/// let summary = summarize(numbers);
/// assert_eq!(summary.sum, 6.0);
/// assert_eq!(summary.mean, 2.0);
/// ```
#[pyfunction]
pub fn summarize(numbers: Vec<f64>) -> Summary {
    let sum: f64 = numbers.iter().sum();
    let mean = if numbers.is_empty() { 0.0 } else { sum / numbers.len() as f64 };
    Summary { sum, mean }
}

#[pymodule]
fn csv_cruncher(_py: Python<'_>, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Summary>()?;
    m.add_function(wrap_pyfunction!(summarize, &m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_summarize_normal_case() {
        let summary = summarize(vec![1.0, 2.0, 3.0]);
        assert_eq!(summary.sum, 6.0);
        assert_eq!(summary.mean, 2.0);
    }
    #[test]
    fn test_summarize_empty_vector() {
        let summary = summarize(vec![]);
        assert_eq!(summary.sum, 0.0);
        assert_eq!(summary.mean, 0.0);
    }
    #[test]
    fn test_summarize_negative_numbers() {
        let summary = summarize(vec![-1.0, -2.0, -3.0]);
        assert_eq!(summary.sum, -6.0);
        assert_eq!(summary.mean, -2.0);
    }
}
