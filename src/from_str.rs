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
//! See [`spec/index.md`](https://github.com/joelparkerhenderson/nhs-number-using-rust/blob/main/spec/index.md)
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
        // Work on bytes, and reject on length before touching content.
        // Both accepted shapes are pure ASCII, where byte length equals
        // character length, so this accepts exactly the same set of
        // strings as a `char`-based parse — while rejecting arbitrarily
        // large untrusted input in O(1) with no allocation. Multi-byte
        // UTF-8 code units are always >= 0x80, so they can never pass
        // the ASCII digit and space checks below.
        let bytes = s.as_bytes();

        fn digit(b: u8) -> Result<i8, ParseError> {
            if b.is_ascii_digit() {
                Ok((b - b'0') as i8)
            } else {
                Err(ParseError)
            }
        }

        match bytes.len() {
            10 => {
                let mut digits: [i8; 10] = [0; 10];
                for i in 0..10 {
                    digits[i] = digit(bytes[i])?
                }
                Ok(NHSNumber { digits })
            }
            12 => {
                if bytes[3] != b' ' || bytes[7] != b' ' {
                    return Err(ParseError);
                }
                let mut digits: [i8; 10] = [0; 10];
                for i in 0..3 {
                    digits[i] = digit(bytes[i])?
                }
                for i in 0..3 {
                    digits[i + 3] = digit(bytes[i + 4])?
                }
                for i in 0..4 {
                    digits[i + 6] = digit(bytes[i + 8])?
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
        // 12 chars but 14 bytes, and in no reading are the separators the
        // ASCII space the parser requires. Should reject.
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
        // Arabic-Indic digits U+0660..U+0669 are visually digits but the
        // parser only accepts ASCII '0'..='9'. Ten of them is 10 chars
        // yet 20 bytes, and none is an ASCII digit — rejected either way.
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

    // ------------------------------------------------------------------
    // Adversarial / unverified input (§5.3, hardening)
    //
    // Inputs an attacker or a broken upstream could feed the parser.
    // Every one must be rejected quickly and without panicking.
    // ------------------------------------------------------------------

    #[test]
    fn test_from_str_with_embedded_nul_bytes() {
        // NUL is one byte, so these hit the accepted lengths — the digit
        // check must still reject them.
        assert!(NHSNumber::from_str("012345678\0").is_err());
        assert!(NHSNumber::from_str("\u{0}123456789").is_err());
        assert!(NHSNumber::from_str("012 345 678\0").is_err());
    }

    #[test]
    fn test_from_str_with_control_characters() {
        assert!(NHSNumber::from_str("01234567\r\n").is_err());
        assert!(NHSNumber::from_str("012 345 6789\n").is_err());
        assert!(NHSNumber::from_str("\u{8}123456789").is_err()); // backspace
        assert!(NHSNumber::from_str("\u{7f}123456789").is_err()); // DEL
    }

    #[test]
    fn test_from_str_with_bidi_override_characters() {
        // U+202E RIGHT-TO-LEFT OVERRIDE can visually disguise digit order
        // in surrounding text; it must never survive parsing.
        assert!(NHSNumber::from_str("\u{202E}0123456789").is_err());
        assert!(NHSNumber::from_str("0123456789\u{202E}").is_err());
    }

    #[test]
    fn test_from_str_with_multibyte_char_at_accepted_byte_length() {
        // "12345678é" is 9 chars but exactly 10 bytes — a byte-length
        // check alone would let it into the digit loop, so the loop must
        // reject the non-ASCII bytes.
        let s = "12345678é";
        assert_eq!(s.len(), 10);
        assert!(NHSNumber::from_str(s).is_err());

        // Same trick at length 12 with the separator positions occupied
        // by digit bytes of a multi-byte char.
        let s = "123 456 78é9";
        assert!(NHSNumber::from_str(s).is_err());
    }

    #[test]
    fn test_from_str_with_full_width_digits() {
        // Full-width digits U+FF10..U+FF19 look identical to ASCII in
        // many fonts. Ten of them; must reject.
        let s = "０１２３４５６７８９";
        assert_eq!(s.chars().count(), 10);
        assert!(NHSNumber::from_str(s).is_err());
    }

    #[test]
    fn test_from_str_with_superscript_and_lookalike_digits() {
        assert!(NHSNumber::from_str("¹²³⁴⁵⁶⁷⁸⁹⁰").is_err());
        // Lowercase L and letter O as 1/0 lookalikes.
        assert!(NHSNumber::from_str("0l234567l9").is_err());
        assert!(NHSNumber::from_str("O123456789").is_err());
    }

    #[test]
    fn test_from_str_with_huge_input_is_rejected_fast() {
        // Unverified input may be arbitrarily large. The parser must
        // reject on length before reading (or allocating for) the
        // content. 10 MiB of digits: rejected, no panic, effectively
        // instant.
        let huge = "1".repeat(10 * 1024 * 1024);
        assert!(NHSNumber::from_str(&huge).is_err());
    }

    #[test]
    fn test_from_str_with_all_byte_lengths_up_to_32() {
        // Only byte lengths 10 and 12 can ever parse; every other length
        // is rejected regardless of content.
        for len in 0..=32 {
            let s = "9".repeat(len);
            let result = NHSNumber::from_str(&s);
            if len == 10 {
                assert!(result.is_ok(), "all-digit length {len} must parse");
            } else {
                assert!(result.is_err(), "length {len} must be rejected");
            }
        }
    }

    /// Property-based fuzzing of the parser (spec/13-testing-strategy.md §13).
    ///
    /// These run a few hundred generated cases per test under `cargo
    /// test`. They assert the two load-bearing security properties:
    /// the parser never panics, and anything it accepts is exactly one
    /// of the two documented shapes.
    mod fuzz {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            /// The parser must never panic, whatever the input.
            #[test]
            fn never_panics_on_arbitrary_strings(s in "\\PC*") {
                let _ = NHSNumber::from_str(&s);
            }

            /// The parser must never panic on arbitrary bytes that
            /// happen to be valid UTF-8 of the accepted lengths.
            #[test]
            fn never_panics_on_length_10_and_12_strings(
                s in proptest::string::string_regex(".{10}|.{12}").unwrap()
            ) {
                let _ = NHSNumber::from_str(&s);
            }

            /// Anything accepted must round-trip: digits all in 0..=9
            /// and the canonical rendering re-parses to the same value.
            #[test]
            fn accepted_input_is_canonical(s in "\\PC*") {
                if let Ok(n) = NHSNumber::from_str(&s) {
                    for d in n.digits {
                        prop_assert!((0..=9).contains(&d));
                    }
                    let rendered = n.to_string();
                    prop_assert_eq!(NHSNumber::from_str(&rendered).unwrap(), n);
                    // The input itself must have been one of the two shapes.
                    let bytes = s.as_bytes();
                    prop_assert!(bytes.len() == 10 || bytes.len() == 12);
                }
            }

            /// Every ten-ASCII-digit string parses (tight form).
            #[test]
            fn tight_form_always_parses(s in "[0-9]{10}") {
                let n = NHSNumber::from_str(&s).unwrap();
                let tight: String =
                    n.digits.iter().map(|d| (b'0' + *d as u8) as char).collect();
                prop_assert_eq!(tight, s);
            }

            /// Every canonical "DDD DDD DDDD" string parses and
            /// re-renders to itself exactly.
            #[test]
            fn canonical_form_round_trips(s in "[0-9]{3} [0-9]{3} [0-9]{4}") {
                let n = NHSNumber::from_str(&s).unwrap();
                prop_assert_eq!(n.to_string(), s);
            }

            /// Mutating a canonical string anywhere outside the two
            /// shapes must reject: replace any one position with a
            /// non-digit, non-space ASCII byte.
            #[test]
            fn corrupted_canonical_form_is_rejected(
                s in "[0-9]{3} [0-9]{3} [0-9]{4}",
                pos in 0usize..12,
                bad in "[!-/:-~]" // printable ASCII, no digits, no space
            ) {
                let mut bytes = s.into_bytes();
                bytes[pos] = bad.as_bytes()[0];
                let corrupted = String::from_utf8(bytes).unwrap();
                prop_assert!(NHSNumber::from_str(&corrupted).is_err());
            }
        }
    }
}
