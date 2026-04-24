use pyo3::prelude::*;

/// Root Python module. Add functions, classes, and submodules here.
#[pymodule]
fn {{crate_name}}(_m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
