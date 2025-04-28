use polars::prelude::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use serde_json;

/// Processes a CSV file, computing sum, mean, and max of the 'value' column.
///
/// # Arguments
/// * `path` - Path to the CSV file.
///
/// # Returns
/// A JSON string with sum, mean, and max.
///
/// # Example
/// ```
/// let result = crunch_csv("data.csv");
/// // Returns "{\"sum\":10.0,\"mean\":2.5,\"max\":4.0}"
/// ```
#[pyfunction]
fn crunch_csv(path: String) -> PyResult<String> {
    let df = LazyCsvReader::new(path)
        .finish()
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?
        .collect()
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

    let values = df
        .column("value")
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?
        .f64()
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

    let sum: f64 = values.sum().unwrap_or(0.0);
    let count = df.height() as f64;
    let mean = if count > 0.0 { sum / count } else { 0.0 };
    let max = values.max().unwrap_or(0.0);

    let result = serde_json::json!({
        "sum": sum,
        "mean": mean,
        "max": max
    });

    Ok(result.to_string())
}

#[pymodule]
fn csv_cruncher(_py: Python<'_>, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(crunch_csv, &m)?)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_crunch_csv() -> PyResult<()> {
        let csv_content = "value\n1.0\n2.0\n3.0\n";
        let path = "test.csv";
        File::create(path)?.write_all(csv_content.as_bytes())?;
        let result = json::parse(&crunch_csv(path.to_string())?)?;
        assert_eq!(result["sum"].as_f64().unwrap(), 6.0);
        assert_eq!(result["mean"].as_f64().unwrap(), 2.0);
        assert_eq!(result["max"].as_f64().unwrap(), 3.0);
        std::fs::remove_file(path)?;
        Ok(())
    }
}
