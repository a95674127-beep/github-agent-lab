# Contributing

Conventions for the `agent-lab` sample crate. The review agent cites this
file in pull request reviews — keep it accurate.

## Style

- Format with `cargo fmt` before pushing; CI expects `cargo fmt --check`
  to pass.
- `cargo clippy --all-targets -- -D warnings` must be clean. Fix the
  lint; don't silence it with `#[allow]` unless the suppression is
  justified in a comment.
- No external dependencies. This crate is deliberately dependency-free;
  a PR that adds one needs a very good reason in its description.
- Public functions get a doc comment with at least one example.
- Names say what the thing does; avoid abbreviations outside the usual
  Rust idioms (`len`, `iter`, `fmt` are fine).

## Tests

- Every behaviour change comes with test changes. A PR that changes code
  in `src/` without touching `tests/` will be flagged.
- Tests live in `tests/` as integration tests and use plain `assert_eq!`
  / `assert!` — no test framework dependencies.
- A test that pins a bug fix must fail before the fix and pass after it.

## Pull requests

- Small and scoped: one change per PR, no drive-by refactors or
  reformatting of unrelated code.
- The description states what changed, why, and how it was verified
  (which commands were run, with what result).
- A human reviews and merges every PR. Agent-authored PRs get the same
  scrutiny as anyone else's — the bot never merges its own work.
