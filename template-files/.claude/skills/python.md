# Python / PyO3 skills

## Testing the boundary

NEVER treat `uv run pytest` as a substitute for `cargo test`. Run both when the PyO3
interface changes. A change can pass all Rust tests and still break the Python surface
if the exposed types, signatures, or error behaviour differ from what Python callers
expect.

Always run in this order after a PyO3 change:

```sh
cargo test --all-features          # Rust unit and integration tests
uv run maturin develop             # Rebuild the extension in-place
uv run pytest                      # Python surface tests
```

## Understanding the boundary

For every `#[pyfunction]` and `#[pymethods]` block Claude generates, understand what
happens on the Python side:

- What Python type does each Rust argument map to?
- What Python type does the return value become?
- What Python exception is raised if the function returns `Err(...)`?

If a `.pyi` stub file is present, check that Claude's type annotations match it. Stubs
are the contract Python tooling (mypy, Pyright) uses for type checking.

## Local iteration

Use `maturin develop` for local development — it installs an editable in-place
extension without a full wheel build. Use `maturin build` only when producing a
distributable wheel. Confusing the two wastes significant compile time.

```sh
uv run maturin develop    # fast — rebuilds only changed Rust code
uv run maturin build      # slow — full wheel, use for release verification
```

## Skill development

PyO3 error handling is non-obvious: Rust `Result::Err` becomes a Python exception,
but which exception class depends on how the error type implements `IntoPyErr`. When
Claude generates error conversion code, verify that the Python exception hierarchy
is what your Python callers will expect.
