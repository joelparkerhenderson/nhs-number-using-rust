//! # Generating a valid testable NHS Number
//!
//! `NHSNumber::testable_random_sample()` gives a random value in the testable
//! range, but it draws the tenth digit randomly too — so only about 1 in 10
//! random draws has a correct check digit. When a test needs a number that
//! is *both* in the testable range *and* passes `validate_check_digit`, it's
//! usually cheaper (and deterministic) to:
//!
//! 1. Pick the first nine digits yourself (all starting with 9, 9, 9 so the
//!    number lands in the testable range).
//! 2. Compute the tenth digit with `calculate_check_digit`.
//! 3. Combine the two into a new `NHSNumber`.
//!
//! This example wraps that recipe in a tiny helper and produces five valid
//! testable numbers from fixed seeds, plus one from a random seed.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example generate_valid
//! ```

use nhs_number::NHSNumber;

/// Build a valid `NHSNumber` from the first nine digits.
///
/// The tenth digit is computed from the first nine via the modulo-11
/// algorithm (`calculate_check_digit`).
///
/// Caller obligations:
///
/// - Every digit must be in `0..=9` — no bounds checking is done here.
/// - The first nine digits must yield `weighted_sum % 11 != 1`. The strict
///   NHS specification has no valid check digit when `sum % 11 == 1`, and
///   `calculate_check_digit` returns the sentinel `10` in that case. This
///   helper does not detect the sentinel; pass such seeds and you will get
///   a malformed number rather than a panic.
fn with_computed_check_digit(first_nine: [i8; 9]) -> NHSNumber {
    // Start with a ten-element array and copy the supplied nine digits into
    // the first nine slots. The tenth slot stays at its default of 0 for
    // now — it gets overwritten below.
    let mut digits: [i8; 10] = [0_i8; 10];
    digits[..9].copy_from_slice(&first_nine);

    // Build a temporary `NHSNumber` just so we can call the check-digit
    // calculator on it. The tenth digit of this probe is whatever we
    // left it at (0), but `calculate_check_digit` only reads digits 0..=8,
    // so that does not matter.
    let probe: NHSNumber = NHSNumber::new(digits);

    // Fill in the tenth digit with the computed check digit and wrap up.
    digits[9] = probe.calculate_check_digit();
    NHSNumber::new(digits)
}

fn main() {
    // === 1. Five deterministic seeds ===
    //
    // Every seed starts with 9, 9, 9 so the resulting number lands in the
    // testable range. The remaining six digits are whatever we want; they
    // only have to be in 0..=9.
    // Each seed below is chosen so that `sum % 11 != 1` — otherwise no digit
    // in 0..=9 could stand in as a check digit (the strict NHS spec calls
    // that case invalid; `calculate_check_digit` returns the sentinel `10`).
    let seeds: [[i8; 9]; 5] = [
        [9, 9, 9, 0, 0, 0, 0, 0, 1],
        [9, 9, 9, 1, 0, 0, 0, 0, 0],
        [9, 9, 9, 5, 0, 0, 0, 0, 0],
        [9, 9, 9, 7, 5, 3, 1, 0, 2],
        [9, 9, 9, 9, 9, 9, 9, 9, 8],
    ];

    for seed in seeds {
        let n: NHSNumber = with_computed_check_digit(seed);
        // The whole point of this recipe: the result is always valid.
        assert!(n.validate_check_digit());
        println!("valid testable: {}", n);
    }

    // === 2. A random-but-valid testable number ===
    //
    // An alternative recipe for a *random* valid number: keep drawing
    // samples until one passes the check. On average this runs the body
    // about ten times, which is negligible for test fixtures. `from_fn` +
    // `find` is an iterator-based way to express "loop until the predicate
    // holds"; a plain `loop { … if … { break … } }` is equally fine.
    let random_valid: NHSNumber = std::iter::from_fn(|| Some(NHSNumber::testable_random_sample()))
        .find(|n| n.validate_check_digit())
        .unwrap();
    println!("random valid testable: {}", random_valid);
    assert!(random_valid.validate_check_digit());
}
