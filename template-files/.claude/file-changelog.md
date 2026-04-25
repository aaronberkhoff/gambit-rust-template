# Per-file AI changelog

When Claude generates a file from scratch or makes a significant change to an existing
file, add or update a changelog block at the very top of the file.

## When to add an entry

- ALWAYS add a block when generating a file from scratch.
- Add an entry when a change touches more than ~20 lines or restructures the file.
- Skip trivial edits: single-line fixes, typo corrections, auto-formatter runs
  (`rustfmt`, `ruff-format`).

## Format by file type

| File type | Comment syntax |
|-----------|----------------|
| `.rs`, `.py` | `// AI: <date> - <summary> (<model>)` |
| `.yml`, `.yaml`, `.toml` | `# AI: <date> - <summary> (<model>)` |
| `.md` | `<!-- AI: <date> - <summary> (<model>) -->` |

## Rules

- Place the block at the very top of the file, above all other content including
  license headers and shebangs.
- Keep each entry to one line: purpose and model only. No session context, no
  task references — those belong in the commit message.
- Append a new entry for each significant change; do not edit previous entries.

## Examples

YAML workflow file:

```yaml
# AI: 2026-04-25 - Initial generation (claude-sonnet-4-6)
# AI: 2026-04-26 - Added aarch64 matrix target (claude-sonnet-4-6)
name: Build
on:
  push:
```

Rust source file:

```rust
// AI: 2026-04-25 - Initial generation (claude-sonnet-4-6)

use std::fmt;
```

Markdown file:

```markdown
<!-- AI: 2026-04-25 - Initial generation (claude-sonnet-4-6) -->

# My Page
```
