use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

// Registers type information used by `cargo run --bin stub_gen` to produce .pyi stubs.
define_stub_info_gatherer!(stub_info);

/// Root Python module. Add functions, classes, and submodules here.
#[pymodule]
fn {{crate_name}}(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
