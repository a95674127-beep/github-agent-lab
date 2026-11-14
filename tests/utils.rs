use agent_lab::{clamp, is_palindrome, slugify, truncate};

#[test]
fn slugify_basic() {
    assert_eq!(slugify("Hello, World!"), "hello-world");
}

#[test]
fn slugify_collapses_and_trims_separators() {
    assert_eq!(slugify("  Rust  Agents  "), "rust-agents");
    assert_eq!(slugify("one--two__three"), "one-two-three");
}

#[test]
fn clamp_within_range() {
    assert_eq!(clamp(5, 0, 10), 5);
}

#[test]
fn clamp_below_and_above_range() {
    assert_eq!(clamp(-3, 0, 10), 0);
    assert_eq!(clamp(42, 0, 10), 10);
}

#[test]
fn palindrome_detected() {
    assert!(is_palindrome("A man, a plan, a canal: Panama"));
    assert!(is_palindrome("racecar"));
}

#[test]
fn non_palindrome_rejected() {
    assert!(!is_palindrome("hello"));
    assert!(!is_palindrome("agent"));
}

#[test]
fn truncate_shorter_than_max_is_unchanged() {
    assert_eq!(truncate("hi", 5), "hi");
}

/// This test FAILS on purpose: it pins the correct behaviour for a string
/// whose length is exactly `max_len`, which the deliberate bug in
/// `agent_lab::truncate` currently gets wrong. The bugfix agent's job,
/// later, is to make this test pass.
#[test]
fn truncate_exact_fit_is_unchanged() {
    assert_eq!(truncate("hello", 5), "hello");
}
