use pyo3::prelude::*;
use pyo3::{PyResult, Python};

#[pyfunction]
pub fn summarize(_py: Python<'_>, numbers: Vec<f64>) -> PyResult<PyObject> {
    let sum: f64 = numbers.iter().sum();
    let count = numbers.len() as f64;
    let mean = if count > 0.0 { sum / count } else { 0.0 };

    // Create Python tuple (sum, mean) using raw C API
    unsafe {
        let tuple = pyo3::ffi::PyTuple_New(2);
        if tuple.is_null() {
            return Err(pyo3::exceptions::PyMemoryError::new_err("Failed to create tuple"));
        }

        let sum_obj = pyo3::ffi::PyFloat_FromDouble(sum);
        if sum_obj.is_null() {
            pyo3::ffi::Py_DECREF(tuple);
            return Err(pyo3::exceptions::PyMemoryError::new_err("Failed to create float for sum"));
        }

        let mean_obj = pyo3::ffi::PyFloat_FromDouble(mean);
        if mean_obj.is_null() {
            pyo3::ffi::Py_DECREF(tuple);
            pyo3::ffi::Py_DECREF(sum_obj);
            return Err(pyo3::exceptions::PyMemoryError::new_err("Failed to create float for mean"));
        }

        pyo3::ffi::PyTuple_SetItem(tuple, 0, sum_obj);
        pyo3::ffi::PyTuple_SetItem(tuple, 1, mean_obj);

        Ok(pyo3::PyObject::from_owned_ptr(_py, tuple))
    }
}

#[pymodule]
fn csv_cruncher(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(summarize, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::types::PyTuple;

    #[test]
    fn test_summarize() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let numbers = vec![1.0, 2.0, 3.0, 4.0];
            let result = summarize(py, numbers).unwrap();
            let tuple = result.extract::<&PyTuple>(py).unwrap();
            let sum: f64 = tuple.get_item(0).unwrap().extract().unwrap();
            let mean: f64 = tuple.get_item(1).unwrap().extract().unwrap();
            assert_eq!(sum, 10.0);
            assert_eq!(mean, 2.5);
        });
    }

    #[test]
    fn test_summarize_empty() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let numbers: Vec<f64> = vec![];
            let result = summarize(py, numbers).unwrap();
            let tuple = result.extract::<&PyTuple>(py).unwrap();
            let sum: f64 = tuple.get_item(0).unwrap().extract().unwrap();
            let mean: f64 = tuple.get_item(1).unwrap().extract().unwrap();
            assert_eq!(sum, 0.0);
            assert_eq!(mean, 0.0);
        });
    }
}
