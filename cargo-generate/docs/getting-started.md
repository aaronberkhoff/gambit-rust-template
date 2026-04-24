# Getting Started

## Requirements

- Rust **1.75.0** or later ([install via rustup](https://rustup.rs))

## Installation

=== "As a library dependency"

    Add {{crate_name}} to your `Cargo.toml`:

    ```sh
    cargo add {{crate_name}}
    ```

    Or edit `Cargo.toml` directly:

    ```toml
    [dependencies]
    {{crate_name}} = "0.1"
    ```

=== "As a binary"

    Install the latest release from crates.io:

    ```sh
    cargo install {{crate_name}}
    ```

    Or download a prebuilt binary from the
    [GitHub Releases](https://github.com/%7B%7Bauthor%7D%7D/%7B%7Bproject_name%7D%7D/releases) page.

## Basic usage

!!! note

    Replace this section with a real working example. The example should be
    copy-pasteable and runnable without modification.

```rust
use {{crate_name}}::SomeType;

fn main() {
    let value = SomeType::new();
    println!("{value:?}");
}
```

## Verifying the installation

```sh
{{crate_name}} --version
```

## Next steps

- [Configuration](configuration.md) — customise behaviour with config options
- [API Reference](https://docs.rs/%7B%7Bproject_name%7D%7D) — full type and function documentation
