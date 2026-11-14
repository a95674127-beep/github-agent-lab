# SANDBOX-NOTES

How this repo was staged from Meta's **GitHub agent cookbook**
(https://dev.meta.ai/docs/cookbook/github-agent), what was changed and
why, and the local verification record. Staged locally on 2026-11-13;
not yet pushed anywhere.

## The deliberate bug (practice fixture)

- **Location:** `src/lib.rs`, function `truncate`.
- **Bug:** the too-long boundary check uses `>=` instead of `>`, so a
  string whose length is exactly `max_len` is wrongly truncated and loses
  its last character (`truncate("hello", 5)` returns `"hell"`). The site
  is marked in code with a `KNOWN BUG (practice fixture)` comment
  explaining the fault and the fix.
- **Designed failing test:** `truncate_exact_fit_is_unchanged`
  in `tests/utils.rs` (`assert_eq!(truncate("hello", 5), "hello")`).

## Verification (local, before staging was declared done)

- `cargo fmt` applied; tree is `cargo fmt --check` clean.
- `cargo clippy --all-targets -- -D warnings` — clean, no warnings.
- `cargo test`:
  - Integration tests (`tests/utils.rs`): **7 passed, 1 failed** — the
    one failure is `truncate_exact_fit_is_unchanged`, failing exactly as
    designed (`left: "hell"`, `right: "hello"`).
  - Doc-tests: **4 passed, 0 failed**.
  - Net: exactly one failing test in the whole suite, and it is the
    practice fixture.

## Adaptations vs. the cookbook originals

### `AGENTS.md` (adapted, per the cookbook's own instruction)

- Project description, key-files list, out-of-scope list, and conventions
  rewritten for this sandbox and its Rust crate (cookbook original was
  filled in for a Python cookbook).
- Conventions cite this repo's real rules from `CONTRIBUTING.md`:
  `cargo fmt`, `cargo clippy -- -D warnings`, no external dependencies,
  tests with every change, doc comments with examples.
- Added one convention the original didn't need: the deliberate
  `KNOWN BUG` fixture is intentional — agents must not fix it in an
  unrelated PR or report its failing test as a regression.
- The **SECURITY section is reproduced verbatim** from the cookbook,
  byte for byte, as the cookbook requires.

### `.opencode/agent/bugfix.md` (stack swap — the only agent whose tools changed)

- Frontmatter description: "run pytest to verify" → "run cargo test to
  verify".
- Bash allow-list: removed `"uv *"`, `"uvx *"`, `"python *"`,
  `"python3 *"`, `"pytest*"`, and `"PYTHONPATH=*"`; added
  `"cargo test*"`, `"cargo clippy*"`, `"cargo fmt*"`, `"cargo build*"`.
  Everything else untouched — `git *`, the `gh pr …` allows, the explicit
  `gh pr merge*` deny, the `rm`/`curl`/`wget`/`nc`/`ssh` denies, and the
  trailing `"*": deny` all stand exactly as the cookbook wrote them.
- Body, step 1: "prefer an inline `python3 -c "…"` or a real test" →
  "prefer a failing `cargo test` case (a real test …)".
- Body, step 4 (added stack-appropriate verification, same honest-report
  rule): verify now reads "Run `cargo test` plus `cargo fmt --check` and
  `cargo clippy --all-targets -- -D warnings`, and report the results
  honestly…".
- `permission.edit: allow` and its rationale comment are unchanged:
  the write agent still only runs behind the maintainer's `agent-fix`
  label gate, and still can never merge.

### The other six agents (no changes)

`triage.md`, `review.md`, `qa.md`, `rfc.md`, `stale.md`, and
`repo-agent.md` are reproduced from the cookbook **unchanged** — none of
them names Python, pytest, or uv, and their permissions posture, gates,
and no-merge / never-close rules are exactly as published.

### `.opencode/command/`

- `review.md` — as printed in the cookbook.
- `triage.md` and `fix.md` — the cookbook prints only the review command
  and describes the other two as "analogous"; these follow that exact
  pattern (description + `agent:` frontmatter, `$ARGUMENTS` passthrough),
  with `fix.md`'s body phrased for this stack (`cargo test`).

### `opencode.json`

Unchanged from the cookbook: Meta Model API provider
(`https://api.meta.ai/v1`, key read from `{env:MODEL_API_KEY}` at run
time — the key itself is never stored in the repo), model
`model_api/muse-spark-1.3`.

### Deliberately NOT staged

- **No `.github/workflows/` files.** The cookbook's five workflows
  (PR review, triage, `/oc` command, bugfix, stale scan) are placed by
  the repo owner himself when deployment starts — they are not part of
  this staging tree.
- No API key, secret, or token appears anywhere in this tree.
