# Usage Guide

This guide walks through every public API in the `nhs-number` crate, with small
runnable snippets. For a working program version of each section, see the
matching file in [`../../examples/`](../../examples/).

## Table of contents

- [Installation](#installation)
- [Construction](#construction)
- [Parsing from a string](#parsing-from-a-string)
- [Formatting / display](#formatting--display)
- [Check digit and validation](#check-digit-and-validation)
- [Testable range](#testable-range)
- [Random samples](#random-samples)
- [Ordering and equality](#ordering-and-equality)
- [Serde support](#serde-support)
- [Error type](#error-type)

## Installation

Add the crate with `cargo add`:

```sh
cargo add nhs-number
```

Or add it to `Cargo.toml` by hand:

```toml
[dependencies]
nhs-number = "1"
```

## Construction

An `NHSNumber` wraps a fixed-length `[i8; 10]` digit array.

```rust
use nhs_number::NHSNumber;

// Via the `new` constructor.
let a = NHSNumber::new([9, 9, 9, 1, 2, 3, 4, 5, 6, 0]);

// Via struct literal — the `digits` field is public.
let b = NHSNumber { digits: [9, 9, 9, 1, 2, 3, 4, 5, 6, 0] };

assert_eq!(a, b);
```

## Parsing from a string

`NHSNumber` implements `FromStr`, so it works with `str::parse` and `FromStr::from_str`.
Two input formats are accepted:

1. **Ten digits, no separators:** `"9991234560"`.
2. **Canonical with single spaces:** `"999 123 4560"` — exactly the shape
   produced by `Display`.

Anything else — wrong length, non-digit characters, leading or trailing spaces,
doubled spaces, hyphens — returns `Err(ParseError)`.

```rust
use nhs_number::NHSNumber;
use std::str::FromStr;

let from_tight     = NHSNumber::from_str("9991234560").unwrap();
let from_canonical = NHSNumber::from_str("999 123 4560").unwrap();
assert_eq!(from_tight, from_canonical);

// `str::parse` works too thanks to `FromStr`.
let parsed: NHSNumber = "999 123 4560".parse().unwrap();
assert_eq!(parsed, from_canonical);
```

## Formatting / display

`NHSNumber` implements `Display` and `Into<String>`. Both produce the canonical
`"DDD DDD DDDD"` format (3 digits, space, 3 digits, space, 4 digits).

```rust
use nhs_number::NHSNumber;

let n = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
assert_eq!(n.to_string(), "012 345 6789");
let s: String = n.into();
assert_eq!(s, "012 345 6789");

// Or the free-function equivalent, skipping the wrapper:
let s2 = nhs_number::format([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
assert_eq!(s2, "012 345 6789");
```

## Check digit and validation

Every NHS Number's tenth digit is a modulo-11 checksum over the first nine.
See [checksum](../checksum/index.md) for the full algorithm.

```rust
use nhs_number::NHSNumber;

let n = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);

// The stored tenth digit.
assert_eq!(n.check_digit(), 3);

// The digit the algorithm *expects*, given digits 0..9.
assert_eq!(n.calculate_check_digit(), 3);

// True only when stored == calculated.
assert!(n.validate_check_digit());
```

Each method has a matching free function:

- `nhs_number::check_digit(digits)`
- `nhs_number::calculate_check_digit(digits)`
- `nhs_number::validate_check_digit(digits)`

### The "no digit fits" case

When the weighted sum of the first nine digits is congruent to `1 (mod 11)`,
the NHS specification says no digit in `0..=9` can stand in as a check
digit. `calculate_check_digit` returns the sentinel value `10` in that case,
and `validate_check_digit` returns `false` regardless of the stored tenth
digit:

```rust
use nhs_number::NHSNumber;

let bad = NHSNumber::new([9, 9, 9, 1, 2, 3, 4, 5, 6, 0]);
assert_eq!(bad.calculate_check_digit(), 10);   // sentinel: invalid
assert!(!bad.validate_check_digit());
```

## Testable range

The NHS reserves `999 000 0000` – `999 999 9999` as a range that is valid but
guaranteed never to be issued. Use it anywhere you need sample data.

```rust
use nhs_number::testable::{TESTABLE_MIN, TESTABLE_MAX, TESTABLE_RANGE_INCLUSIVE};

assert_eq!(TESTABLE_MIN.to_string(), "999 000 0000");
assert_eq!(TESTABLE_MAX.to_string(), "999 999 9999");
assert!(TESTABLE_RANGE_INCLUSIVE.contains(&*TESTABLE_MIN));
```

The bounds are `LazyLock<NHSNumber>`; dereference with `*` to get the
underlying value.

## Random samples

```rust
use nhs_number::NHSNumber;

// Both styles produce a number in the testable range.
let a = NHSNumber::testable_random_sample();
let b = nhs_number::testable_random_sample();

assert!(a >= *nhs_number::testable::TESTABLE_MIN);
assert!(b <= *nhs_number::testable::TESTABLE_MAX);
```

Random samples are **not guaranteed** to have a valid check digit. If your
test needs a valid number, resample until `validate_check_digit()` is true, or
compute and overwrite the tenth digit.

## Ordering and equality

`NHSNumber` derives `PartialEq`, `Eq`, `PartialOrd`, and `Ord`, which compares
lexicographically on the digit array — so ordering matches natural numeric
ordering.

```rust
use nhs_number::NHSNumber;

let a = NHSNumber::new([0; 10]);
let b = NHSNumber::new([9; 10]);
assert!(a < b);
```

This means `NHSNumber` can be used as a `BTreeMap`/`BTreeSet` key directly.

## Serde support

`NHSNumber` derives `Serialize` and `Deserialize`. The on-the-wire shape is the
struct's public layout, i.e. `{ "digits": [d0, d1, ..., d9] }`.

If you want string-form JSON instead (e.g. `"999 123 4560"`), wrap it in a
newtype and implement `Serialize`/`Deserialize` manually using the `Display`
and `FromStr` impls.

## Error type

Parse failures use a single zero-sized `ParseError`:

```rust
pub struct ParseError;
```

That intentional simplicity means callers differentiate only "parsed" from
"did not parse" — tighter error taxonomies (wrong length, wrong separator,
non-digit, etc.) belong in caller code.
