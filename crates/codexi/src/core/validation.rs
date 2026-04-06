// src/core/validation.rs

pub fn validate_text_rules(s: &str, min: usize, max: usize) -> Result<(), String> {
    if s.contains('\n') {
        return Err("newlines not allowed".into());
    }

    let s = s.trim();

    if s.is_empty() {
        return Err("empty".into());
    }

    let len = s.chars().count();

    if len < min {
        return Err(format!("too short (min {})", min));
    }

    if len > max {
        return Err(format!("too long (max {})", max));
    }

    if s.starts_with('-') {
        return Err("cannot start with '-'".into());
    }

    Ok(())
}

/*------------------------ TEST ------------------------*/

#[cfg(test)]
mod tests {
    use super::*;

    // --- valid case ---

    #[test]
    fn valid_normal_text() {
        assert!(validate_text_rules("Hello", 1, 10).is_ok());
    }

    #[test]
    fn valid_exact_min_length() {
        assert!(validate_text_rules("ab", 2, 10).is_ok());
    }

    #[test]
    fn valid_exact_max_length() {
        assert!(validate_text_rules("abcde", 1, 5).is_ok());
    }

    #[test]
    fn valid_with_leading_trailing_spaces() {
        // trim() est apply, si "  hi  " count as "hi" (2 chars)
        assert!(validate_text_rules("  hi  ", 1, 5).is_ok());
    }

    #[test]
    fn valid_unicode_chars() {
        // "été" = 3 char Unicode
        assert!(validate_text_rules("été", 1, 5).is_ok());
    }

    // --- empty string ---

    #[test]
    fn empty_string_returns_error() {
        assert_eq!(validate_text_rules("", 1, 10), Err("empty".into()));
    }

    #[test]
    fn whitespace_only_returns_empty_error() {
        // after trim(), the trim is empty
        assert_eq!(validate_text_rules("   ", 1, 10), Err("empty".into()));
    }

    // --- Lenght ---

    #[test]
    fn too_short_returns_error() {
        assert_eq!(validate_text_rules("a", 3, 10), Err("too short (min 3)".into()));
    }

    #[test]
    fn too_long_returns_error() {
        assert_eq!(validate_text_rules("abcdef", 1, 5), Err("too long (max 5)".into()));
    }

    #[test]
    fn length_counted_in_unicode_chars_not_bytes() {
        // "éàü" = 3 chars Unicode but 6 bytes UTF-8
        assert!(validate_text_rules("éàü", 3, 3).is_ok());
    }

    #[test]
    fn trim_affects_length_check() {
        // "  abc  " after trim = "abc" (3 chars), not 7
        assert!(validate_text_rules("  abc  ", 3, 3).is_ok());
    }

    // --- new line ---

    #[test]
    fn newline_in_middle_returns_error() {
        assert_eq!(
            validate_text_rules("hello\nworld", 1, 20),
            Err("newlines not allowed".into())
        );
    }

    #[test]
    fn newline_at_end_returns_error() {
        assert_eq!(
            validate_text_rules("hello\n", 1, 10),
            Err("newlines not allowed".into())
        );
    }

    // --- Hyphen at the beginning ---

    #[test]
    fn starts_with_dash_returns_error() {
        assert_eq!(
            validate_text_rules("-invalid", 1, 10),
            Err("cannot start with '-'".into())
        );
    }

    #[test]
    fn dash_not_at_start_is_valid() {
        assert!(validate_text_rules("valid-text", 1, 15).is_ok());
    }

    #[test]
    fn dash_at_start_after_trim_returns_error() {
        // trim() enlève les espaces, donc "  -text" devient "-text"
        assert_eq!(
            validate_text_rules("  -text", 1, 10),
            Err("cannot start with '-'".into())
        );
    }

    // --- Error priority ---

    #[test]
    fn empty_check_before_length_check() {
        // min=5 but the string is empty — shall return "empty", not "too short"
        assert_eq!(validate_text_rules("", 5, 10), Err("empty".into()));
    }

    #[test]
    fn newline_check_before_length_check() {
        // "a\n" is to short (min=5) AND contain a newline — shall return "newlines not allowed"
        assert_eq!(validate_text_rules("a\n", 5, 10), Err("newlines not allowed".into()));
    }
}
