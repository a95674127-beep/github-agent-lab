---
description: Fix a maintainer-gated bug and open a PR
agent: bugfix
---

Fix the following (or the labeled issue if nothing is specified):

$ARGUMENTS

Confirm the root cause from the real code, make a minimal fix with a test that
fails before and passes after, run cargo test to verify, and open a PR for
human review. If you cannot confidently locate the bug, open no PR and comment
with what you found instead.
