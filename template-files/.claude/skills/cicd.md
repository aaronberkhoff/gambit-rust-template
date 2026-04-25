# CI/CD skills

## Before accepting a workflow change

Answer these three questions before committing any change to `.github/workflows/`:

1. What triggers this job? (push, PR, tag, schedule, `workflow_dispatch`)
1. What does it do, and what is the success condition?
1. What would cause it to fail, and would that failure be actionable?

If you cannot answer all three, ask Claude to explain before accepting the change.

## Job dependencies

Understand every `needs:` entry before adding one. A misplaced dependency can block
the entire pipeline when an unrelated job fails. Ask:

- Does this job genuinely need the dependency's output, or just its success signal?
- Is the dependency on the critical path, or can this job run in parallel?

## Timeouts

NEVER raise `timeout-minutes` without understanding why the job is slow. Common
legitimate reasons: instrumented builds (coverage), cross-compilation, large test
matrices. Raising a timeout to fix a flaky job hides the real problem.

## Cache keys

`Swatinem/rust-cache` cache keys are intentional. The specialised keys in this
template are:

| Key      | Job               | Reason                                                      |
| -------- | ----------------- | ----------------------------------------------------------- |
| `msrv`   | `msrv.yml`        | Prevents 1.75.0 artifacts polluting stable build cache      |
| `python` | `python-test.yml` | maturin produces `cdylib` artifacts that differ from `rlib` |

Do not remove or consolidate these keys without understanding artifact isolation.

## Sync awareness

Workflow files under `.github/workflows/` are synced from the template repo
(`aaronberkhoff/gambit-rust-template`). If a downstream change is accepted that
diverges from the template, note it in the PR description so the template can be
updated. Otherwise the next sync PR will revert the change.

## Skill development

CI failures are the most reliable signal that a workflow change is wrong. When a job
fails after a change Claude made, read the full log before asking Claude to fix it —
the log often reveals a misunderstanding about the job's purpose that Claude cannot
see without it.
