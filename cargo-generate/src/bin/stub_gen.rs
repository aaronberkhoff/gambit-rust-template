/// Generates Python stub files (.pyi) from PyO3 type annotations.
/// Run with: cargo run --bin stub_gen
///
/// Stubs are written to the project root as <module_name>.pyi.
/// Commit the generated file so IDEs and type checkers can use it offline.
fn main() -> pyo3_stub_gen::Result<()> {
    let stub = {{crate_name}}::stub_info()?;
    stub.generate()?;
    Ok(())
}
