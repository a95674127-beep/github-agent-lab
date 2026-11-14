//! Small text/number utilities used as practice material for the
//! GitHub-agent sandbox.
//!
//! Everything in this crate is intentionally tiny and dependency-free so the
//! agents have something safe to read, review, and (for one marked fixture)
//! repair.

/// Convert a string into a URL-style slug: lowercase ASCII letters and
/// digits are kept, every run of other characters collapses into a single
/// `-`, and leading/trailing `-` are trimmed.
///
/// # Examples
///
/// ```
/// use agent_lab::slugify;
/// assert_eq!(slugify("Hello, World!"), "hello-world");
/// assert_eq!(slugify("  Rust  Agents  "), "rust-agents");
/// ```
pub fn slugify(input: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = true; // trims leading separators for free
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    slug
}

/// Clamp `value` into the inclusive range `[min, max]`.
///
/// Assumes `min <= max`.
///
/// # Examples
///
/// ```
/// use agent_lab::clamp;
/// assert_eq!(clamp(5, 0, 10), 5);
/// assert_eq!(clamp(-3, 0, 10), 0);
/// assert_eq!(clamp(42, 0, 10), 10);
/// ```
pub fn clamp(value: i64, min: i64, max: i64) -> i64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Return `true` if `input` reads the same forwards and backwards,
/// ignoring case and any non-alphanumeric characters.
///
/// # Examples
///
/// ```
/// use agent_lab::is_palindrome;
/// assert!(is_palindrome("A man, a plan, a canal: Panama"));
/// assert!(!is_palindrome("hello"));
/// ```
pub fn is_palindrome(input: &str) -> bool {
    let cleaned: Vec<char> = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();
    cleaned.iter().eq(cleaned.iter().rev())
}

/// Truncate `input` to at most `max_len` characters (counted as Unicode
/// scalar values, not bytes). A string that already fits is returned
/// unchanged.
///
/// # Examples
///
/// ```
/// use agent_lab::truncate;
/// assert_eq!(truncate("hi", 5), "hi");
/// ```
pub fn truncate(input: &str, max_len: usize) -> String {
    // KNOWN BUG (practice fixture): the boundary check uses `>=` instead of
    // `>`, so a string whose length is exactly `max_len` is wrongly treated
    // as too long and loses its last character. `truncate("hello", 5)`
    // returns "hell" instead of "hello". The fix is to only truncate when
    // the input is strictly longer than `max_len`.
    if input.chars().count() >= max_len {
        input.chars().take(max_len.saturating_sub(1)).collect()
    } else {
        input.to_string()
    }
}
