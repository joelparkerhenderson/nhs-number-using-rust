//! Opt-in serde wrapper that puts the **canonical string form** on the
//! wire instead of the default `{ "digits": [...] }` struct shape.
//!
//! Wrap an [`NHSNumber`] in [`NHSNumberString`] when your wire format
//! wants the human-readable `"DDD DDD DDDD"` form:
//!
//! - **Serialize** renders via [`Display`](std::fmt::Display) — always
//!   the twelve-character canonical form (`spec/05-string-forms.md` §5.1).
//! - **Deserialize** parses via [`FromStr`] — it
//!   accepts exactly the two documented shapes (`"DDDDDDDDDD"` and
//!   `"DDD DDD DDDD"`, `spec/05-string-forms.md` §5.2) and therefore **guarantees every
//!   digit is in `0..=9`**.
//!
//! The deserialisation error message never echoes the rejected input,
//! so a near-miss NHS Number cannot leak into logs through the error
//! path (see `AGENTS/safety.md` §3).
//!
//! See `spec/11-serialisation.md` §11.2 (rule R22) for the contract.
//!
//! Example — round-trip through JSON:
//!
//! ```rust
//! use nhs_number::NHSNumber;
//! use nhs_number::serde_string::NHSNumberString;
//! use std::str::FromStr;
//!
//! let n = NHSNumber::from_str("999 100 0003").unwrap();
//! let wrapped = NHSNumberString(n);
//!
//! let json = serde_json::to_string(&wrapped).unwrap();
//! assert_eq!(json, r#""999 100 0003""#);
//!
//! let back: NHSNumberString = serde_json::from_str(&json).unwrap();
//! assert_eq!(back.0, n);
//! ```

use crate::NHSNumber;
use crate::parse_error::ParseError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

/// Newtype that serialises the wrapped [`NHSNumber`] as its canonical
/// `"DDD DDD DDDD"` string and deserialises via the crate's
/// [`FromStr`] parser.
///
/// The inner value is a public tuple field, so wrapping and unwrapping
/// are plain constructor / field syntax:
///
/// ```rust
/// use nhs_number::NHSNumber;
/// use nhs_number::serde_string::NHSNumberString;
///
/// let n = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
/// let wrapped = NHSNumberString(n);
/// let unwrapped: NHSNumber = wrapped.0;
/// assert_eq!(unwrapped, n);
/// ```
///
/// `From` conversions are provided in both directions:
///
/// ```rust
/// use nhs_number::NHSNumber;
/// use nhs_number::serde_string::NHSNumberString;
///
/// let n = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
/// let wrapped = NHSNumberString::from(n);
/// let back = NHSNumber::from(wrapped);
/// assert_eq!(back, n);
/// ```
///
/// Deserialisation accepts both documented input shapes and rejects
/// everything else, exactly like [`FromStr`]:
///
/// ```rust
/// use nhs_number::serde_string::NHSNumberString;
///
/// let tight: NHSNumberString = serde_json::from_str(r#""9991000003""#).unwrap();
/// let canonical: NHSNumberString = serde_json::from_str(r#""999 100 0003""#).unwrap();
/// assert_eq!(tight, canonical);
///
/// assert!(serde_json::from_str::<NHSNumberString>(r#""999-100-0003""#).is_err());
/// ```
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NHSNumberString(pub NHSNumber);

impl From<NHSNumber> for NHSNumberString {
    fn from(n: NHSNumber) -> Self {
        NHSNumberString(n)
    }
}

impl From<NHSNumberString> for NHSNumber {
    fn from(w: NHSNumberString) -> Self {
        w.0
    }
}

/// Delegates to the wrapped [`NHSNumber`]'s canonical form.
impl fmt::Display for NHSNumberString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Delegates to the wrapped [`NHSNumber`]'s parser.
impl FromStr for NHSNumberString {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NHSNumber::from_str(s).map(NHSNumberString)
    }
}

/// Serialise as the canonical twelve-character string (`spec/05-string-forms.md` §5.1).
impl Serialize for NHSNumberString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.0)
    }
}

/// Deserialise from a string via [`FromStr`].
///
/// Accepts exactly the two documented shapes; the error message for a
/// rejected string never includes the string itself.
impl<'de> Deserialize<'de> for NHSNumberString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = NHSNumberString;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("an NHS Number string in \"DDDDDDDDDD\" or \"DDD DDD DDDD\" form")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // Deliberately no payload echo: the rejected candidate
                // must not leak into error messages or logs
                // (AGENTS/safety.md §3).
                NHSNumberString::from_str(v).map_err(|_| E::custom("invalid NHS Number string"))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_is_canonical_quoted_string() {
        let w = NHSNumberString(NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]));
        let actual = serde_json::to_string(&w).unwrap();
        let expect = r#""999 100 0003""#;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_deserialize_canonical_form() {
        let actual: NHSNumberString = serde_json::from_str(r#""999 100 0003""#).unwrap();
        let expect = NHSNumberString(NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]));
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_deserialize_tight_form() {
        let actual: NHSNumberString = serde_json::from_str(r#""9991000003""#).unwrap();
        let expect = NHSNumberString(NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]));
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_round_trip() {
        for digits in [[0; 10], [9; 10], [9, 4, 3, 4, 7, 6, 5, 9, 1, 9]] {
            let w = NHSNumberString(NHSNumber::new(digits));
            let json = serde_json::to_string(&w).unwrap();
            let back: NHSNumberString = serde_json::from_str(&json).unwrap();
            assert_eq!(back, w);
        }
    }

    #[test]
    fn test_deserialize_rejects_invalid_strings() {
        for bad in [
            r#""""#,
            r#""999-100-0003""#,
            r#"" 999 100 0003""#,
            r#""999 100 00030""#,
            r#""abc def ghij""#,
        ] {
            assert!(
                serde_json::from_str::<NHSNumberString>(bad).is_err(),
                "{bad} must be rejected"
            );
        }
    }

    #[test]
    fn test_deserialize_rejects_non_strings() {
        assert!(serde_json::from_str::<NHSNumberString>("9991000003").is_err());
        assert!(serde_json::from_str::<NHSNumberString>("null").is_err());
        assert!(
            serde_json::from_str::<NHSNumberString>(r#"{"digits":[9,9,9,1,0,0,0,0,0,3]}"#).is_err()
        );
    }

    #[test]
    fn test_deserialize_error_does_not_echo_input() {
        // Safety §3: a near-miss candidate must not leak through the
        // error path. The distinctive digit run must be absent from the
        // error's message.
        let err = serde_json::from_str::<NHSNumberString>(r#""999-100-0003""#).unwrap_err();
        let message = err.to_string();
        assert!(
            !message.contains("999-100-0003") && !message.contains("9991000003"),
            "error message must not echo the input: {message}"
        );
    }

    #[test]
    fn test_display_and_from_str_delegate() {
        let w = NHSNumberString(NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]));
        assert_eq!(w.to_string(), "999 100 0003");
        let parsed = NHSNumberString::from_str("999 100 0003").unwrap();
        assert_eq!(parsed, w);
    }

    #[test]
    fn test_from_conversions_round_trip() {
        let n = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
        let w = NHSNumberString::from(n);
        let back = NHSNumber::from(w);
        assert_eq!(back, n);
    }
}
