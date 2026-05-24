//! # Validation
//!
//! Every NHS Number's tenth and final digit is a modulo-11 checksum
//! calculated from the first nine digits. This example shows how to:
//!
//! 1. Read the stored check digit.
//! 2. Recompute the expected check digit from the first nine digits.
//! 3. Ask the library to compare the two and give a single yes/no answer.
//!
//! The same three operations are available as both methods on `NHSNumber`
//! and as free functions on `[i8; 10]`. They are shown side by side so you
//! can see that the two forms are interchangeable.
//!
//! The full checksum algorithm — multiply, sum, modulo, subtract — is
//! documented in `docs/checksum/index.md`.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example validation
//! ```

use nhs_number::NHSNumber;
use std::str::FromStr;

fn main() {
    // === Start with a number known to be valid ===
    //
    // `999 100 0003` is in the testable range and validates by the modulo-11
    // algorithm. We parse it from its "DDD DDD DDDD" form so the code looks
    // like what you would write against a real-world input.
    let valid: NHSNumber = NHSNumber::from_str("999 100 0003").unwrap();

    // --- Method style on the `NHSNumber` value ---
    //
    // `check_digit` returns the tenth digit exactly as it is stored — no
    // arithmetic, just an array lookup.
    assert_eq!(valid.check_digit(), 3);

    // `calculate_check_digit` runs the modulo-11 algorithm over the first
    // nine digits and returns the single digit that *should* appear in the
    // tenth position.
    assert_eq!(valid.calculate_check_digit(), 3);

    // `validate_check_digit` is simply `check_digit() == calculate_check_digit()`
    // packaged as a `bool`. It's the one-call check you almost always want at
    // the API boundary.
    assert!(valid.validate_check_digit());

    // --- Free-function style on the raw `[i8; 10]` ---
    //
    // Identical results; pick this form when you already have a digit array
    // and do not need the wrapper struct.
    let digits: [i8; 10] = valid.digits;
    assert_eq!(nhs_number::check_digit(digits), 3);
    assert_eq!(nhs_number::calculate_check_digit(digits), 3);
    assert!(nhs_number::validate_check_digit(digits));

    // === Rejecting an invalid number ===
    //
    // If a transcription error flipped the final digit from 3 to 4, the
    // stored check digit (4) would no longer match the calculated one (3),
    // and `validate_check_digit` would return `false`.
    let invalid: NHSNumber = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 4]);
    assert!(!invalid.validate_check_digit());

    // Use `check_digit` and `calculate_check_digit` together when you want to
    // tell the user *what* went wrong, not just *that* it went wrong.
    println!(
        "invalid number {} — got {}, expected {}",
        invalid,
        invalid.check_digit(),
        invalid.calculate_check_digit()
    );

    // === The `sum % 11 == 1` case: no digit fits ===
    //
    // `999 123 4560` is the textbook example where the weighted sum of the
    // first nine digits is congruent to 1 modulo 11. Per the NHS specification
    // no digit in 0..=9 is a valid check digit; `calculate_check_digit`
    // returns the sentinel value `10`, and `validate_check_digit` returns
    // `false` no matter what the stored tenth digit is.
    let unfittable: NHSNumber = NHSNumber::new([9, 9, 9, 1, 2, 3, 4, 5, 6, 0]);
    assert_eq!(unfittable.calculate_check_digit(), 10);
    assert!(!unfittable.validate_check_digit());

    // === The canonical Wikipedia example: 943 476 5919 ===
    //
    // This is the number used in the official example derivation of the
    // check-digit algorithm. See `docs/checksum/index.md` for the full table.
    // It validates, which is our sanity check that this crate's algorithm
    // matches the reference.
    let wiki: NHSNumber = NHSNumber::from_str("943 476 5919").unwrap();
    assert_eq!(wiki.check_digit(), 9);
    assert_eq!(wiki.calculate_check_digit(), 9);
    assert!(wiki.validate_check_digit());

    println!("ok");
}
