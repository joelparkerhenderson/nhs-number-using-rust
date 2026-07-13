[`nhs-number` specification](index.md) — section 2 of 19. Section numbers (§2.x) are stable and cited from code, tests, and commit messages.

# 2. Domain model

### 2.1 What an NHS Number is

A ten-digit identifier in the `3 3 4` format, where the tenth digit is a
modulo-11 checksum over the first nine. The displayed form is three groups
separated by single spaces, e.g. `943 476 5919`.

Each of the ten positions holds a single decimal digit (`0..=9`).

### 2.2 Authoritative references

- Wikipedia: [NHS Number](https://en.wikipedia.org/wiki/NHS_number).
- Wikipedia: [National Health Service](https://en.wikipedia.org/wiki/National_Health_Service).

Where this spec and Wikipedia disagree, this spec — backed by tests — is
what the crate implements; record the disagreement in §18.

### 2.3 Behavioural rule index

The spec's load-bearing rules are listed here for cross-reference. Each
rule has a stable ID (`R<n>`) so tests and commit messages can cite the
exact rule they enforce, change, or relax. The detailed definition lives
in the section linked on the right.

| Rule | Statement                                                            | Defined in |
| ---- | -------------------------------------------------------------------- | ---------- |
| R1   | An NHSNumber wraps exactly ten `i8` digits, each in `0..=9`.         | §3.1       |
| R2   | The `digits` field is `pub`; struct-literal construction is stable.  | §3.2       |
| R3   | Every method on `NHSNumber` has a free-function counterpart with the same return value on the same input. | §4.2 |
| R4   | `Display` produces exactly `"DDD DDD DDDD"` (12 chars, ASCII).       | §5.1       |
| R5   | `FromStr` accepts exactly two shapes: `"DDDDDDDDDD"` and `"DDD DDD DDDD"`. | §5.2 |
| R6   | `FromStr` only accepts ASCII digits 0–9.                             | §5.2       |
| R7   | The parser performs no normalisation.                                | §5.5       |
| R8   | `parse(display(n)) == n` for every `n: NHSNumber`.                   | §5.4       |
| R9   | The check digit is computed by the modulo-11 algorithm in §6.1.      | §6.1       |
| R10  | `calculate_check_digit` returns the sentinel `10` when `sum % 11 == 1`. | §6.1.1   |
| R11  | `validate_check_digit` returns `false` whenever the sentinel applies. | §6.1.1   |
| R12  | `NHSNumber` implements `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Serialize` (all derived) and `Deserialize` (hand-written; see R20). | §3.3 |
| R13  | `Ord` is lexicographic on `digits`, which coincides with numeric.    | §9         |
| R14  | `testable_random_sample()` always returns a value with `digits[0..3] == [9, 9, 9]`. | §8.1 |
| R15  | The check digit of a random sample is **not** computed; it is drawn at random. | §8.1 |
| R16  | The serialised JSON shape is `{ "digits": [..10] }`.                  | §11.1      |
| R17  | `ParseError` is a zero-sized unit struct with no payload.            | §4.5, §12  |
| R18  | `From<NHSNumber> for String` delegates to `Display`; the std-lib blanket impl therefore also gives `Into<String>`, and both produce the same value as `to_string()`. | §4.3, §5.1 |
| R19  | `NHSNumber` derives `Hash`, consistent with `Eq`.                     | §3.3, §9   |
| R20  | `Deserialize` keeps the `{ "digits": [..10] }` wire shape but **rejects** any digit outside `0..=9`; the error message echoes no payload data. | §11.1 |
| R21  | `ParseError` implements `Display` (fixed message, no input echo) and `std::error::Error`. | §4.5, §12 |
| R22  | `serde_string::NHSNumberString` serialises as the canonical string form and deserialises via `FromStr` (both accepted shapes). | §11.2 |
| R23  | `is_issuable_range` returns `true` iff the first nine digits lie in an issued range net of reservations (§7.5); the tenth digit is ignored; out-of-domain digits return `false`. | §7.5 |

Adding, tightening, or removing a rule is a versioning decision (§14);
record the new rule with the next available ID and never reuse a retired
one.
