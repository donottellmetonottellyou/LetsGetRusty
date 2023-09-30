use pyo3::prelude::*;

use std::collections::HashMap;

#[pyfunction]
fn count_words(s: String) -> Py<PyAny> {
    let mut hash = HashMap::new();
    for sub_str in s.split(' ') {
        hash.entry(sub_str).and_modify(|c| *c += 1).or_insert(1);
    }

    Python::with_gil(|py| hash.to_object(py))
}

/// A Python module implemented in Rust.
#[pymodule]
fn ffi_rust_from_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_words, m)?)?;
    Ok(())
}
