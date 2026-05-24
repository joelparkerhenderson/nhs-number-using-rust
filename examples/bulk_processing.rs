//! # Bulk processing
//!
//! A realistic scenario: you have a list of candidate NHS Number strings
//! coming from some external source — a CSV column, an API payload, a text
//! area in a back-office tool — and you need to classify each one as:
//!
//! - **Valid** — parsed correctly *and* the stored check digit matches.
//! - **Invalid checksum** — parsed correctly but the check digit is wrong.
//! - **Unparseable** — the string does not match either accepted format.
//!
//! The three-way classification is more useful than a two-way "good/bad"
//! because the remediations differ: a checksum failure usually means a
//! transcription error that a human can spot, while an unparseable string
//! usually needs a format normalisation step earlier in the pipeline.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example bulk_processing
//! ```

use nhs_number::NHSNumber;
use std::str::FromStr;

/// One of three disjoint outcomes for each candidate input.
///
/// Using a dedicated `enum` (rather than a `Result<Option<_>, _>` or similar)
/// makes the caller's `match` arms read like prose. It also leaves room to
/// carry the parsed number forward for the two "we managed to parse" cases,
/// so the caller can report specific details without re-parsing.
#[derive(Debug)]
enum Outcome {
    /// Parsed and passed the check-digit check.
    Valid(NHSNumber),
    /// Parsed but the stored check digit differs from the calculated one.
    InvalidChecksum(NHSNumber),
    /// Did not match either accepted input format.
    Unparseable,
}

/// Classify a single candidate string.
///
/// This function concentrates the "is it an NHS Number?" decision in one
/// place — every caller that wants the three-way answer goes through here.
fn classify(input: &str) -> Outcome {
    match NHSNumber::from_str(input) {
        // Parsed *and* checksum agrees — the happy path.
        Ok(n) if n.validate_check_digit() => Outcome::Valid(n),
        // Parsed but checksum disagrees. Keep the parsed value so the caller
        // can show the stored vs. expected check digit side by side.
        Ok(n) => Outcome::InvalidChecksum(n),
        // Parser rejected the string outright. There is nothing useful to
        // carry forward — the bytes never formed an `NHSNumber`.
        Err(_) => Outcome::Unparseable,
    }
}

fn main() {
    // A grab-bag of realistic inputs covering every branch of `classify`.
    //
    // Hard-coding the expected outcome next to each input keeps this
    // self-checking: if the parser's behaviour ever changes, the counts at
    // the bottom will no longer match and the `assert_eq!` calls will fire.
    let inputs: [&str; 6] = [
        "999 100 0003", // valid — testable range, canonical format
        "9991000003",   // valid — same number, tight format (no spaces)
        "943 476 5919", // valid — the Wikipedia reference number
        "999 100 0004", // invalid checksum — last digit flipped from 3 to 4
        "012-345-6789", // unparseable — hyphen separators not supported
        "hello world!", // unparseable — not even close to digits
    ];

    // Accumulate counts per branch so we can print a summary and assert on
    // the totals at the end.
    let mut valid_count: usize = 0;
    let mut bad_checksum_count: usize = 0;
    let mut unparseable_count: usize = 0;

    // Iterate, classify, and print a one-line status per input. Note the
    // `{:>14}` format specifier — it right-pads each input to 14 columns so
    // the arrows line up in the output, which is easier to read at a glance.
    for input in inputs {
        match classify(input) {
            Outcome::Valid(n) => {
                println!("{:>14} → VALID            → {}", input, n);
                valid_count += 1;
            }
            Outcome::InvalidChecksum(n) => {
                // Report both digits so a human reviewer can see the
                // likely typo without reopening the source record.
                println!(
                    "{:>14} → INVALID CHECKSUM → got {} expected {}",
                    input,
                    n.check_digit(),
                    n.calculate_check_digit()
                );
                bad_checksum_count += 1;
            }
            Outcome::Unparseable => {
                println!("{:>14} → UNPARSEABLE", input);
                unparseable_count += 1;
            }
        }
    }

    println!();
    println!("summary:");
    println!("  valid             : {valid_count}");
    println!("  invalid checksum  : {bad_checksum_count}");
    println!("  unparseable       : {unparseable_count}");

    // Self-check: if any of these fail, the example itself has drifted from
    // the behaviour documented above.
    assert_eq!(valid_count, 3);
    assert_eq!(bad_checksum_count, 1);
    assert_eq!(unparseable_count, 2);
}
