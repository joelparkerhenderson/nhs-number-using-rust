//! The error type returned by [`NHSNumber::from_str`](crate::NHSNumber).
//!
//! `ParseError` is a zero-sized unit struct that signals "this string is
//! not a syntactically valid NHS Number". It deliberately carries no
//! detail — callers who need a richer taxonomy (wrong length, wrong
//! separator, non-digit character, …) wrap or map it at the parse site.
//!
//! See [`spec/index.md`](https://github.com/joelparkerhenderson/nhs-number-using-rust/blob/main/spec/index.md)
//! §12 for the design rationale.

/// Error returned by `<NHSNumber as FromStr>::from_str` for any string
/// that is not one of the two accepted shapes (see
/// [`spec/05-string-forms.md`] §5).
///
/// `ParseError` is a unit struct — every error value compares equal:
///
/// ```rust
/// use nhs_number::NHSNumber;
/// use nhs_number::parse_error::ParseError;
/// use std::str::FromStr;
///
/// let a = NHSNumber::from_str("not even close").unwrap_err();
/// let b = NHSNumber::from_str("wrong length").unwrap_err();
/// assert_eq!(a, b);
/// assert_eq!(a, ParseError);
/// ```
///
/// To map it to your own richer error type:
///
/// ```rust
/// use nhs_number::NHSNumber;
/// use std::str::FromStr;
///
/// #[derive(Debug, PartialEq)]
/// enum MyError {
///     BadNhsNumber(String),
/// }
///
/// let bad = "not a number";
/// let result: Result<NHSNumber, MyError> =
///     NHSNumber::from_str(bad).map_err(|_| MyError::BadNhsNumber(bad.into()));
/// assert_eq!(result, Err(MyError::BadNhsNumber("not a number".into())));
/// ```
///
/// `ParseError` also implements [`Display`](std::fmt::Display) and
/// [`std::error::Error`], so it flows through `?` into boxed-error
/// stacks without manual mapping:
///
/// ```rust
/// use nhs_number::NHSNumber;
/// use std::error::Error;
/// use std::str::FromStr;
///
/// fn parse(s: &str) -> Result<NHSNumber, Box<dyn Error>> {
///     Ok(NHSNumber::from_str(s)?)
/// }
///
/// assert!(parse("999 100 0003").is_ok());
/// assert!(parse("not a number").is_err());
/// ```
///
/// [`spec/05-string-forms.md`]: https://github.com/joelparkerhenderson/nhs-number-using-rust/blob/main/spec/05-string-forms.md
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

/// A fixed message with **no input echo** — the rejected candidate
/// string must never leak into error messages or logs
/// (see `AGENTS/safety.md` §3).
///
/// ```rust
/// use nhs_number::parse_error::ParseError;
///
/// assert_eq!(ParseError.to_string(), "invalid NHS Number string");
/// ```
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("invalid NHS Number string")
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_is_zero_sized() {
        assert_eq!(std::mem::size_of::<ParseError>(), 0);
    }

    #[test]
    fn test_parse_error_equality() {
        assert_eq!(ParseError, ParseError);
    }

    #[test]
    fn test_parse_error_debug() {
        let dbg = format!("{:?}", ParseError);
        assert_eq!(dbg, "ParseError");
    }

    #[test]
    fn test_parse_error_display_is_fixed_message() {
        let actual = ParseError.to_string();
        let expect = "invalid NHS Number string";
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_parse_error_implements_std_error() {
        fn assert_error<T: std::error::Error>() {}
        assert_error::<ParseError>();
        // And it boxes cleanly.
        let boxed: Box<dyn std::error::Error> = Box::new(ParseError);
        assert_eq!(boxed.to_string(), "invalid NHS Number string");
    }

    #[test]
    fn test_parse_error_propagates_via_question_mark() {
        use crate::NHSNumber;
        use std::str::FromStr;

        fn parse(s: &str) -> Result<NHSNumber, Box<dyn std::error::Error>> {
            Ok(NHSNumber::from_str(s)?)
        }
        assert!(parse("999 100 0003").is_ok());
        assert!(parse("bad").is_err());
    }
}
