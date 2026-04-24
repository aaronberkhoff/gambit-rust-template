# Getting Started

## Requirements

- Rust **1.75.0** or later ([install via rustup](https://rustup.rs))

## Installation

=== "As a library dependency"

    Add {{project_name}} to your `Cargo.toml`:

    ```sh
    cargo add {{project_name}}
    ```

    Or edit `Cargo.toml` directly:

    ```toml
    [dependencies]
    {{project_name}} = "0.1"
    ```

=== "As a binary"

    Install the latest release from crates.io:

    ```sh
    cargo install {{project_name}}
    ```

    Or download a prebuilt binary from the
    [GitHub Releases](https://github.com/{{author}}/{{project_name}}/releases) page.

## Basic usage

!!! note
    Replace this section with a real working example. The example should be
    copy-pasteable and runnable without modification.

```rust
use {{project_name}}::SomeType;

fn main() {
    let value = SomeType::new();
    println!("{value:?}");
}
```

## Verifying the installation

```sh
{{project_name}} --version
```

## Next steps

- [Configuration](configuration.md) — customise behaviour with config options
- [API Reference](https://docs.rs/{{project_name}}) — full type and function documentation
