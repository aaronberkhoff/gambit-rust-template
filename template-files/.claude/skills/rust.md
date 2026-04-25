# Rust skills

## Code quality

- NEVER use `.unwrap()` or `.expect()` in library code — propagate errors with `?`
    and return a typed `Result`.
- NEVER use `panic!` or `unreachable!()` in paths reachable from Python. PyO3 catches
    panics and converts them to Python exceptions, but the traceback is opaque and the
    error message is unhelpful.
- Prefer `#[expect(...)]` over `#[allow(...)]`. `#[expect]` is validated at compile
    time and the compiler removes it automatically when the warning disappears, so
    suppressions do not silently accumulate.
- ALWAYS add a `// SAFETY:` comment justifying every `unsafe` block. The comment must
    explain which invariants are being upheld and why they hold at this call site.
- Use full variable names, not single-letter abbreviations. `index` not `i`,
    `connection` not `conn`, `error` not `e`.
- Prefer `if let` and let-chains over deeply nested `match` arms.

## Testing

- ALWAYS add a test for changed behaviour. Use a unit test for pure logic and an
    integration test for public API surface changes.
- NEVER run the full test suite to verify a single change. Target the relevant test:
    `cargo test <test_name>` or `cargo test <module>::`.
- Prefer `insta` snapshot tests over string-matching assertions for complex output —
    they produce clearer diffs on failure and are easier to update intentionally.

## Dependencies

- Use `cargo update --precise <crate> <version>` for targeted lockfile changes only.
- NEVER run `cargo update` (updates all dependencies at once) without being asked to.
- NEVER add a new dependency without running `cargo deny check` first — licences and
    duplicate crates are enforced in `deny.toml`.

## Skill development

If Claude proposes a lifetime annotation, a `.clone()`, or an `Arc<Mutex<_>>`,
understand why that solution was chosen and whether a simpler alternative exists
before committing. These are often correct but are also common crutches — the
reason matters.

When Claude implements a standard trait (`From`, `Display`, `Iterator`, `Error`),
read the trait documentation before accepting the implementation. You are signing a
contract with the type system; understand what callers will expect.
