//! # Parsing
//!
//! NHS Numbers arrive as strings from almost every real-world source: CSV
//! columns, API payloads, user paste buffers, URLs. This example walks through
//! the parser in detail:
//!
//! 1. The two formats that are accepted.
//! 2. How to call the parser (three interchangeable styles).
//! 3. A tour of the inputs the parser rejects — and *why* it rejects each
//!    one — so that callers know what their upstream normalisation has to
//!    handle.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example parsing
//! ```

use nhs_number::NHSNumber;
// `ParseError` is the single unit struct returned for every parse failure.
// Match on it when you need to branch on "did we parse" vs "did we not".
use nhs_number::parse_error::ParseError;
// `FromStr` is the standard-library trait that powers both
// `NHSNumber::from_str(s)` and the `"…".parse::<NHSNumber>()` sugar.
use std::str::FromStr;

fn main() {
    // === Accepted format #1: ten contiguous digits ===
    //
    // When the input is exactly ten characters long and every character is a
    // decimal digit, the parser reads each character in turn and stores it.
    let tight: NHSNumber = NHSNumber::from_str("9991234560").unwrap();
    assert_eq!(tight.digits, [9, 9, 9, 1, 2, 3, 4, 5, 6, 0]);

    // === Accepted format #2: canonical "DDD DDD DDDD" with single spaces ===
    //
    // When the input is exactly twelve characters long, the parser requires a
    // space at position 3 (between the first and second group) and another at
    // position 7 (between the second and third group). Any other layout —
    // including the same spaces in different positions — is rejected.
    let canonical: NHSNumber = NHSNumber::from_str("999 123 4560").unwrap();
    assert_eq!(canonical.digits, [9, 9, 9, 1, 2, 3, 4, 5, 6, 0]);

    // Both formats describe the same number, and so compare equal.
    assert_eq!(tight, canonical);

    // === Call styles ===
    //
    // Three interchangeable ways to invoke the parser. Pick whichever reads
    // best at the call site — the result is identical.

    // Style A: explicit `FromStr::from_str`.
    let a: NHSNumber = NHSNumber::from_str("999 123 4560").unwrap();

    // Style B: the turbofish `parse()` sugar. Useful inside expression
    // positions because it composes well with `?`.
    let b: NHSNumber = "999 123 4560".parse::<NHSNumber>().unwrap();

    // Style C: `parse()` with a type annotation on the binding. Cleaner when
    // the target type is already named on the `let`.
    let c: NHSNumber = "999 123 4560".parse().unwrap();

    assert_eq!(a, b);
    assert_eq!(b, c);

    // === Round-trip: parse → display → parse ===
    //
    // A parse followed by a `Display` followed by another parse should always
    // recover the original value. This is the property you want whenever you
    // round-trip data through logs, databases, or wire formats.
    let displayed: String = canonical.to_string();
    let reparsed: NHSNumber = NHSNumber::from_str(&displayed).unwrap();
    assert_eq!(canonical, reparsed);

    // === Rejected inputs ===
    //
    // Each of the strings below is rejected with `Err(ParseError)`. The
    // comment on each explains *why* — use these as a reference when your
    // input source needs pre-parse normalisation.
    let invalid_inputs: [(&str, &str); 10] = [
        ("", "empty string has length 0, neither 10 nor 12"),
        ("12345", "five characters, below both accepted lengths"),
        (
            "01234567890",
            "eleven characters, between the accepted lengths",
        ),
        (
            "012-345-6789",
            "twelve characters but separators are hyphens, not spaces",
        ),
        (
            " 012 345 6789",
            "leading space — position 3 is a digit, not the required space",
        ),
        (
            "012 345 6789 ",
            "trailing space — position 3 is still a space but length is now 13",
        ),
        (
            "012  345  6789",
            "doubled spaces push the second group out of its expected positions",
        ),
        ("012 3456789", "one space only — the length becomes 11"),
        (
            "012345 6789",
            "one space only, in the wrong place — length 11",
        ),
        (
            "abc 123 4567",
            "letters in the first group fail `to_digit(10)` lookup",
        ),
    ];

    for (input, reason) in invalid_inputs {
        let result: Result<NHSNumber, ParseError> = NHSNumber::from_str(input);
        assert!(
            result.is_err(),
            "expected error for {:?} ({})",
            input,
            reason
        );
    }

    println!("ok");
}
