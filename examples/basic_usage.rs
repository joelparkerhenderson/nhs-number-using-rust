//! # Basic usage
//!
//! This example introduces the core `NHSNumber` type and the three common
//! ways to work with it:
//!
//! 1. Constructing a value from a ten-digit array.
//! 2. Formatting it as a human-readable string.
//! 3. Dropping the wrapper and using the equivalent free function.
//!
//! The NHS Number used below — `999 100 0003` — is drawn from the "testable"
//! range `999 000 0000` – `999 999 9999`. That whole range is guaranteed
//! never to be issued to a real patient, so its numbers are safe to hard-code
//! in source files and tests. (Roughly nine in ten numbers in the range fail
//! the modulo-11 check; `999 100 0003` is one of the ones that passes.)
//!
//! Run with:
//!
//! ```sh
//! cargo run --example basic_usage
//! ```

// `NHSNumber` is the only type we need from the crate for this example.
// The crate also re-exports its `testable` module contents at the top level
// (via `pub use testable::*;` in `src/lib.rs`), which is why helpers like
// `nhs_number::testable_random_sample` work without an explicit path.
use nhs_number::NHSNumber;

fn main() {
    // === 1. Construct from a `[i8; 10]` digit array ===
    //
    // Each digit is a single decimal digit in the range 0..=9, stored as a
    // signed 8-bit integer. The array is exactly ten elements — no more, no
    // less — so the shape is enforced at compile time.
    //
    // Digits are stored most-significant first, i.e. digits[0] is the
    // leftmost digit of the displayed number.
    let digits: [i8; 10] = [9, 9, 9, 1, 0, 0, 0, 0, 0, 3];
    let nhs_number: NHSNumber = NHSNumber::new(digits);

    // === 2. Construct via a struct literal ===
    //
    // The `digits` field on `NHSNumber` is `pub`, so callers can also build a
    // value directly with struct-literal syntax. This is equivalent to
    // `NHSNumber::new(...)`.
    let same_number: NHSNumber = NHSNumber { digits };
    assert_eq!(nhs_number, same_number);

    // === 3. Convert to a human-readable string ===
    //
    // `NHSNumber` implements `std::fmt::Display`, so `to_string()` — plus
    // anything else that accepts `Display`, such as `println!`, `format!`,
    // and `write!` — produces the canonical "DDD DDD DDDD" layout:
    //
    //   three digits, space, three digits, space, four digits
    let formatted: String = nhs_number.to_string();
    println!("formatted: {}", formatted);
    assert_eq!(formatted, "999 100 0003");

    // The `Into<String>` impl is just a thin wrapper around `to_string()`,
    // so the two produce identical output. Use whichever reads best in
    // context — `to_string()` is usually the most familiar.
    let as_string: String = nhs_number.into();
    assert_eq!(as_string, "999 100 0003");

    // === 4. Ordering works out of the box ===
    //
    // `NHSNumber` derives `Ord`, which compares lexicographically across the
    // digit array. Because all numbers have the same length and digits are
    // stored most-significant first, lexicographic ordering coincides with
    // the natural numeric ordering. That means a `Vec<NHSNumber>` sorted with
    // `.sort()` lines up the values exactly as you'd read them numerically.
    let smallest: NHSNumber = NHSNumber::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let largest: NHSNumber = NHSNumber::new([9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
    assert!(smallest < largest);

    // === 5. Free-function alternative ===
    //
    // The crate also exposes free functions that operate directly on
    // `[i8; 10]`. They are useful when you already have a digit array and do
    // not want to build the wrapper struct. The two forms always agree.
    let free_formatted: String = nhs_number::format(digits);
    assert_eq!(free_formatted, formatted);

    println!("ok");
}
