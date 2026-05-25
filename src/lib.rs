//! # NHS Number
//!
//! **[documentation](https://docs.rs/nhs-number/)**
//! •
//! **[source](https://github.com/joelparkerhenderson/nhs-number)**
//! •
//! **[llms.txt](https://raw.githubusercontent.com/joelparkerhenderson/nhs-number/refs/heads/main/llms.txt)**
//! •
//! **[crate](https://crates.io/crates/nhs-number)**
//! •
//! **[email](mailto:joel@joelparkerhenderson.com)**
//!
//! A National Health Service (NHS) Number is a unique number allocated in a shared
//! numbering scheme to registered users of the public health services in
//! England and the Isle of Man.
//!
//! The NHS Number is the key to the identification of patients, especially in
//! delivering safe care across provider organisations, and is required in all new
//! software deployed within the National Health Service (NHS) organisations.
//!
//! References:
//!
//! * [National Health Service (NHS)](https://en.wikipedia.org/wiki/National_Health_Service)
//!
//! * [NHS Number](https://en.wikipedia.org/wiki/NHS_number)
//!
//! ## Syntax
//!
//! The current system uses a ten-digit number in '3 3 4' format with the final
//! digit being an error-detecting checksum. Examples given include 987 654 4321.
//!
//! ## Ranges
//!
//! Currently issued numbers are in these ranges:
//!
//! * 300 000 000 to 399 999 999 (England)
//!
//! * 400 000 000 to 499 999 999 (England, Isle of Man)
//!
//! * 600 000 000 to 799 999 999 (England, Isle of Man)
//!
//! Unavailable number ranges include:
//!
//! * 320 000 001 to 399 999 999 (allocated to the Northern Irish system)
//!
//! * 010 100 0000 to 311 299 9999 (used for CHI numbers in Scotland)
//!
//! For test purposes, this range is valid but is guaranteed to never be issued:
//!
//! * 999 000 0000 to 999 999 9999
//!
//! ## Checksum
//!
//! The checksum is calculated by multiplying each of the first nine digits by 11
//! minus its position. Using the number 943 476 5919 as an example:
//!
//! * The first digit is 9. This is multiplied by 10.
//!
//! * The second digit is 4. This is multiplied by 9.
//!
//! * And so on until the ninth digit (1) is multiplied by 2.
//!
//! * The result of this calculation is summed. In this example: (9×10) + (4×9) +
//!   (3×8) + (4×7) + (7×6) + (6×5) + (5×4) + (9×3) + (1×2) = 299.
//!
//! * The remainder when dividing this number by 11 is calculated, yielding a number
//!   in the range 0–10, which would be 2 in this case.
//!
//! * Finally, this number is subtracted from 11 to give the checksum in the range
//!   1–11, in this case 9, which becomes the last digit of the NHS Number.
//!
//! * A checksum of 11 is represented by 0 in the final NHS Number. If the checksum
//!   is 10 then the number is not valid.
//!
//! ## Quick start
//!
//! Parse, validate, and re-render an NHS Number:
//!
//! ```rust
//! use nhs_number::NHSNumber;
//! use std::str::FromStr;
//!
//! // A test-safe number drawn from the reserved testable range.
//! let input = "999 100 0003";
//!
//! let n = NHSNumber::from_str(input).unwrap();
//! assert!(n.validate_check_digit());
//! assert_eq!(n.to_string(), input);
//! ```
//!
//! Compute a check digit from raw digits:
//!
//! ```rust
//! // The Wikipedia worked example: digits 9 4 3 4 7 6 5 9 1 → check digit 9.
//! let digits = [9, 4, 3, 4, 7, 6, 5, 9, 1, 9];
//! assert_eq!(nhs_number::calculate_check_digit(digits), 9);
//! assert!(nhs_number::validate_check_digit(digits));
//! ```
//!
//! Generate a sample for tests:
//!
//! ```rust
//! use nhs_number::{NHSNumber, testable::TESTABLE_RANGE_INCLUSIVE};
//!
//! let sample = NHSNumber::testable_random_sample();
//! assert!(TESTABLE_RANGE_INCLUSIVE.contains(&sample));
//! ```
//!
//! ## Public surface (at a glance)
//!
//! - Value type: [`NHSNumber`]
//! - Parsing: `<NHSNumber as FromStr>::from_str` — see [`mod@from_str`]
//! - Errors: [`parse_error::ParseError`]
//! - Check-digit functions: [`check_digit`], [`calculate_check_digit`],
//!   [`validate_check_digit`] (and matching [`NHSNumber`] methods)
//! - Formatting: [`format()`], [`Display`](std::fmt::Display) (and
//!   [`Into<String>`])
//! - Testable range: [`mod@testable`] with [`testable::TESTABLE_MIN`],
//!   [`testable::TESTABLE_MAX`], [`testable::TESTABLE_RANGE_INCLUSIVE`],
//!   and [`testable::testable_random_sample`] (re-exported at the crate
//!   root)
//!
//! ## Specification
//!
//! The canonical behavioural specification for this crate lives in
//! `spec.md` at the repo root. When the spec and the code disagree, the
//! spec is the source of truth; see `AGENTS/spec-driven-development.md`
//! for how to evolve both.
//!
use serde::{Deserialize, Serialize};
use std::fmt;

pub mod from_str;
pub mod parse_error;
pub mod testable;
pub use testable::*;

/// A ten-digit NHS Number — the unique identifier allocated to patients
/// of the National Health Service of England and the Isle of Man.
///
/// The struct wraps a fixed-length `[i8; 10]` digit array. Each element
/// is expected to be in `0..=9`; the parser ([`FromStr`](std::str::FromStr))
/// enforces this for caller-supplied strings, but
/// [`new`](NHSNumber::new) and the public `digits` field do not.
///
/// References:
///
/// - [National Health Service (NHS)](https://en.wikipedia.org/wiki/National_Health_Service)
/// - [NHS Number](https://en.wikipedia.org/wiki/NHS_number)
///
/// # Construction
///
/// Use either [`NHSNumber::new`] or a struct literal — the `digits` field
/// is public, and both forms produce the same value:
///
/// ```rust
/// use nhs_number::NHSNumber;
///
/// let via_new = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
/// let via_literal = NHSNumber { digits: [9, 9, 9, 1, 0, 0, 0, 0, 0, 3] };
/// assert_eq!(via_new, via_literal);
/// ```
///
/// # Parse, validate, display
///
/// Parse from a string with [`FromStr`](std::str::FromStr); validate the
/// check digit; render back to the canonical `"DDD DDD DDDD"` form:
///
/// ```rust
/// use nhs_number::NHSNumber;
/// use std::str::FromStr;
///
/// let n = NHSNumber::from_str("999 100 0003").unwrap();
/// assert!(n.validate_check_digit());
/// assert_eq!(n.to_string(), "999 100 0003");
/// ```
///
/// # Ordering and equality
///
/// `NHSNumber` derives `PartialEq`/`Eq` (element-wise) and `PartialOrd`/`Ord`
/// (lexicographic on the digit array — which matches natural numeric
/// ordering since every value has the same width and the most-significant
/// digit comes first):
///
/// ```rust
/// use nhs_number::NHSNumber;
///
/// let a = NHSNumber::new([0; 10]);
/// let b = NHSNumber::new([9; 10]);
/// assert!(a < b);
/// ```
///
/// This makes `NHSNumber` directly usable as a `BTreeSet` element or a
/// `BTreeMap` key. (It does not derive `Hash`; see `spec.md` §18.1.)
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct NHSNumber {
    pub digits: [i8; 10],
}

impl NHSNumber {
    /// Construct an `NHSNumber` from a ten-digit array.
    ///
    /// Equivalent to a struct-literal construction
    /// (`NHSNumber { digits: ... }`). No bounds checking is performed on
    /// the digits — callers handling untrusted input should parse with
    /// [`FromStr`](std::str::FromStr) instead.
    ///
    /// Example:
    ///
    /// ```rust
    /// use nhs_number::NHSNumber;
    ///
    /// let n = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
    /// assert_eq!(n.digits, [9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
    /// assert_eq!(n.to_string(), "999 100 0003");
    /// ```
    ///
    #[allow(dead_code)]
    pub fn new(digits: [i8; 10]) -> Self {
        NHSNumber { digits }
    }

    /// Return the **stored** tenth digit — the check digit as it appears
    /// in this `NHSNumber`.
    ///
    /// This is a pure accessor: it does not compute or validate anything.
    /// Use [`calculate_check_digit`](NHSNumber::calculate_check_digit) to
    /// derive the digit from the first nine, and
    /// [`validate_check_digit`](NHSNumber::validate_check_digit) to compare
    /// the two.
    ///
    /// Example:
    ///
    /// ```rust
    /// use nhs_number::NHSNumber;
    ///
    /// let n = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
    /// assert_eq!(n.check_digit(), 3);
    /// ```
    ///
    /// This method calls the function [`crate::check_digit`].
    ///
    #[allow(dead_code)]
    pub fn check_digit(&self) -> i8 {
        crate::check_digit(self.digits)
    }

    /// Compute the check digit that the first nine digits **should** have,
    /// using the NHS modulo-11 algorithm.
    ///
    /// Algorithm (see `spec.md` §6):
    ///
    /// 1. Weight each of `digits[0..9]` by `10 − i` and sum the products.
    /// 2. Take the sum modulo 11; subtract from 11 to get a raw value in
    ///    `1..=11`.
    /// 3. Map: `11 → 0`, `10 → sentinel`, otherwise the raw value.
    ///
    /// **Returns `10` as a sentinel** meaning "no digit in `0..=9` fits"
    /// — that case is `sum % 11 == 1`. Because every stored check digit is
    /// in `0..=9`, the sentinel can never equal
    /// [`check_digit`](NHSNumber::check_digit), so
    /// [`validate_check_digit`](NHSNumber::validate_check_digit) correctly
    /// reports `false` for those numbers.
    ///
    /// Example — typical case:
    ///
    /// ```rust
    /// use nhs_number::NHSNumber;
    ///
    /// let n = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
    /// assert_eq!(n.calculate_check_digit(), 3);
    /// ```
    ///
    /// Example — Wikipedia reference number:
    ///
    /// ```rust
    /// use nhs_number::NHSNumber;
    ///
    /// let n = NHSNumber::new([9, 4, 3, 4, 7, 6, 5, 9, 1, 9]);
    /// assert_eq!(n.calculate_check_digit(), 9);
    /// ```
    ///
    /// Example — sentinel `10` for an unfittable number:
    ///
    /// ```rust
    /// use nhs_number::NHSNumber;
    ///
    /// // 999 123 4560 — weighted sum 320, 320 % 11 == 1, so no digit fits.
    /// let n = NHSNumber::new([9, 9, 9, 1, 2, 3, 4, 5, 6, 0]);
    /// assert_eq!(n.calculate_check_digit(), 10);
    /// ```
    ///
    /// This method calls the function [`crate::calculate_check_digit`].
    ///
    #[allow(dead_code)]
    pub fn calculate_check_digit(&self) -> i8 {
        crate::calculate_check_digit(self.digits)
    }

    /// Return `true` iff the stored check digit matches the one the NHS
    /// algorithm would compute from the first nine digits.
    ///
    /// Returns `false` when:
    ///
    /// - the stored and calculated digits simply differ, **or**
    /// - the calculated digit is the sentinel `10` (no digit can fit; see
    ///   [`calculate_check_digit`](NHSNumber::calculate_check_digit)).
    ///
    /// A passing `validate_check_digit` confirms **arithmetic
    /// self-consistency only** — not that the number identifies any real
    /// patient. See `spec.md` §10.
    ///
    /// Example — a valid number:
    ///
    /// ```rust
    /// use nhs_number::NHSNumber;
    ///
    /// let valid = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
    /// assert!(valid.validate_check_digit());
    /// ```
    ///
    /// Example — a wrong stored check digit fails validation:
    ///
    /// ```rust
    /// use nhs_number::NHSNumber;
    ///
    /// let wrong = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 4]);
    /// assert!(!wrong.validate_check_digit());
    /// ```
    ///
    /// Example — every stored tenth digit is invalid when no digit fits:
    ///
    /// ```rust
    /// use nhs_number::NHSNumber;
    ///
    /// for d in 0i8..=9 {
    ///     let n = NHSNumber::new([9, 9, 9, 1, 2, 3, 4, 5, 6, d]);
    ///     assert!(!n.validate_check_digit());
    /// }
    /// ```
    ///
    /// This method calls the function [`crate::validate_check_digit`].
    ///
    #[allow(dead_code)]
    pub fn validate_check_digit(&self) -> bool {
        crate::validate_check_digit(self.digits)
    }

    /// Return a random `NHSNumber` from the reserved **testable** range
    /// `999 000 0000 – 999 999 9999`.
    ///
    /// The first three digits are always `9, 9, 9`; the remaining seven
    /// are drawn uniformly from `0..=9` using [`rand::rng`]. The tenth
    /// (check) digit is drawn randomly, **not** computed — so roughly
    /// nine in ten samples have an invalid check digit. That is
    /// intentional; it lets tests exercise the rejection branch of
    /// `validate_check_digit`.
    ///
    /// Example — the sample is always in the testable range:
    ///
    /// ```rust
    /// use nhs_number::{NHSNumber, testable::{TESTABLE_MIN, TESTABLE_MAX}};
    ///
    /// let sample = NHSNumber::testable_random_sample();
    /// assert!(sample >= *TESTABLE_MIN);
    /// assert!(sample <= *TESTABLE_MAX);
    /// assert_eq!(&sample.digits[0..3], &[9, 9, 9]);
    /// ```
    ///
    /// Example — get a sample that is *also* valid by check digit:
    ///
    /// ```rust
    /// use nhs_number::NHSNumber;
    ///
    /// let valid_sample = std::iter::from_fn(|| {
    ///     Some(NHSNumber::testable_random_sample())
    /// })
    /// .find(|n| n.validate_check_digit())
    /// .unwrap();
    /// assert!(valid_sample.validate_check_digit());
    /// ```
    ///
    /// This method calls the function [`crate::testable_random_sample`].
    ///
    #[allow(dead_code)]
    pub fn testable_random_sample() -> NHSNumber {
        crate::testable_random_sample()
    }
}

/// Format an `NHSNumber` as the canonical `"DDD DDD DDDD"` string —
/// three digits, single space, three digits, single space, four digits.
///
/// The output is **always** twelve characters wide. There are no
/// alternative separators; if you need a different layout, post-process
/// the string yourself.
///
/// Example:
///
/// ```rust
/// use nhs_number::NHSNumber;
///
/// let n = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
/// assert_eq!(format!("{}", n), "999 100 0003");
/// assert_eq!(n.to_string(), "999 100 0003");
/// assert_eq!(n.to_string().len(), 12);
/// ```
///
/// `Display` round-trips with [`FromStr`](std::str::FromStr):
///
/// ```rust
/// use nhs_number::NHSNumber;
/// use std::str::FromStr;
///
/// let n = NHSNumber::new([9, 4, 3, 4, 7, 6, 5, 9, 1, 9]);
/// let parsed = NHSNumber::from_str(&n.to_string()).unwrap();
/// assert_eq!(n, parsed);
/// ```
///
/// Equivalent to the free function [`crate::format`].
///
impl fmt::Display for NHSNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{} {}{}{} {}{}{}{}",
            self.digits[0],
            self.digits[1],
            self.digits[2],
            self.digits[3],
            self.digits[4],
            self.digits[5],
            self.digits[6],
            self.digits[7],
            self.digits[8],
            self.digits[9],
        )
    }
}

/// Convert an `NHSNumber` into its canonical `String` form by delegating
/// to [`Display`](fmt::Display).
///
/// Because of the standard-library blanket
/// `impl<T, U> Into<U> for T where U: From<T>`, this `From` impl also
/// gives callers `Into<String> for NHSNumber` — both `String::from(n)`
/// and `let s: String = n.into();` work and produce the same value as
/// `n.to_string()`.
///
/// Example — `String::from`:
///
/// ```rust
/// use nhs_number::NHSNumber;
///
/// let n = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// let s = String::from(n);
/// assert_eq!(s, "012 345 6789");
/// ```
///
/// Example — `Into<String>` via the blanket impl:
///
/// ```rust
/// use nhs_number::NHSNumber;
///
/// let n = NHSNumber::new([9, 4, 3, 4, 7, 6, 5, 9, 1, 9]);
/// let via_into: String = n.into();
/// assert_eq!(via_into, n.to_string());
/// ```
///
impl From<NHSNumber> for String {
    fn from(n: NHSNumber) -> String {
        n.to_string()
    }
}

// ---------------------------------------------------------------------------
// Free-function utilities
//
// Every free function below mirrors a method on `NHSNumber`. Use the free
// function when you already hold a raw `[i8; 10]` and do not want to build
// the wrapper struct; use the method when you have an `NHSNumber`. The two
// forms always return the same value on the same input — this is an
// invariant enforced by `tests::properties` in this file.
// ---------------------------------------------------------------------------

/// Format a ten-digit array as the canonical `"DDD DDD DDDD"` string.
///
/// Equivalent to [`NHSNumber::to_string`](std::string::ToString::to_string)
/// (via the [`Display`](fmt::Display) impl) and to the [`Into<String>`]
/// impl on `NHSNumber`. The two forms always agree.
///
/// Example:
///
/// ```rust
/// let s = nhs_number::format([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// assert_eq!(s, "012 345 6789");
/// ```
///
/// Example — equivalence with the [`Display`](fmt::Display) impl:
///
/// ```rust
/// use nhs_number::NHSNumber;
///
/// let digits = [9, 9, 9, 1, 0, 0, 0, 0, 0, 3];
/// assert_eq!(nhs_number::format(digits), NHSNumber::new(digits).to_string());
/// ```
///
#[allow(dead_code)]
pub fn format(digits: [i8; 10]) -> String {
    format!(
        "{}{}{} {}{}{} {}{}{}{}",
        digits[0],
        digits[1],
        digits[2],
        digits[3],
        digits[4],
        digits[5],
        digits[6],
        digits[7],
        digits[8],
        digits[9],
    )
}

/// Return the **stored** tenth digit of a ten-digit array — the check
/// digit as it appears in the input.
///
/// This is a pure accessor: it does not compute or validate anything.
/// Equivalent to [`NHSNumber::check_digit`] on the corresponding
/// `NHSNumber`.
///
/// Example:
///
/// ```rust
/// let digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
/// assert_eq!(nhs_number::check_digit(digits), 9);
/// ```
///
/// Example — equivalence with [`NHSNumber::check_digit`]:
///
/// ```rust
/// use nhs_number::NHSNumber;
///
/// let digits = [9, 9, 9, 1, 0, 0, 0, 0, 0, 3];
/// let n = NHSNumber::new(digits);
/// assert_eq!(nhs_number::check_digit(digits), n.check_digit());
/// ```
///
#[allow(dead_code)]
pub fn check_digit(digits: [i8; 10]) -> i8 {
    digits[9]
}

/// Compute the check digit for a ten-digit array using the NHS modulo-11
/// algorithm (see `spec.md` §6).
///
/// **Algorithm:**
///
/// 1. Weight each of `digits[0..9]` by `10 − i` and sum the products.
/// 2. Take `sum % 11`; let `raw = 11 − (sum % 11)`, so `raw ∈ 1..=11`.
/// 3. Map: `raw == 11` → `0`; `raw == 10` → sentinel `10`; otherwise `raw`.
///
/// **Return value `10` is a sentinel** meaning "no digit in `0..=9` can
/// stand in as the check digit" — that case occurs when `sum % 11 == 1`.
/// Because a stored check digit is always in `0..=9`, the sentinel can
/// never equal a stored value, so [`validate_check_digit`] correctly
/// returns `false` for such numbers.
///
/// The function is **infallible on in-domain input** (every digit in
/// `0..=9`) and does **no** bounds checking — out-of-range elements are
/// undefined behaviour at the spec level.
///
/// Example — typical case (`999 100 0003`, testable range):
///
/// ```rust
/// let digits = [9, 9, 9, 1, 0, 0, 0, 0, 0, 3];
/// assert_eq!(nhs_number::calculate_check_digit(digits), 3);
/// ```
///
/// Example — Wikipedia reference number `943 476 5919`:
///
/// ```rust
/// let digits = [9, 4, 3, 4, 7, 6, 5, 9, 1, 9];
/// assert_eq!(nhs_number::calculate_check_digit(digits), 9);
/// ```
///
/// Example — the `raw == 11` branch (sum congruent to 0):
///
/// ```rust
/// // `999 100 0100` — weighted sum 253, 253 % 11 == 0.
/// let digits = [9, 9, 9, 1, 0, 0, 0, 1, 0, 0];
/// assert_eq!(nhs_number::calculate_check_digit(digits), 0);
/// ```
///
/// Example — the sentinel branch (sum congruent to 1):
///
/// ```rust
/// // `999 123 4560` — weighted sum 320, 320 % 11 == 1, so no digit fits.
/// let digits = [9, 9, 9, 1, 2, 3, 4, 5, 6, 0];
/// assert_eq!(nhs_number::calculate_check_digit(digits), 10);
/// ```
///
/// Equivalent to [`NHSNumber::calculate_check_digit`] on the corresponding
/// `NHSNumber`.
///
#[allow(dead_code)]
pub fn calculate_check_digit(digits: [i8; 10]) -> i8 {
    let sum: usize = digits
        .iter()
        .take(9)
        .enumerate()
        .map(|(i, &d)| d as usize * (10 - i))
        .sum();
    let raw = 11 - (sum % 11);
    if raw == 11 { 0 } else { raw as i8 }
}

/// Return `true` iff the stored tenth digit equals the one
/// [`calculate_check_digit`] would compute from the first nine.
///
/// Returns `false` when:
///
/// - the stored and calculated digits simply differ, **or**
/// - the calculated value is the sentinel `10` (no digit can stand in;
///   see [`calculate_check_digit`]).
///
/// Equivalent to [`NHSNumber::validate_check_digit`] on the corresponding
/// `NHSNumber`.
///
/// Example — a valid testable number:
///
/// ```rust
/// let digits = [9, 9, 9, 1, 0, 0, 0, 0, 0, 3];
/// assert!(nhs_number::validate_check_digit(digits));
/// ```
///
/// Example — a wrong stored check digit:
///
/// ```rust
/// let digits = [9, 9, 9, 1, 0, 0, 0, 0, 0, 4]; // last digit should be 3
/// assert!(!nhs_number::validate_check_digit(digits));
/// ```
///
/// Example — the "no digit fits" branch (`sum % 11 == 1`):
///
/// ```rust
/// let digits = [9, 9, 9, 1, 2, 3, 4, 5, 6, 0];
/// assert!(!nhs_number::validate_check_digit(digits));
/// ```
///
#[allow(dead_code)]
pub fn validate_check_digit(digits: [i8; 10]) -> bool {
    crate::check_digit(digits) == crate::calculate_check_digit(digits)
}

#[cfg(test)]
mod tests {

    mod structure {
        use super::super::*;

        #[test]
        fn test_new() {
            let a: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
            let actual = a.to_string();
            let expect = "012 345 6789";
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_new_preserves_digits() {
            let digits = [9, 4, 3, 4, 7, 6, 5, 9, 1, 9];
            let n = NHSNumber::new(digits);
            assert_eq!(n.digits, digits);
        }

        #[test]
        fn test_struct_literal_construction() {
            let a = NHSNumber {
                digits: [9, 9, 9, 1, 0, 0, 0, 0, 0, 3],
            };
            let b = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
            assert_eq!(a, b);
        }

        #[test]
        fn test_display() {
            let a: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
            let actual = a.to_string();
            let expect = "012 345 6789";
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_display_length_is_always_twelve() {
            for first in 0..=9 {
                let n = NHSNumber::new([first as i8; 10]);
                assert_eq!(n.to_string().chars().count(), 12);
            }
        }

        #[test]
        fn test_display_spaces_at_positions_3_and_7() {
            let n = NHSNumber::new([9, 4, 3, 4, 7, 6, 5, 9, 1, 9]);
            let s = n.to_string();
            let bytes = s.as_bytes();
            assert_eq!(bytes[3], b' ');
            assert_eq!(bytes[7], b' ');
            for (i, b) in bytes.iter().enumerate() {
                if i == 3 || i == 7 {
                    continue;
                }
                assert!(b.is_ascii_digit(), "byte at {i} should be ASCII digit");
            }
        }

        #[test]
        fn test_into_string() {
            let a: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
            let actual: String = a.into();
            let expect = "012 345 6789";
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_string_from() {
            let n = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
            let actual = String::from(n);
            let expect = "012 345 6789";
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_into_string_agrees_with_display() {
            let n = NHSNumber::new([9, 4, 3, 4, 7, 6, 5, 9, 1, 9]);
            let via_into: String = n.into();
            let via_display = n.to_string();
            assert_eq!(via_into, via_display);
        }

        #[test]
        fn test_string_from_agrees_with_display_and_into() {
            // R18: From<NHSNumber> for String, Into<String> (blanket),
            // and Display all produce the same value.
            let n = NHSNumber::new([9, 4, 3, 4, 7, 6, 5, 9, 1, 9]);
            let via_from = String::from(n);
            let via_into: String = n.into();
            let via_display = n.to_string();
            assert_eq!(via_from, via_display);
            assert_eq!(via_into, via_display);
            assert_eq!(via_from, via_into);
        }

        #[test]
        fn test_partial_eq() {
            {
                let a: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
                let b: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
                assert_eq!(a, b);
            }
            {
                let a: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
                let b: NHSNumber = NHSNumber::new([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
                assert_ne!(a, b);
            }
        }

        #[test]
        fn test_partial_eq_per_position() {
            // Any single-position difference must break equality.
            let base = [0i8; 10];
            for i in 0..10 {
                let mut other = base;
                other[i] = 1;
                assert_ne!(NHSNumber::new(base), NHSNumber::new(other));
            }
        }

        #[test]
        fn test_check_digit() {
            let a = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
            let actual: i8 = a.check_digit();
            let expect: i8 = 9;
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_check_digit_reads_tenth_position() {
            for tenth in 0i8..=9 {
                let n = NHSNumber::new([0, 0, 0, 0, 0, 0, 0, 0, 0, tenth]);
                assert_eq!(n.check_digit(), tenth);
            }
        }

        #[test]
        fn test_calculate_check_digit() {
            // `999 100 0003` — typical case: raw in 1..=9.
            let a: NHSNumber = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
            assert_eq!(a.calculate_check_digit(), 3);

            // `943 476 5919` — Wikipedia reference number.
            let b: NHSNumber = NHSNumber::new([9, 4, 3, 4, 7, 6, 5, 9, 1, 9]);
            assert_eq!(b.calculate_check_digit(), 9);

            // raw == 11 case: `sum % 11 == 0` must map to check digit 0.
            // `999 100 0100` — weighted sum 253, 253 % 11 == 0.
            let c: NHSNumber = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 1, 0, 0]);
            assert_eq!(c.calculate_check_digit(), 0);

            // raw == 10 case: `sum % 11 == 1` must return sentinel 10.
            // `999 123 4560` — weighted sum 320, 320 % 11 == 1.
            let d: NHSNumber = NHSNumber::new([9, 9, 9, 1, 2, 3, 4, 5, 6, 0]);
            assert_eq!(d.calculate_check_digit(), 10);
        }

        #[test]
        fn test_calculate_check_digit_ignores_tenth_position() {
            // Mutating only digit[9] must not change the calculated value.
            for stored in 0i8..=9 {
                let n = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, stored]);
                assert_eq!(n.calculate_check_digit(), 3);
            }
        }

        #[test]
        fn test_validate_check_digit() {
            {
                // Valid by strict NHS spec.
                let a: NHSNumber = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
                assert!(a.validate_check_digit());
            }
            {
                // Same first nine digits, wrong stored check digit.
                let a: NHSNumber = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 4]);
                assert!(!a.validate_check_digit());
            }
            {
                // `sum % 11 == 1` → no digit fits → must be invalid for
                // every stored tenth digit in 0..=9.
                for stored in 0i8..=9 {
                    let n = NHSNumber::new([9, 9, 9, 1, 2, 3, 4, 5, 6, stored]);
                    assert!(
                        !n.validate_check_digit(),
                        "999 123 456{stored} must be invalid (sum % 11 == 1)"
                    );
                }
            }
        }

        #[test]
        fn test_testable_random_sample() {
            let a: NHSNumber = NHSNumber::testable_random_sample();
            assert!(a >= *crate::testable::TESTABLE_MIN);
            assert!(a <= *crate::testable::TESTABLE_MAX);
        }

        #[test]
        fn test_testable_random_sample_starts_with_999() {
            for _ in 0..32 {
                let n = NHSNumber::testable_random_sample();
                assert_eq!(&n.digits[0..3], &[9, 9, 9]);
            }
        }
    }

    mod utilities {

        #[test]
        fn test_format() {
            let digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            let actual = crate::format(digits);
            let expect = "012 345 6789";
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_format_all_zeros() {
            assert_eq!(crate::format([0; 10]), "000 000 0000");
        }

        #[test]
        fn test_format_all_nines() {
            assert_eq!(crate::format([9; 10]), "999 999 9999");
        }

        #[test]
        fn test_check_digit() {
            let digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            let actual: i8 = crate::check_digit(digits);
            let expect: i8 = 9;
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_calculate_check_digit() {
            // Typical case.
            let digits = [9, 9, 9, 1, 0, 0, 0, 0, 0, 3];
            assert_eq!(crate::calculate_check_digit(digits), 3);

            // raw == 11 → 0.
            let digits = [9, 9, 9, 1, 0, 0, 0, 1, 0, 0];
            assert_eq!(crate::calculate_check_digit(digits), 0);

            // raw == 10 → sentinel 10 (no digit fits).
            let digits = [9, 9, 9, 1, 2, 3, 4, 5, 6, 0];
            assert_eq!(crate::calculate_check_digit(digits), 10);
        }

        #[test]
        fn test_calculate_check_digit_all_zeros() {
            // sum = 0, 0 % 11 == 0, raw == 11, check digit 0.
            assert_eq!(crate::calculate_check_digit([0; 10]), 0);
        }

        #[test]
        fn test_validate_check_digit_free_fn() {
            assert!(crate::validate_check_digit([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]));
            assert!(!crate::validate_check_digit([9, 9, 9, 1, 0, 0, 0, 0, 0, 4]));
        }
    }

    /// Property tests — invariants that must hold across many inputs.
    /// These back §5.4 (round-trip) and §4.2 (free-fn / method equivalence)
    /// of `spec.md`.
    mod properties {
        use super::super::*;
        use std::str::FromStr;

        /// Sample fixtures that span the interesting input space:
        /// boundary values, the Wikipedia reference numbers, and several
        /// numbers from the testable range.
        fn sample_fixtures() -> Vec<NHSNumber> {
            vec![
                NHSNumber::new([0; 10]),
                NHSNumber::new([9; 10]),
                NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
                NHSNumber::new([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
                NHSNumber::new([9, 4, 3, 4, 7, 6, 5, 9, 1, 9]), // 943 476 5919
                NHSNumber::new([9, 8, 7, 6, 5, 4, 4, 3, 2, 1]), // 987 654 4321
                NHSNumber::new([9, 9, 9, 0, 0, 0, 0, 0, 0, 0]), // TESTABLE_MIN
                NHSNumber::new([9, 9, 9, 9, 9, 9, 9, 9, 9, 9]), // TESTABLE_MAX
                NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]), // 999 100 0003
            ]
        }

        #[test]
        fn round_trip_via_canonical_form() {
            for n in sample_fixtures() {
                let s = n.to_string();
                let parsed = NHSNumber::from_str(&s).expect("display form must parse");
                assert_eq!(parsed, n, "round-trip failed for {s}");
            }
        }

        #[test]
        fn round_trip_via_tight_form() {
            for n in sample_fixtures() {
                let tight: String = n.digits.iter().map(|d| (b'0' + *d as u8) as char).collect();
                let parsed = NHSNumber::from_str(&tight).expect("tight form must parse");
                assert_eq!(parsed, n, "round-trip failed for {tight}");
            }
        }

        #[test]
        fn method_and_free_fn_format_agree() {
            for n in sample_fixtures() {
                assert_eq!(n.to_string(), crate::format(n.digits));
            }
        }

        #[test]
        fn method_and_free_fn_check_digit_agree() {
            for n in sample_fixtures() {
                assert_eq!(n.check_digit(), crate::check_digit(n.digits));
            }
        }

        #[test]
        fn method_and_free_fn_calculate_check_digit_agree() {
            for n in sample_fixtures() {
                assert_eq!(
                    n.calculate_check_digit(),
                    crate::calculate_check_digit(n.digits)
                );
            }
        }

        #[test]
        fn method_and_free_fn_validate_check_digit_agree() {
            for n in sample_fixtures() {
                assert_eq!(
                    n.validate_check_digit(),
                    crate::validate_check_digit(n.digits)
                );
            }
        }

        #[test]
        fn calculate_check_digit_is_in_valid_range() {
            // For every input, the function returns either a real digit
            // (0..=9) or the sentinel 10 — never anything else.
            for n in sample_fixtures() {
                let c = n.calculate_check_digit();
                assert!((0..=10).contains(&c), "{c} out of [0..=10]");
            }
        }
    }

    /// Boundary tests — explicit coverage of §6.5's three branches.
    mod boundaries {
        use super::super::*;

        #[test]
        fn sum_mod_11_eq_0_yields_check_digit_zero() {
            // `999 100 0100` — weighted sum 253, 253 % 11 == 0.
            let digits = [9, 9, 9, 1, 0, 0, 0, 1, 0, 0];
            assert_eq!(crate::calculate_check_digit(digits), 0);
        }

        #[test]
        fn sum_mod_11_eq_1_yields_sentinel_ten() {
            // `999 123 4560` — weighted sum 320, 320 % 11 == 1.
            let digits = [9, 9, 9, 1, 2, 3, 4, 5, 6, 0];
            assert_eq!(crate::calculate_check_digit(digits), 10);
        }

        #[test]
        fn sum_mod_11_in_2_to_10_yields_eleven_minus_remainder() {
            // For each remainder r in 2..=10, build a digit array whose
            // weighted sum has that remainder and verify the check digit
            // equals 11 - r.
            //
            // The weighted-sum formula is Σ d[i] × (10 − i) for i in 0..=8.
            // We sweep digit[8] from 0..=9 (weight 2) holding the rest at 0,
            // then verify each resulting check digit.
            for d8 in 0i8..=9 {
                let digits = [0, 0, 0, 0, 0, 0, 0, 0, d8, 0];
                let sum = d8 as usize * 2;
                let raw = 11 - (sum % 11);
                let expected = if raw == 11 {
                    0
                } else if raw == 10 {
                    10
                } else {
                    raw as i8
                };
                assert_eq!(
                    crate::calculate_check_digit(digits),
                    expected,
                    "for d[8]={d8}"
                );
            }
        }

        #[test]
        fn all_zeros_round_trips() {
            let zeros = NHSNumber::new([0; 10]);
            assert_eq!(zeros.to_string(), "000 000 0000");
            assert!(zeros.validate_check_digit());
        }

        #[test]
        fn all_nines_round_trips() {
            let nines = NHSNumber::new([9; 10]);
            assert_eq!(nines.to_string(), "999 999 9999");
            // sum = 9 × (10+9+...+2) = 9 × 54 = 486. 486 % 11 = 2. 11 - 2 = 9.
            assert_eq!(nines.calculate_check_digit(), 9);
            assert!(nines.validate_check_digit());
        }
    }

    /// Ordering and collection-use tests — back §9 of `spec.md`.
    mod ordering {
        use super::super::*;
        use std::collections::{BTreeMap, BTreeSet};

        #[test]
        fn ord_matches_numeric_intuition() {
            let lo = NHSNumber::new([0; 10]);
            let mid = NHSNumber::new([5; 10]);
            let hi = NHSNumber::new([9; 10]);
            assert!(lo < mid);
            assert!(mid < hi);
            assert!(lo < hi);
        }

        #[test]
        fn ord_breaks_ties_left_to_right() {
            // Only differ in position 5; lexicographic on the digit array.
            let a = NHSNumber::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
            let b = NHSNumber::new([1, 2, 3, 4, 5, 7, 7, 8, 9, 0]);
            assert!(a < b);
        }

        #[test]
        fn vec_sort_is_ascending() {
            let mut v = vec![
                NHSNumber::new([9; 10]),
                NHSNumber::new([0; 10]),
                NHSNumber::new([5; 10]),
            ];
            v.sort();
            assert_eq!(
                v,
                vec![
                    NHSNumber::new([0; 10]),
                    NHSNumber::new([5; 10]),
                    NHSNumber::new([9; 10]),
                ]
            );
        }

        #[test]
        fn btreeset_dedups_and_orders() {
            let mut set = BTreeSet::new();
            set.insert(NHSNumber::new([9; 10]));
            set.insert(NHSNumber::new([0; 10]));
            set.insert(NHSNumber::new([0; 10])); // dup
            assert_eq!(set.len(), 2);
            let first = set.iter().next().copied().unwrap();
            assert_eq!(first, NHSNumber::new([0; 10]));
        }

        #[test]
        fn btreemap_use_as_key() {
            let mut map: BTreeMap<NHSNumber, &'static str> = BTreeMap::new();
            map.insert(NHSNumber::new([0; 10]), "min");
            map.insert(NHSNumber::new([9; 10]), "max");
            assert_eq!(map.get(&NHSNumber::new([0; 10])), Some(&"min"));
            assert_eq!(map.get(&NHSNumber::new([9; 10])), Some(&"max"));
        }
    }

    /// Trait-impl smoke tests — assert the derived traits compile and
    /// behave as expected (Copy, Clone, Send, Sync, Serialize, Deserialize).
    mod traits {
        use super::super::*;

        fn assert_copy<T: Copy>() {}
        fn assert_clone<T: Clone>() {}
        fn assert_send_sync<T: Send + Sync>() {}
        fn assert_serde<T: serde::Serialize + serde::de::DeserializeOwned>() {}

        #[test]
        fn nhs_number_is_copy_clone_send_sync_serde() {
            assert_copy::<NHSNumber>();
            assert_clone::<NHSNumber>();
            assert_send_sync::<NHSNumber>();
            assert_serde::<NHSNumber>();
        }

        #[test]
        fn copy_does_not_move() {
            let a = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
            let b = a;
            // a is still usable after the copy — proves Copy semantics.
            assert_eq!(a, b);
        }

        #[test]
        fn clone_produces_equal_value() {
            let a = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
            #[allow(clippy::clone_on_copy)]
            let b = a.clone();
            assert_eq!(a, b);
        }

        #[test]
        fn debug_format_is_non_empty() {
            let n = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
            let dbg = format!("{n:?}");
            assert!(dbg.contains("NHSNumber"));
            assert!(dbg.contains("digits"));
        }
    }
}
