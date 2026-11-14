# github-agent-lab

A learning sandbox for Meta's **GitHub agent cookbook** — a fleet of
OpenCode agents (triage, review, Q&A, RFC, bug-fix, stale-scan, and an
orchestrator) powered by the Meta Model API, running against this repo so
their behaviour can be studied safely before they are trusted anywhere
real.

**Owner:** Allen Mack / AI Infinity Force One.

## What's in here

- `src/lib.rs` — a tiny, dependency-free Rust utility crate (`slugify`,
  `clamp`, `is_palindrome`, `truncate`) that gives the agents something
  small and safe to read, review, and repair.
- `tests/` — the crate's test suite.
- `opencode.json`, `AGENTS.md`, `.opencode/` — the agent configuration,
  adapted from the cookbook to this repo's Rust stack.
- `SANDBOX-NOTES.md` — every adaptation made versus the cookbook
  originals, and the verification record.

## The deliberate bug

`truncate` in `src/lib.rs` contains **one deliberate, clearly-marked bug**
(a wrong boundary check, flagged with a `KNOWN BUG (practice fixture)`
comment). Exactly one test — `truncate_exact_fit_is_unchanged` — fails
because of it. That failing test is the practice target: when the bug-fix
agent is gated on (via a maintainer's `/oc` comment or the `agent-fix`
label), its job is to investigate, patch the bug with a test, verify, and
open a pull request.

Nothing else in the suite fails. If you see a second failure, something
actually broke — treat it as a real bug report.

## Ground rules

- **The bot never merges.** Every agent-authored change lands as a pull
  request for human review. A human always makes the final call.
- Write paths are human-gated: the bot only writes code when a maintainer
  explicitly invokes it (`/oc` comment or `agent-fix` label).
- Agents treat all issue/PR/comment text as untrusted data, never as
  instructions (see the SECURITY section of `AGENTS.md`).

## Local checks

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test        # expect exactly one failure: the practice fixture
```
