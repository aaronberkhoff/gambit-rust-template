# gambit-rust-template

An opinionated Rust project template with a batteries-included CI suite, multi-platform
release pipelines, and tooling configuration. Generate a new project in seconds and
inherit a full workflow suite that stays up to date via automated pull requests.

## Who is this for?

This template targets Rust binary crates that want:

- A consistent CI baseline across multiple repos without copy-pasting workflows by hand
- Automated licence and security auditing on a schedule
- Multi-platform release binaries with SHA256 checksums on every version tag
- Low-friction template updates delivered as pull requests to downstream repos

## Quickstart

Install [cargo-generate](https://github.com/cargo-generate/cargo-generate) if you
haven't already:

```sh
cargo install cargo-generate
```

Generate a new project:

```sh
cargo generate --git https://github.com/aaronberkhoff/gambit-rust-template \
    --name my-project
```

You will be prompted for:

| Prompt | Used for |
| ------ | -------- |
| Project name | Crate name in `Cargo.toml` (must be `snake_case`) |
| Description | `description` field in `Cargo.toml` |
| Author | `authors` field and repository URL in `Cargo.toml` |

The generated project is ready to build immediately with `cargo build`.

## What's included

### Tooling configuration

| File | Purpose |
| ---- | ------- |
| `rustfmt.toml` | 100-char line width, crate-level import merging, `std`/external/crate import grouping |
| `.clippy.toml` | MSRV pinned to 1.75.0 |
| `.cargo/config.toml` | Sparse registry protocol, explicit default target, short aliases (`t`, `c`, `f`) |
| `deny.toml` | cargo-deny: allowed licences, banned crates, advisory policy |

### GitHub Actions workflows

These files are placed in `.github/workflows/` of the generated project and kept
up to date via the sync mechanism described below.

| Workflow | Trigger | What it does |
| -------- | ------- | ------------ |
| `ci.yml` | push / PR to `main` | Build, test, Clippy, rustfmt, MSRV check (1.75.0), doc tests, code coverage |
| `audit.yml` | weekly (Mon 06:00 UTC) + manual | `cargo audit` against the RustSec advisory database |
| `deny.yml` | push / PR to `main` | `cargo deny` — licence compliance, banned deps, advisories |
| `release.yml` | `vX.Y.Z` tag | Validates tag matches `Cargo.toml` version; builds 5-target matrix; GitHub release with SHA256 checksums |
| `docs.yml` | push to `main` (docs changes) + manual | Builds MkDocs Material site and deploys to GitHub Pages |

### Code coverage

The `coverage` job in `ci.yml` uses [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
and writes a summary table directly to the **Actions job summary** tab — visible in the
workflow run UI without downloading anything. A full HTML report is also uploaded as
a downloadable artifact (retained for 14 days).

### Release matrix targets

| Target | Runner |
| ------ | ------ |
| `x86_64-unknown-linux-gnu` | `ubuntu-latest` |
| `aarch64-unknown-linux-gnu` | `ubuntu-latest` + cross |
| `x86_64-apple-darwin` | `macos-13` |
| `aarch64-apple-darwin` | `macos-latest` (M1) |
| `x86_64-pc-windows-msvc` | `windows-latest` |

### Documentation site

Generated projects include a full [MkDocs Material](https://squidfunk.github.io/mkdocs-material/)
site under `docs/` — the same stack used by [uv](https://docs.astral.sh/uv/) and
[Ruff](https://docs.astral.sh/ruff/). Starter pages included:

| Page | Purpose |
| ---- | ------- |
| `docs/index.md` | Landing page with features, install snippet, quick example |
| `docs/getting-started.md` | Installation options and first usage walkthrough |
| `docs/configuration.md` | Configuration reference (prefilled structure to fill in) |
| `docs/changelog.md` | Keep a Changelog format with comparison links |

To enable deployment: go to **Settings → Pages → Source** and set it to
**GitHub Actions** (one-time, per repo).

### GitHub repository files

| File | Purpose |
| ---- | ------- |
| `.github/dependabot.yml` | Keeps Actions versions up to date (weekly, batched into one PR) |
| `.github/SECURITY.md` | Vulnerability disclosure policy |

## How sync works

`template-files/` in this repository mirrors the files that downstream repos should
have. When a change to `main` touches anything under `template-files/`, the
**Sync template files** workflow opens a pull request in every repo listed in
`repos.txt`.

Each downstream repo reviews and merges the PR as it would any dependency update.
Conflicts are resolved in the PR, giving downstream teams full control over what
they accept.

See [SETUP.md](SETUP.md) for instructions on configuring the required `SYNC_TOKEN`
secret and adding repos to `repos.txt`.

## Requirements

- **cargo-generate** ≥ 0.19 (for `--define` flag support)
- A GitHub Personal Access Token with **Contents**, **Pull requests**, and **Workflows**
  write permissions on each downstream repo — see [SETUP.md](SETUP.md)

## MSRV

The template configures downstream projects with a minimum supported Rust version of
**1.75.0** (December 2023). This is declared in `.clippy.toml` and in the generated
`Cargo.toml` (`rust-version` field). If you raise or lower the MSRV, update both
files and the `msrv` job in `ci.yml`.
