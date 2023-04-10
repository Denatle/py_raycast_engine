use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn return_pixels() -> PyResult<Vec<i8>> {
    let result: Vec<i8> = vec![1, 2, 3];
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn raycast(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(return_pixels, m)?)?;
    Ok(())
}