//! The error type returned by [`NHSNumber::from_str`](crate::NHSNumber).
//!
//! `ParseError` is a zero-sized unit struct that signals "this string is
//! not a syntactically valid NHS Number". It deliberately carries no
//! detail — callers who need a richer taxonomy (wrong length, wrong
//! separator, non-digit character, …) wrap or map it at the parse site.
//!
//! See [`spec.md`](https://github.com/joelparkerhenderson/nhs-number/blob/main/spec.md)
//! §12 for the design rationale.

/// Error returned by `<NHSNumber as FromStr>::from_str` for any string
/// that is not one of the two accepted shapes (see [`spec.md`] §5).
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
/// [`spec.md`]: https://github.com/joelparkerhenderson/nhs-number/blob/main/spec.md
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

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
}
