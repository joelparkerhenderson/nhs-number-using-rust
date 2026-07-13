//! The reserved **testable** NHS Number range and a random sampler.
//!
//! The NHS reserves `999 000 0000` – `999 999 9999` as a range that is
//! syntactically valid (it can pass the modulo-11 check digit) but is
//! guaranteed **never** to be issued to a real patient. Use it for unit
//! tests, doc-tests, fixtures, demos, and any other situation where a
//! real number would be inappropriate.
//!
//! This module exposes three lazy statics describing the range and a
//! random sampler:
//!
//! - [`TESTABLE_MIN`] — `999 000 0000`
//! - [`TESTABLE_MAX`] — `999 999 9999`
//! - [`TESTABLE_RANGE_INCLUSIVE`] — an inclusive [`RangeInclusive`] over
//!   the above bounds; useful with [`RangeInclusive::contains`].
//! - [`testable_random_sample`] — returns a random [`NHSNumber`] whose
//!   first three digits are `9, 9, 9` and whose remaining seven digits
//!   are drawn uniformly from `0..=9`.
//!
//! The crate re-exports this module's contents at the crate root via
//! `pub use testable::*;`, so callers can write
//! `nhs_number::testable_random_sample()` and `*nhs_number::TESTABLE_MIN`
//! without spelling out the module path.
//!
//! See [`spec/index.md`](https://github.com/joelparkerhenderson/nhs-number-using-rust/blob/main/spec/index.md)
//! §7.3 and §8 for the contract.
//!
//! Example — confirm a constructed number lies in the testable range:
//!
//! ```rust
//! use nhs_number::{NHSNumber, testable::*};
//!
//! let n = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);
//! assert!(TESTABLE_RANGE_INCLUSIVE.contains(&n));
//! ```

use crate::NHSNumber;
use rand::RngExt;
use std::ops::RangeInclusive;
use std::sync::LazyLock;

/// Inclusive lower bound of the testable NHS Number range — `999 000 0000`.
///
/// Backed by [`LazyLock`], so dereference with `*` to read the value:
///
/// ```rust
/// use nhs_number::testable::TESTABLE_MIN;
///
/// assert_eq!(TESTABLE_MIN.to_string(), "999 000 0000");
/// assert_eq!((*TESTABLE_MIN).digits, [9, 9, 9, 0, 0, 0, 0, 0, 0, 0]);
/// ```
///
/// Use it as a guard rail when accepting numbers from a "testable" path:
///
/// ```rust
/// use nhs_number::{NHSNumber, testable::TESTABLE_MIN};
///
/// let n = NHSNumber::new([9, 9, 9, 0, 1, 2, 3, 4, 5, 6]);
/// assert!(n >= *TESTABLE_MIN);
/// ```
///
#[allow(dead_code)]
pub static TESTABLE_MIN: LazyLock<NHSNumber> = LazyLock::new(|| NHSNumber {
    digits: [9, 9, 9, 0, 0, 0, 0, 0, 0, 0],
});

/// Inclusive upper bound of the testable NHS Number range — `999 999 9999`.
///
/// Backed by [`LazyLock`], so dereference with `*` to read the value:
///
/// ```rust
/// use nhs_number::testable::TESTABLE_MAX;
///
/// assert_eq!(TESTABLE_MAX.to_string(), "999 999 9999");
/// assert_eq!((*TESTABLE_MAX).digits, [9; 10]);
/// ```
///
/// Use it together with [`TESTABLE_MIN`] as a half of a manual range
/// check:
///
/// ```rust
/// use nhs_number::{NHSNumber, testable::TESTABLE_MAX};
///
/// let n = NHSNumber::new([9, 9, 9, 0, 1, 2, 3, 4, 5, 6]);
/// assert!(n <= *TESTABLE_MAX);
/// ```
///
#[allow(dead_code)]
pub static TESTABLE_MAX: LazyLock<NHSNumber> = LazyLock::new(|| NHSNumber {
    digits: [9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
});

/// The inclusive testable range, `[TESTABLE_MIN, TESTABLE_MAX]`.
///
/// Backed by [`LazyLock`]; the inner value supports
/// [`RangeInclusive::contains`]:
///
/// ```rust
/// use nhs_number::{NHSNumber, testable::TESTABLE_RANGE_INCLUSIVE};
///
/// let in_range = NHSNumber::new([9, 9, 9, 0, 1, 2, 3, 4, 5, 6]);
/// assert!(TESTABLE_RANGE_INCLUSIVE.contains(&in_range));
///
/// let out_of_range = NHSNumber::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// assert!(!TESTABLE_RANGE_INCLUSIVE.contains(&out_of_range));
/// ```
///
#[allow(dead_code)]
pub static TESTABLE_RANGE_INCLUSIVE: LazyLock<RangeInclusive<NHSNumber>> =
    LazyLock::new(|| RangeInclusive::new(*TESTABLE_MIN, *TESTABLE_MAX));

/// Return a random [`NHSNumber`] in the testable range.
///
/// Every returned value has `digits[0..3] == [9, 9, 9]`; the remaining
/// seven digits are drawn uniformly from `0..=9` using [`rand::rng`].
///
/// **The tenth digit is drawn randomly, not computed** — so roughly nine
/// in ten samples have an invalid modulo-11 check digit. This is
/// intentional; it lets tests exercise the rejection branch of
/// [`validate_check_digit`](crate::validate_check_digit). For a valid
/// random sample, either loop until `validate_check_digit()` returns
/// `true`, or pick the first nine digits yourself and compute the tenth.
///
/// Example — the sample is always in the testable range:
///
/// ```rust
/// use nhs_number::{NHSNumber, testable::*};
///
/// let n = testable_random_sample();
/// assert!(n >= *TESTABLE_MIN);
/// assert!(n <= *TESTABLE_MAX);
/// assert_eq!(&n.digits[0..3], &[9, 9, 9]);
/// ```
///
/// Example — generate a valid-by-checksum random testable number:
///
/// ```rust
/// use nhs_number::NHSNumber;
///
/// let n = std::iter::from_fn(|| Some(NHSNumber::testable_random_sample()))
///     .find(|n| n.validate_check_digit())
///     .unwrap();
/// assert!(n.validate_check_digit());
/// ```
///
#[allow(dead_code)]
pub fn testable_random_sample() -> NHSNumber {
    let mut rng = rand::rng();
    NHSNumber {
        digits: [
            9,
            9,
            9,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_exact_value() {
        assert_eq!(TESTABLE_MIN.digits, [9, 9, 9, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(TESTABLE_MIN.to_string(), "999 000 0000");
    }

    #[test]
    fn test_max_exact_value() {
        assert_eq!(TESTABLE_MAX.digits, [9; 10]);
        assert_eq!(TESTABLE_MAX.to_string(), "999 999 9999");
    }

    #[test]
    fn test_min_strictly_less_than_max() {
        assert!(*TESTABLE_MIN < *TESTABLE_MAX);
    }

    #[test]
    fn test_range_inclusive_contains_endpoints() {
        assert!(TESTABLE_RANGE_INCLUSIVE.contains(&*TESTABLE_MIN));
        assert!(TESTABLE_RANGE_INCLUSIVE.contains(&*TESTABLE_MAX));
    }

    #[test]
    fn test_range_inclusive_contains_interior() {
        let interior = NHSNumber::new([9, 9, 9, 1, 2, 3, 4, 5, 6, 7]);
        assert!(TESTABLE_RANGE_INCLUSIVE.contains(&interior));
    }

    #[test]
    fn test_range_inclusive_excludes_below() {
        let below = NHSNumber::new([9, 9, 8, 9, 9, 9, 9, 9, 9, 9]);
        assert!(!TESTABLE_RANGE_INCLUSIVE.contains(&below));
    }

    #[test]
    fn test_range_inclusive_excludes_low_numbers() {
        let zeros = NHSNumber::new([0; 10]);
        assert!(!TESTABLE_RANGE_INCLUSIVE.contains(&zeros));
    }

    #[test]
    fn test_range_inclusive_excludes_issued_ranges() {
        // Any number in §7.1 issued ranges is below TESTABLE_MIN.
        let issued = NHSNumber::new([6, 0, 0, 0, 0, 0, 0, 0, 0, 0]); // 600...
        assert!(!TESTABLE_RANGE_INCLUSIVE.contains(&issued));
    }

    #[test]
    fn test_random_sample_in_range() {
        for _ in 0..64 {
            let n = testable_random_sample();
            assert!(n >= *TESTABLE_MIN);
            assert!(n <= *TESTABLE_MAX);
            assert!(TESTABLE_RANGE_INCLUSIVE.contains(&n));
        }
    }

    #[test]
    fn test_random_sample_first_three_digits_are_999() {
        for _ in 0..64 {
            let n = testable_random_sample();
            assert_eq!(&n.digits[0..3], &[9, 9, 9]);
        }
    }

    #[test]
    fn test_random_sample_all_remaining_digits_in_range() {
        for _ in 0..64 {
            let n = testable_random_sample();
            for d in &n.digits[3..] {
                assert!((0..=9).contains(d), "digit out of range: {d}");
            }
        }
    }

    #[test]
    fn test_random_sample_is_non_deterministic() {
        // Two calls almost certainly differ. The probability of collision
        // on seven uniform `0..=9` digits is 10^-7 ≈ negligible — but to
        // avoid any chance of flake, try a small batch.
        let samples: Vec<_> = (0..16).map(|_| testable_random_sample()).collect();
        let distinct: std::collections::BTreeSet<_> = samples.iter().copied().collect();
        assert!(distinct.len() > 1, "samples were unexpectedly identical");
    }
}
