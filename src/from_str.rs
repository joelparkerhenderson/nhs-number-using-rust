//! `FromStr` parser for [`NHSNumber`].
//!
//! This module is the **only** entry point for parsing user-supplied
//! strings into an `NHSNumber`. It accepts exactly two shapes, both ASCII:
//!
//! 1. `"DDDDDDDDDD"` — ten contiguous decimal digits, no separators.
//! 2. `"DDD DDD DDDD"` — three groups of digits separated by single
//!    ASCII spaces at positions 3 and 7.
//!
//! Every other input — including different lengths, hyphens, leading or
//! trailing whitespace, doubled spaces, Unicode no-break spaces, Arabic-
//! Indic digits, or any non-digit character — returns
//! [`Err(ParseError)`](crate::parse_error::ParseError).
//!
//! The parser performs **no normalisation** of its input. Callers whose
//! upstream sources produce variant forms (hyphen separators, mixed
//! whitespace, full-width digits) must normalise before delegating here.
//!
//! See [`spec.md`](https://github.com/joelparkerhenderson/nhs-number/blob/main/spec.md)
//! §5 for the full contract.

use crate::NHSNumber;
use crate::parse_error::ParseError;
use std::str::FromStr;

/// Parse an [`NHSNumber`] from one of the two accepted string shapes.
///
/// A valid NHS Number string is one of:
///
/// - **Ten digits, no separators:** e.g. `"0123456789"`.
/// - **Canonical with single spaces:** e.g. `"012 345 6789"` — three digits,
///   space, three digits, space, four digits.
///
/// Both shapes must use ASCII digits 0–9 in every digit position. Anything
/// else returns [`Err(ParseError)`](crate::parse_error::ParseError) with
/// no further detail; richer error reporting is the caller's job.
///
/// Example — parsing both shapes:
///
/// ```rust
/// use nhs_number::NHSNumber;
/// use std::str::FromStr;
///
/// let from_tight     = NHSNumber::from_str("9991000003").unwrap();
/// let from_canonical = NHSNumber::from_str("999 100 0003").unwrap();
/// assert_eq!(from_tight, from_canonical);
/// ```
///
/// Example — rejecting an input that does not match either shape:
///
/// ```rust
/// use nhs_number::NHSNumber;
/// use std::str::FromStr;
///
/// // Hyphens are not an accepted separator.
/// assert!(NHSNumber::from_str("999-100-0003").is_err());
/// ```
///
/// Example — combined with `str::parse`:
///
/// ```rust
/// use nhs_number::NHSNumber;
///
/// let parsed: NHSNumber = "999 100 0003".parse().unwrap();
/// assert_eq!(parsed.digits, [9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
/// ```
///
impl FromStr for NHSNumber {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        match chars.len() {
            10 => {
                let mut digits: [i8; 10] = [0; 10];
                for i in 0..10 {
                    digits[i] = chars[i].to_digit(10).ok_or(ParseError)? as i8
                }
                Ok(NHSNumber { digits })
            }
            12 => {
                if chars[3] != ' ' || chars[7] != ' ' {
                    return Err(ParseError);
                }
                let mut digits: [i8; 10] = [0; 10];
                for i in 0..3 {
                    digits[i] = chars[i].to_digit(10).ok_or(ParseError)? as i8
                }
                for i in 0..3 {
                    digits[i + 3] = chars[i + 4].to_digit(10).ok_or(ParseError)? as i8
                }
                for i in 0..4 {
                    digits[i + 6] = chars[i + 8].to_digit(10).ok_or(ParseError)? as i8
                }
                Ok(NHSNumber { digits })
            }
            _ => Err(ParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------
    // Accepted shapes (§5.2)
    // ------------------------------------------------------------------

    #[test]
    fn test_from_str_with_length_10_without_spaces() {
        let s = String::from("0123456789");
        let actual: NHSNumber = NHSNumber::from_str(&s).unwrap();
        let expect: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_from_str_with_length_12_with_spaces() {
        let s = String::from("012 345 6789");
        let actual: NHSNumber = NHSNumber::from_str(&s).unwrap();
        let expect: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_from_str_with_str_parse_method() {
        // The `FromStr` impl makes `str::parse` work too.
        let n: NHSNumber = "999 100 0003".parse().unwrap();
        assert_eq!(n.digits, [9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
    }

    #[test]
    fn test_from_str_all_zeros_both_forms() {
        let tight = NHSNumber::from_str("0000000000").unwrap();
        let canonical = NHSNumber::from_str("000 000 0000").unwrap();
        assert_eq!(tight, canonical);
        assert_eq!(tight.digits, [0; 10]);
    }

    #[test]
    fn test_from_str_all_nines_both_forms() {
        let tight = NHSNumber::from_str("9999999999").unwrap();
        let canonical = NHSNumber::from_str("999 999 9999").unwrap();
        assert_eq!(tight, canonical);
        assert_eq!(tight.digits, [9; 10]);
    }

    #[test]
    fn test_from_str_round_trips_via_display() {
        // For every starting NHSNumber, parsing its `Display` output gives
        // the same value back.
        let fixtures = [
            NHSNumber::new([0; 10]),
            NHSNumber::new([9; 10]),
            NHSNumber::new([9, 4, 3, 4, 7, 6, 5, 9, 1, 9]),
            NHSNumber::new([9, 8, 7, 6, 5, 4, 4, 3, 2, 1]),
            NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]),
        ];
        for n in fixtures {
            let s = n.to_string();
            let parsed = NHSNumber::from_str(&s).unwrap();
            assert_eq!(parsed, n);
        }
    }

    // ------------------------------------------------------------------
    // Rejection cases (§5.3)
    // ------------------------------------------------------------------

    #[test]
    fn test_from_str_with_wrong_characters() {
        let s = String::from("012-345-6789");
        let result: Result<NHSNumber, ParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_with_wrong_leading_space() {
        let s = String::from(" 012 345 6789");
        let result: Result<NHSNumber, ParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_with_first_space_without_last_space() {
        let s = String::from("012 3456789");
        let result: Result<NHSNumber, ParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_without_first_space_with_last_space() {
        let s = String::from("012345 6789");
        let result: Result<NHSNumber, ParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_with_wrong_inner_space() {
        let s = String::from("012  345  6789");
        let result: Result<NHSNumber, ParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_with_wrong_trailing_space() {
        let s = String::from("012 345 6789 ");
        let result: Result<NHSNumber, ParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_with_wrong_length() {
        let s = String::from("012");
        let result: Result<NHSNumber, ParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_with_empty_string() {
        assert!(NHSNumber::from_str("").is_err());
    }

    #[test]
    fn test_from_str_with_length_11() {
        // One above ten, one below twelve — both rejected.
        assert!(NHSNumber::from_str("01234567890").is_err());
        assert!(NHSNumber::from_str("01234 56789").is_err());
    }

    #[test]
    fn test_from_str_with_length_13() {
        // One above twelve.
        assert!(NHSNumber::from_str("01234567890ab").is_err());
        assert!(NHSNumber::from_str("0123 4567 8901").is_err());
    }

    #[test]
    fn test_from_str_with_letters_only() {
        assert!(NHSNumber::from_str("abcdefghij").is_err());
        assert!(NHSNumber::from_str("abc def ghij").is_err());
    }

    #[test]
    fn test_from_str_with_letters_mixed_in() {
        // First group letters in an otherwise canonical-shaped string.
        assert!(NHSNumber::from_str("abc 123 4567").is_err());
        // Last group letters.
        assert!(NHSNumber::from_str("012 345 abcd").is_err());
    }

    #[test]
    fn test_from_str_with_nbsp_separators() {
        // U+00A0 NO-BREAK SPACE is two bytes in UTF-8 but one `char` —
        // length 12 in `chars().count()` but the separators are not the
        // ASCII space the parser checks for. Should reject.
        let s = "012\u{00A0}345\u{00A0}6789";
        assert_eq!(s.chars().count(), 12);
        assert!(NHSNumber::from_str(s).is_err());
    }

    #[test]
    fn test_from_str_with_tab_separators() {
        let s = "012\t345\t6789";
        assert_eq!(s.chars().count(), 12);
        assert!(NHSNumber::from_str(s).is_err());
    }

    #[test]
    fn test_from_str_with_arabic_indic_digits() {
        // Arabic-Indic digits U+0660..U+0669 are visually digits but
        // `char::to_digit(10)` only accepts ASCII '0'..='9'. Each is a
        // single `char`, so the length check passes (10), but every
        // digit conversion fails — overall result: rejected.
        let s = "٠١٢٣٤٥٦٧٨٩";
        assert_eq!(s.chars().count(), 10);
        assert!(NHSNumber::from_str(s).is_err());
    }

    #[test]
    fn test_from_str_with_negative_sign() {
        // The minus sign is not a digit and not a space.
        assert!(NHSNumber::from_str("-123456789").is_err());
    }

    #[test]
    fn test_from_str_with_plus_sign() {
        assert!(NHSNumber::from_str("+123456789").is_err());
    }

    #[test]
    fn test_from_str_with_decimal_point() {
        assert!(NHSNumber::from_str("12345.6789").is_err());
    }

    #[test]
    fn test_from_str_with_emoji() {
        // One multi-char emoji blows past the length check or fails digit
        // conversion — either way, reject.
        assert!(NHSNumber::from_str("0123🦀5678").is_err());
    }

    #[test]
    fn test_from_str_error_value_equals_parse_error() {
        // ParseError is a unit struct; all errors compare equal.
        let a = NHSNumber::from_str("bad").unwrap_err();
        let b = NHSNumber::from_str("also bad").unwrap_err();
        assert_eq!(a, b);
        assert_eq!(a, ParseError);
    }
}
