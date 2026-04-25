# Contributing

## Pre-commit hooks

Pre-commit hooks run checks and auto-fixes locally before each commit, keeping CI green by catching issues at the source.

### Setup

```sh
uv run pre-commit install
```

Run all hooks manually against every file:

```sh
uv run pre-commit run --all-files
```

Update hook versions to their latest releases:

```sh
uv run pre-commit autoupdate
```

### Available hooks

| Hook                      | Tool             | What it does                                                   |
| ------------------------- | ---------------- | -------------------------------------------------------------- |
| `trailing-whitespace`     | pre-commit-hooks | Strips trailing whitespace                                     |
| `end-of-file-fixer`       | pre-commit-hooks | Ensures files end with a newline                               |
| `check-yaml`              | pre-commit-hooks | Validates YAML syntax                                          |
| `check-toml`              | pre-commit-hooks | Validates TOML syntax                                          |
| `check-json`              | pre-commit-hooks | Validates JSON syntax                                          |
| `check-merge-conflict`    | pre-commit-hooks | Blocks files with unresolved merge conflict markers            |
| `check-case-conflict`     | pre-commit-hooks | Catches filenames that collide on case-insensitive filesystems |
| `check-added-large-files` | pre-commit-hooks | Blocks accidentally committed large files                      |
| `detect-private-key`      | pre-commit-hooks | Blocks accidentally committed private keys                     |
| `no-commit-to-branch`     | pre-commit-hooks | Prevents direct commits to `main`                              |
| `typos`                   | crate-ci/typos   | Spell-checks source, docs, and config files                    |
| `ruff-format`             | ruff             | Auto-formats Python files (Black-compatible)                   |
| `ruff`                    | ruff             | Lints Python and auto-fixes safe issues                        |
| `mdformat`                | mdformat         | Auto-formats Markdown files                                    |
| `cargo-fmt`               | local            | Auto-formats changed Rust files                                |
| `cargo-clippy`            | local            | Lints all Rust targets; warnings are errors                    |

!!! tip "Slow clippy?"
    `cargo-clippy` recompiles on every changed Rust file, which can be slow on large
    projects. Remove the `cargo-clippy` hook and rely on the [`clippy` CI job](#clippy)
    instead.

______________________________________________________________________

## CI/CD pipeline

### On every push and pull request to `main`

These jobs run on every push to `main` and on every pull request targeting `main`.
All jobs must pass before a PR can be merged.

#### Build

**File:** `.github/workflows/build.yml`

```sh
cargo build --all-features
```

Verifies the crate compiles cleanly with every feature enabled.

#### Test

**File:** `.github/workflows/test.yml`

```sh
cargo test --all-features
```

Runs the full unit and integration test suite.

#### Clippy

**File:** `.github/workflows/clippy.yml`

```sh
cargo clippy --all-targets --all-features -- -D warnings
```

Runs the Rust linter across all targets. `-D warnings` promotes every warning to
an error, enforcing a clean lint baseline.

#### Rustfmt

**File:** `.github/workflows/fmt.yml`

```sh
cargo fmt --all -- --check
```

Checks that all Rust source files are formatted according to `rustfmt.toml`. Exits
non-zero if any file would be changed.

#### MSRV

**File:** `.github/workflows/msrv.yml`

Builds and tests the crate against the minimum supported Rust version (**1.75.0**).
Feature flags are intentionally omitted — feature-gated dependencies may declare a
higher MSRV than the core crate promises.

#### Doc tests

**File:** `.github/workflows/doc-test.yml`

```sh
cargo test --doc
```

Runs all examples embedded in rustdoc comments as a separate job so doc failures are
visible independently from unit test failures.

#### Python bindings

**File:** `.github/workflows/python-test.yml`

Builds the PyO3 extension with `maturin develop` and runs the Python test suite with
`pytest`. Uses a separate Rust cache key (`python`) because maturin produces `cdylib`
artifacts that differ from the `rlib` artifacts produced by plain `cargo test`.

#### Code coverage

**File:** `.github/workflows/coverage.yml`

Generates a coverage report using `cargo-llvm-cov` (requires the `llvm-tools-preview`
component). Produces two outputs:

- A Markdown summary table written to the Actions job summary (visible in the workflow
    run UI without downloading anything).
- An HTML report uploaded as the `coverage-report` artifact, retained for 14 days.

The job has a 60-minute timeout because instrumented builds are significantly slower
than normal builds.

#### Dependency checks

**File:** `.github/workflows/deny.yml`

Runs `cargo-deny` to check:

- **Licenses** — all dependency licenses are on the allow-list in `deny.toml`
- **Bans** — no banned or duplicate crates
- **Advisories** — no known security vulnerabilities in the dependency tree

#### Spell check

**File:** `.github/workflows/spell-check.yml`

Runs `typos` across all source, documentation, and config files. To whitelist a
project-specific term that `typos` incorrectly flags, add it to `_typos.toml` at the
repo root:

```toml
[default.extend-words]
myterm = "myterm"
```

______________________________________________________________________

### On push to `main` (path-filtered)

#### Deploy docs

**File:** `.github/workflows/docs.yml`

Triggered when files under `docs/`, `mkdocs.yml`, or `pyproject.toml` change on
`main`. Builds the MkDocs Material site with `--strict` (warnings are errors) and
deploys it to GitHub Pages.

!!! note "One-time setup"
    Go to **Settings → Pages → Source** and set it to **GitHub Actions** before the
    first deploy.

______________________________________________________________________

### On version tag (`v*.*.*`)

Tag a release with a version tag to trigger the release pipeline:

```sh
git tag v1.2.3
git push origin v1.2.3
```

#### Release

**File:** `.github/workflows/release.yml`

1. **Validate** — checks that the tag version matches the `version` field in
    `Cargo.toml`. Fails fast with a clear error if they diverge.

1. **Build** — cross-compiles release binaries for all supported targets in parallel:

    | Target                      | Runner                |
    | --------------------------- | --------------------- |
    | `x86_64-unknown-linux-gnu`  | ubuntu-latest         |
    | `aarch64-unknown-linux-gnu` | ubuntu-latest (cross) |
    | `x86_64-apple-darwin`       | macos-13              |
    | `aarch64-apple-darwin`      | macos-latest          |
    | `x86_64-pc-windows-msvc`    | windows-latest        |

1. **Release** — collects all binaries, generates `SHA256SUMS.txt`, and publishes a
    GitHub release with auto-generated release notes.

To also publish to crates.io, uncomment the `publish-crate` job in `release.yml` and
add `CARGO_REGISTRY_TOKEN` as a repository secret.

#### Publish to PyPI

**File:** `.github/workflows/publish-pypi.yml`

Builds Python wheels for all supported platforms using `maturin` and publishes them
to PyPI alongside a source distribution (`sdist`).

| Target         | Runner         |
| -------------- | -------------- |
| Linux x86_64   | ubuntu-latest  |
| Linux aarch64  | ubuntu-latest  |
| Windows x86_64 | windows-latest |
| macOS arm64    | macos-latest   |
| macOS x86_64   | macos-13       |

!!! note "One-time setup"
    Configure [Trusted Publishing](https://docs.pypi.org/trusted-publishers/) in your
    PyPI project settings (recommended), or add a `PYPI_API_TOKEN` secret and a `pypi`
    environment in **Settings → Environments**.

______________________________________________________________________

### Scheduled

#### Security audit

**File:** `.github/workflows/audit.yml`

Runs `cargo-audit` against the [RustSec Advisory Database](https://rustsec.org) every
Monday at 06:00 UTC. Can also be triggered manually from the **Actions** tab via
`workflow_dispatch`.
