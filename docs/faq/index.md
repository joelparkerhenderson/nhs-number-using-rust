# Frequently Asked Questions

### Is this crate an official NHS product?

No. It is an open-source Rust crate published by Joel Parker Henderson. It
is licensed for use by anyone under a permissive multi-license (MIT OR
Apache-2.0 OR GPL-2.0 OR GPL-3.0 OR BSD-3-Clause).

### Can I use this crate in production NHS software?

The crate models the NHS Number format and validates its check digit according
to the public specification. Whether it is appropriate for a particular
deployment is a decision for the deploying organisation — review the source,
pin the version, and test against your own corpus before relying on it.

### Which jurisdictions does the NHS Number cover?

NHS England and NHS Isle of Man. Scotland uses a different identifier (CHI
numbers); Northern Ireland uses a separate sub-range of the same 10-digit
space. See [ranges](../ranges/index.md) for the precise numeric ranges.

### Does a valid check digit mean the number belongs to a real patient?

**No.** The check digit is a transcription-error catcher, not a lookup. Any
ten digits whose modulo-11 checksum works out are "valid" by the algorithm —
but only the NHS's own registers know which numbers have actually been issued
to patients. The testable range `999 000 0000` – `999 999 9999` is the one
range where you can be certain a valid-by-checksum number is _not_ a real
patient.

### Why are there both methods and free functions for the same operation?

Some callers already have a `[i8; 10]` and do not want to build an
`NHSNumber` just to call a method; others already have an `NHSNumber` and
prefer dot-call syntax. The two forms always return the same result:

```rust
let d = [9, 9, 9, 1, 0, 0, 0, 0, 0, 3];
let n = nhs_number::NHSNumber::new(d);

assert_eq!(n.calculate_check_digit(),
           nhs_number::calculate_check_digit(d));
```

### What does `FromStr` accept, and what does it reject?

Accepted:

- Ten contiguous digits: `"9991234560"`
- Canonical format with single spaces at positions 3 and 7:
  `"999 123 4560"`

Everything else is rejected with `ParseError`, including:

- Any other string length.
- Any non-digit character (including hyphens).
- Leading, trailing, or doubled spaces.
- Spaces in positions other than 3 and 7.

If your input source uses a different separator (e.g. hyphens or no spaces
from a legacy system), normalise it in caller code before parsing.

### Why does `testable_random_sample()` sometimes return a number whose check digit is wrong?

By design. The function returns a random draw from the full testable range,
which includes the ~90% of ten-digit sequences with invalid check digits.
That is often what you want — e.g. when stress-testing a validator.

If you need a random testable number that also has a valid check digit:

1. Loop `testable_random_sample()` until `validate_check_digit()` returns
   `true`; or
2. Pick the first nine digits yourself and compute the tenth with
   `calculate_check_digit` (see
   [`examples/generate_valid.rs`](../../examples/generate_valid.rs)).

### Can I serialise an `NHSNumber` as its human-readable string?

The default `Serialize` / `Deserialize` impls use the struct layout (a
`digits` field containing a ten-element array of `i8`). If you want the
human-readable `"DDD DDD DDDD"` form on the wire, wrap `NHSNumber` in a
newtype and implement the two traits manually using `Display` and `FromStr`.

### How does the check-digit algorithm work?

Multiply each of the first nine digits by decreasing weights (10, 9, …, 2),
sum, take modulo 11, subtract from 11; 11 maps to 0, 10 means the number is
invalid (the crate signals that case by returning the sentinel value `10`
from `calculate_check_digit`). See [checksum](../checksum/index.md) for the
fully worked examples.

### Where should I report a bug or suggest a change?

Open an issue at
<https://github.com/joelparkerhenderson/nhs-number-using-rust/issues>, or email the
maintainer at
[joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com).
