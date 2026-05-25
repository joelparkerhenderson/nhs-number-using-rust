# spec.md — `nhs-number` crate specification

**Status:** living document. Updated alongside every behavioural change.
**Audience:** maintainers, AI agents, downstream integrators reading the
crate's contract.
**Companion docs:** [`AGENTS.md`](AGENTS.md) for agent guidance,
[`index.md`](index.md) for the user-facing README,
[`docs/api/index.md`](docs/api/index.md) for the rendered API surface.

This file is the **canonical specification** that drives spec-driven
development (see [`AGENTS/spec-driven-development.md`](AGENTS/spec-driven-development.md)).
When the spec and the code disagree, the spec is the source of truth and
the code is a bug — or the spec needs updating *before* the code changes.

The discipline:

1. **Behaviour is described here first.** A PR that changes observable
   behaviour without touching this file is incomplete.
2. **Every behavioural rule is testable.** Sections that state a rule
   point to the test that enforces it, either via a `[Rule …]` tag (see
   §2.3) or via the §13.1 coverage table.
3. **Plans and tasks live here too.** §16 holds the **roadmap** in
   priority order; §17 holds the **backlog of open tasks** with stable
   `T<n>` IDs; §18 holds **open questions and known divergences**. There
   is no separate `plan.md` or `tasks.md`.

---

## Table of contents

1.  [Purpose and scope](#1-purpose-and-scope)
2.  [Domain model](#2-domain-model)
3.  [Data model](#3-data-model)
4.  [Public API surface](#4-public-api-surface)
5.  [String forms (parsing and formatting)](#5-string-forms-parsing-and-formatting)
6.  [Check-digit algorithm](#6-check-digit-algorithm)
7.  [Number ranges](#7-number-ranges)
8.  [Random sampling](#8-random-sampling)
9.  [Ordering, equality, and collection use](#9-ordering-equality-and-collection-use)
10. [Patient-safety framing](#10-patient-safety-framing)
11. [Serialisation](#11-serialisation)
12. [Error handling](#12-error-handling)
13. [Testing strategy](#13-testing-strategy)
14. [Compatibility and versioning](#14-compatibility-and-versioning)
15. [Dependencies and build](#15-dependencies-and-build)
16. [Roadmap](#16-roadmap)
17. [Open tasks (backlog)](#17-open-tasks-backlog)
18. [Open questions and known divergences](#18-open-questions-and-known-divergences)
19. [Glossary](#19-glossary)

---

## 1. Purpose and scope

### 1.1 Purpose

Provide a small, dependable Rust value type — `NHSNumber` — that models a
National Health Service (NHS) Number across **NHS England** and
**NHS Isle of Man**, with first-class support for:

- parsing the two canonical string forms,
- formatting back to a canonical string form,
- reading and recomputing the modulo-11 check digit,
- expressing the reserved testable range as types and constants,
- ordering, equality, and `serde` serialisation.

### 1.2 In scope

- The structure, parsing, formatting, ordering, and check-digit validation
  of a ten-digit NHS Number.
- A reserved testable range (`999 000 0000` – `999 999 9999`) exposed as
  typed constants and a random sampler.
- A documented sentinel for the "no digit fits" case (raw check value
  equals 10) so the API stays infallible at the type level while still
  catching the case at runtime.

### 1.3 Out of scope (non-goals)

- Looking up whether a given NHS Number has actually been issued to a
  patient (no registry access).
- Mapping NHS Numbers to patient identities, demographics, or care records.
- Validating non-NHS-England/IoM identifiers (e.g. Scottish CHI, Northern
  Irish HCN).
- Cryptographic protection of NHS Numbers in transit or at rest.
- Localisation of the rendered string form (it is fixed: `"DDD DDD DDDD"`).
- Range membership checks (issued/reserved/testable) beyond what callers
  can build from the public constants in §7.

---

## 2. Domain model

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
| R12  | `NHSNumber` derives `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Serialize`, `Deserialize`. | §3.3 |
| R13  | `Ord` is lexicographic on `digits`, which coincides with numeric.    | §9         |
| R14  | `testable_random_sample()` always returns a value with `digits[0..3] == [9, 9, 9]`. | §8.1 |
| R15  | The check digit of a random sample is **not** computed; it is drawn at random. | §8.1 |
| R16  | The serialised JSON shape is `{ "digits": [..10] }`.                  | §11.1      |
| R17  | `ParseError` is a zero-sized unit struct with no payload.            | §4.5, §12  |
| R18  | `From<NHSNumber> for String` delegates to `Display`; the std-lib blanket impl therefore also gives `Into<String>`, and both produce the same value as `to_string()`. | §4.3, §5.1 |

Adding, tightening, or removing a rule is a versioning decision (§14);
record the new rule with the next available ID and never reuse a retired
one.

---

## 3. Data model

### 3.1 The `NHSNumber` struct [R1]

```rust
pub struct NHSNumber {
    pub digits: [i8; 10],
}
```

**Invariants:**

- `digits` is always exactly ten elements (enforced by the type system).
- Each element **should** be in `0..=9`. Out-of-range values are not
  rejected at construction, but the check-digit algorithm is only defined
  on this domain (§6.4). Callers handling untrusted input must parse via
  [`FromStr`](#5-string-forms-parsing-and-formatting), which enforces the
  digit range.
- `digits[0]` is the most-significant digit (leftmost in the displayed
  form).

### 3.2 Public field [R2]

The `digits` field is `pub`. Callers may construct an `NHSNumber` directly
via a struct literal:

```rust
let n = NHSNumber { digits: [9, 9, 9, 1, 2, 3, 4, 5, 6, 0] };
```

This is documented and stable; do not remove `pub` from the field.

### 3.3 Derived traits [R12]

| Trait                      | Semantics                                                   |
| -------------------------- | ----------------------------------------------------------- |
| `Debug`                    | Standard derive.                                            |
| `Clone`, `Copy`            | Cheap — the struct is 10 bytes.                             |
| `PartialEq`, `Eq`          | Digit-by-digit equality.                                    |
| `PartialOrd`, `Ord`        | Lexicographic on the digit array (matches numeric).         |
| `Serialize`, `Deserialize` | Serde with the default struct layout (`{ "digits": […] }`). |

`Hash` is intentionally **not** derived (see §17.T1 and §18.1). Adding it
is a minor-version change and must come with a doc-test exercising a
`HashMap`-keyed use case.

---

## 4. Public API surface

The full surface is fixed by this section. Adding to it is a minor-version
bump; changing or removing anything is a major-version bump (§14).

### 4.1 Methods on `NHSNumber`

| Signature                                          | Purpose                                            |
| -------------------------------------------------- | -------------------------------------------------- |
| `NHSNumber::new(digits: [i8; 10]) -> NHSNumber`    | Construct from a ten-digit array.                  |
| `NHSNumber::check_digit(&self) -> i8`              | Return the tenth digit as stored.                  |
| `NHSNumber::calculate_check_digit(&self) -> i8`    | Compute the tenth digit from digits 0..9 (see §6). |
| `NHSNumber::validate_check_digit(&self) -> bool`   | `check_digit() == calculate_check_digit()`.        |
| `NHSNumber::testable_random_sample() -> NHSNumber` | Random value in the testable range (see §8).       |

### 4.2 Free functions on `[i8; 10]` [R3]

| Signature                                            | Equivalent to                             |
| ---------------------------------------------------- | ----------------------------------------- |
| `fn format(digits: [i8; 10]) -> String`              | `NHSNumber::to_string()` / `Into<String>` |
| `fn check_digit(digits: [i8; 10]) -> i8`             | `NHSNumber::check_digit`                  |
| `fn calculate_check_digit(digits: [i8; 10]) -> i8`   | `NHSNumber::calculate_check_digit`        |
| `fn validate_check_digit(digits: [i8; 10]) -> bool`  | `NHSNumber::validate_check_digit`         |
| `fn testable::testable_random_sample() -> NHSNumber` | `NHSNumber::testable_random_sample`       |

Each free function and its corresponding method **must** return the same
value on the same input. This is an enforced invariant via tests (see
`src/lib.rs::tests::properties`).

### 4.3 Trait implementations

| Impl                          | Behaviour                                                                                              | Rules     |
| ----------------------------- | ------------------------------------------------------------------------------------------------------ | --------- |
| `Display`                     | Format as `"DDD DDD DDDD"` (see §5).                                                                   | R4        |
| `From<NHSNumber> for String`  | Delegates to `to_string()`. Provides `Into<String>` via the standard-library blanket impl.             | R18       |
| `FromStr`, `Err = ParseError` | Parse `"DDDDDDDDDD"` or `"DDD DDD DDDD"` (§5).                                                         | R5, R6, R17 |

### 4.4 The `testable` module

```rust
pub static TESTABLE_MIN: LazyLock<NHSNumber>;            // 999 000 0000
pub static TESTABLE_MAX: LazyLock<NHSNumber>;            // 999 999 9999
pub static TESTABLE_RANGE_INCLUSIVE: LazyLock<RangeInclusive<NHSNumber>>;
pub fn testable_random_sample() -> NHSNumber;
```

Re-exported at the crate root via `pub use testable::*;`, so callers may
write `nhs_number::testable_random_sample()` and `*nhs_number::TESTABLE_MIN`.

### 4.5 The `ParseError` type [R17]

```rust
pub struct ParseError;
```

| Property         | Guarantee                                                       |
| ---------------- | --------------------------------------------------------------- |
| Layout           | Zero-sized unit struct (`size_of::<ParseError>() == 0`).        |
| Derives          | `Debug`, `PartialEq`, `Eq`.                                     |
| Returned by      | `<NHSNumber as FromStr>::from_str` (the only source).           |
| Payload          | None — by design. See §12.                                      |

The type intentionally carries no detail; callers who need richer error
reporting wrap or map it at the parse site (see §12 for the recommended
pattern).

---

## 5. String forms (parsing and formatting)

### 5.1 Canonical display form [R4, R18]

`Display` always produces **exactly** twelve characters:

```
DDD DDD DDDD
```

- three digits, single space, three digits, single space, four digits;
- no leading, trailing, or doubled spaces;
- no alternative separators (hyphen, period, slash, NBSP);
- output is pure ASCII.

`From<NHSNumber> for String` and the blanket-provided `Into<String>`
delegate to `Display`, so they produce the same twelve-character string.

### 5.2 Accepted input forms [R5, R6]

`FromStr::from_str` accepts **exactly two** shapes:

1. **Ten contiguous digits**, no separators: `"DDDDDDDDDD"` (length 10).
2. **Canonical with single spaces**: `"DDD DDD DDDD"` (length 12, space at
   position 3 and position 7 only).

Both must be **ASCII** digits 0–9 in every digit position. Non-ASCII
digit characters that render as digits (Arabic-Indic `٠..٩`, full-width
`０..９`, Devanagari `०..९`, etc.) are rejected because
`char::to_digit(10)` only accepts ASCII `0..=9`.

### 5.3 Rejected input forms

Everything else returns `Err(ParseError)`. Notable examples — each is
covered by a dedicated test in `src/from_str.rs::tests`:

| Input              | Reason                                                         |
| ------------------ | -------------------------------------------------------------- |
| `""`               | Length 0 — neither accepted length.                            |
| `"12345"`          | Length 5.                                                      |
| `"01234567890"`    | Length 11.                                                     |
| `"0123 4567 8901"` | Length 14.                                                     |
| `"012-345-6789"`   | Length 12 but separators are hyphens.                          |
| `" 012 345 6789"`  | Leading space — position 3 is a digit, not the required space. |
| `"012 345 6789 "`  | Trailing space — length becomes 13.                            |
| `"012  345  6789"` | Doubled spaces shift the second group out of position.         |
| `"012 3456789"`    | One space only — length 11.                                    |
| `"012345 6789"`    | One space only, wrong place — length 11.                       |
| `"abc 123 4567"`   | Non-digit characters.                                          |
| `"012 345 abcd"`   | Non-digit characters in the last group.                        |
| `"012\u{00A0}345\u{00A0}6789"` | NBSP separators, not ASCII space.                  |
| `"012\t345\t6789"` | Tab separators, not ASCII space.                               |
| `"٠١٢٣٤٥٦٧٨٩"`     | Arabic-Indic digits; not ASCII.                                |
| `"-123456789"`     | Sign is not a digit.                                           |
| `"+123456789"`     | Sign is not a digit.                                           |
| `"12345.6789"`     | Decimal point.                                                 |
| `"0123🦀5678"`     | Emoji / non-digit `char`.                                      |

### 5.4 Round-trip property [R8]

For every `n: NHSNumber`:

```
NHSNumber::from_str(&n.to_string()).unwrap() == n
```

The reverse direction also holds: a value produced by `from_str` from a
canonical string renders back to the same canonical string. This is
enforced by `src/lib.rs::tests::properties::round_trip_via_canonical_form`
and `round_trip_via_tight_form`.

### 5.5 Normalisation policy [R7]

The parser performs **no normalisation** of its input. Specifically, it
does not:

- trim leading or trailing whitespace,
- collapse runs of internal whitespace,
- swap alternative separators (hyphens, full-stops, NBSP, tabs) for spaces,
- accept non-ASCII digit characters even when they map to a value 0–9
  under Unicode's `Nd` category,
- accept uppercase or other non-digit characters that look like digits
  (e.g. lowercase `l` for `1`).

Callers whose upstream sources produce variant forms must normalise
before delegating to `FromStr`. This keeps the parser fast and its
contract small; richer ergonomics belong in caller code.

---

## 6. Check-digit algorithm

### 6.1 Definition (the canonical spec) [R9]

Given `digits[0..=8]`, the check digit is computed as follows:

1. **Weight and sum.** Multiply each of the first nine digits by
   `weight = 10 − i` (so `digits[0]` × 10, `digits[1]` × 9, …,
   `digits[8]` × 2). Sum the nine products.

2. **Take modulo 11.** Compute `remainder = sum mod 11`.
   `remainder ∈ {0, 1, 2, …, 10}`.

3. **Subtract from 11.** Compute `raw = 11 − remainder`.
   `raw ∈ {1, 2, …, 11}`.

4. **Map to the check digit:**
   - `raw == 11` → check digit is `0`.
   - `raw == 10` → the number is **invalid**; no digit in `0..=9` can stand
     in for the check digit, so such a number is not issued.
   - otherwise → check digit is `raw` (i.e. one of `1..=9`).

### 6.1.1 Sentinel for the "invalid" case [R10, R11]

`calculate_check_digit` must return an `i8` for every input, so the spec
reserves a sentinel value: when `raw == 10`, the function returns `10`.
Because every stored tenth digit is in `0..=9`, the sentinel can never
equal `check_digit()`, so `validate_check_digit` correctly returns `false`
for every number whose weighted sum is congruent to `1 (mod 11)`.

### 6.2 Worked example: `943 476 5919`

`digits = [9, 4, 3, 4, 7, 6, 5, 9, 1, 9]`

| i       | d[i] | weight | product |
| ------- | ---- | ------ | ------- |
| 0       | 9    | 10     | 90      |
| 1       | 4    | 9      | 36      |
| 2       | 3    | 8      | 24      |
| 3       | 4    | 7      | 28      |
| 4       | 7    | 6      | 42      |
| 5       | 6    | 5      | 30      |
| 6       | 5    | 4      | 20      |
| 7       | 9    | 3      | 27      |
| 8       | 1    | 2      | 2       |
| **sum** |      |        | **299** |

- `299 mod 11 = 2`
- `11 − 2 = 9`
- Check digit: `9`. Stored tenth digit: `9`. ✓ — branch `raw ∈ 1..=9`.

### 6.2.1 Worked example: `999 100 0100` (raw == 11)

`digits = [9, 9, 9, 1, 0, 0, 0, 1, 0, 0]`

- weighted sum: 90 + 81 + 72 + 7 + 0 + 0 + 0 + 3 + 0 = **253**.
- `253 mod 11 = 0`, `raw = 11 − 0 = 11`.
- Mapped to check digit: `0`.
- Stored tenth digit: `0`. ✓ — branch `raw == 11`.

### 6.2.2 Worked example: `999 123 4560` (raw == 10, sentinel)

`digits = [9, 9, 9, 1, 2, 3, 4, 5, 6, 0]`

- weighted sum: 90 + 81 + 72 + 7 + 12 + 15 + 16 + 15 + 12 = **320**.
- `320 mod 11 = 1`, `raw = 11 − 1 = 10`.
- No digit in `0..=9` fits, so `calculate_check_digit` returns the
  sentinel `10`.
- `validate_check_digit` returns `false` for every stored tenth digit
  `0..=9` because none of them can equal `10`.

### 6.3 What the check digit catches (and what it does not)

Detects:

- Every single-digit error.
- Every transposition of two adjacent digits.
- Most other common data-entry mistakes.

Does **not** detect:

- An attacker deliberately choosing digits that produce a valid checksum.
- The (rare) error patterns that happen to leave the checksum invariant.
- Whether a given number has been issued to a real patient — this is a
  syntactic check, not a registry lookup. See §10 for the patient-safety
  framing.

### 6.4 Behaviour on out-of-range digits

`calculate_check_digit` does no bounds checking on its `[i8; 10]` input.
If any digit is outside `0..=9`, the result is mathematically meaningless
at the spec level and the implementation's behaviour is undefined by this
spec.

The current implementation widens digits to `usize` before multiplying,
so it cannot panic from `i8` overflow on otherwise plausible input; but
that is an *implementation detail*, not a guaranteed behaviour. The
`FromStr` parser is the only supported entry point for caller-supplied
data — it enforces `0..=9` per digit.

### 6.5 Conformance summary

The implementation must agree with §6.1 / §6.1.1 across every input. In
particular, the three boundary cases are:

| `sum % 11` | `raw` | Result                                    |
| ---------- | ----- | ----------------------------------------- |
| 0          | 11    | check digit `0`                           |
| 1          | 10    | sentinel `10` — invalid, no digit fits    |
| 2..=10     | 9..=1 | `11 − remainder` (check digit in `1..=9`) |

Each case has a dedicated unit test in `src/lib.rs::tests::boundaries`.

---

## 7. Number ranges

### 7.1 Currently issued

| Range                         | Jurisdictions        |
| ----------------------------- | -------------------- |
| `300 000 000` – `399 999 999` | England              |
| `400 000 000` – `499 999 999` | England, Isle of Man |
| `600 000 000` – `799 999 999` | England, Isle of Man |

### 7.2 Reserved for other systems (not issued as NHS Numbers)

| Range                           | Used for                            |
| ------------------------------- | ----------------------------------- |
| `320 000 001` – `399 999 999`   | Northern Irish Health & Care system |
| `010 100 0000` – `311 299 9999` | CHI numbers in Scotland             |

### 7.3 Testable range

| Range                           | Use                                    |
| ------------------------------- | -------------------------------------- |
| `999 000 0000` – `999 999 9999` | Test data, demos, fixtures, seed data. |

This range is **never issued** to real patients. The crate exposes it via:

- `nhs_number::testable::TESTABLE_MIN` — `999 000 0000`,
- `nhs_number::testable::TESTABLE_MAX` — `999 999 9999`,
- `nhs_number::testable::TESTABLE_RANGE_INCLUSIVE`,
- `nhs_number::testable_random_sample()` (re-exported at the crate root).

The bounds are `LazyLock<NHSNumber>`; dereference with `*` to get the
underlying value. `RangeInclusive::contains(&n)` is the supported
membership check.

### 7.4 The crate does **not** validate ranges

`NHSNumber::new` and `FromStr` accept any ten-digit sequence regardless of
range. Range membership is a deployment-level concern (a real production
NHS Number falls into §7.1 ranges; a unit test fixture must come from
§7.3). The crate exposes the testable range so callers can write that check
without re-implementing it. Whether to add a built-in `is_issuable_range`
helper is tracked as §17.T3.

---

## 8. Random sampling

### 8.1 `testable_random_sample()` contract [R14, R15]

```rust
fn testable_random_sample() -> NHSNumber
```

- The first three digits are always `9, 9, 9` — every sample lands in the
  testable range.
- The remaining seven digits are drawn uniformly at random from `0..=9`
  using `rand::rng()`.
- The tenth (check) digit is **drawn randomly**, *not* computed. ≈90% of
  samples have an invalid check digit.
- Non-deterministic: two calls return different values (with very high
  probability).

### 8.2 Why not always-valid?

Returning invalid-checksum samples is **intentional**: it lets tests
exercise the rejection branch of `validate_check_digit`. Callers who need a
valid-by-checksum random sample have two options:

1. Loop:

   ```rust
   loop {
       let n = NHSNumber::testable_random_sample();
       if n.validate_check_digit() { break n; }
   }
   ```

2. Pick the first nine digits and compute the tenth with
   `calculate_check_digit`. See
   [`examples/generate_valid.rs`](examples/generate_valid.rs).

---

## 9. Ordering, equality, and collection use [R13]

- `PartialEq` / `Eq`: two `NHSNumber`s are equal iff their `digits` arrays
  are equal element-wise.
- `PartialOrd` / `Ord`: lexicographic comparison on the `digits` array.
  Because all values have the same length and the most-significant digit
  comes first, this matches natural numeric ordering.
- `Clone`, `Copy`: the struct is 10 bytes; copies are essentially free.

This makes `NHSNumber` directly usable as:

- a `Vec<NHSNumber>` element with `.sort()`,
- a `BTreeSet<NHSNumber>` element,
- a `BTreeMap<NHSNumber, V>` key.

`NHSNumber` does not derive `Hash`. If a future change adds it, it is a
minor-version bump and must come with a doc-test (see §17.T1).

A runnable demonstration of `Vec::sort` and `BTreeSet` lives in
[`examples/sorting.rs`](examples/sorting.rs); a `BTreeMap` example is
tracked as §17.T5.

---

## 10. Patient-safety framing

This spec is a behavioural contract for a Rust library. It is not a clinical
guarantee. In particular:

- A passing `validate_check_digit` confirms only the digits' arithmetic
  self-consistency, **not** that the number identifies a real patient.
- Numbers in the testable range are syntactically valid but are not — and
  must never be — used to represent a real patient.
- Real patient matching requires far more than this crate provides
  (demographic checks, registry lookup, governance, audit).

See [`AGENTS/safety.md`](AGENTS/safety.md) for the constraints that bind
contributors and AI agents working in this repo.

---

## 11. Serialisation

### 11.1 Default `serde` shape [R16]

The derived `Serialize`/`Deserialize` impls use the struct's default
layout. In JSON, that is:

```json
{ "digits": [9, 9, 9, 1, 2, 3, 4, 5, 6, 0] }
```

This shape is stable; changing it is a major-version bump.

### 11.2 String-form serialisation

Callers who want the human-readable `"999 123 4560"` form on the wire wrap
`NHSNumber` in a newtype and implement `Serialize`/`Deserialize` manually,
using `Display` and `FromStr`. The crate intentionally does **not** ship
this wrapper today — both wire shapes are reasonable, and the choice is
the caller's. An additive opt-in newtype is tracked as §17.T2.

---

## 12. Error handling

The crate exposes one error type:

```rust
pub struct ParseError;
```

- Returned only by `<NHSNumber as FromStr>::from_str`.
- Carries no payload — by design.
- Callers who want richer diagnostics (which length? which separator? which
  position?) should write a wrapper parser that pre-validates the string
  before delegating to `from_str`, **or** map it to a richer error type at
  the parse site:

  ```rust
  # use nhs_number::NHSNumber;
  # use std::str::FromStr;
  # #[derive(Debug, PartialEq)] enum MyError { BadNhsNumber(String) }
  let bad = "not a number";
  let result: Result<NHSNumber, MyError> =
      NHSNumber::from_str(bad).map_err(|_| MyError::BadNhsNumber(bad.into()));
  ```

The crate's own functions are otherwise **infallible** — `format`,
`check_digit`, `calculate_check_digit`, `validate_check_digit`, and the
testable-range helpers cannot fail and never panic on in-domain input
(per §6.4).

---

## 13. Testing strategy

The spec is enforced through three layers:

1. **Unit tests** in `#[cfg(test)] mod tests` blocks alongside each module.
2. **Doc-tests** in `///` comments on every public item.
3. **Examples** under [`examples/`](examples/) — each is a complete program
   that ends in `assert!` / `assert_eq!` checks.

Every assertion follows the `actual = …; expect = …; assert_eq!(actual,
expect);` pattern. New public items add **both** a unit test and a
doc-test. See [`AGENTS/testing.md`](AGENTS/testing.md).

### 13.1 Coverage targets

Every clause of this spec must map to at least one executable test:

| Spec clause                          | Tested in                                                                            |
| ------------------------------------ | ------------------------------------------------------------------------------------ |
| §3.1 invariants                      | `src/lib.rs::tests::structure::test_new_preserves_digits`                            |
| §3.2 struct-literal construction     | `src/lib.rs::tests::structure::test_struct_literal_construction`                     |
| §3.3 derived traits                  | `src/lib.rs::tests::traits::*`                                                       |
| §4.1 method ↔ §4.2 free-fn agreement | `src/lib.rs::tests::properties::method_and_free_fn_*_agree`                          |
| §4.5 ParseError shape                | `src/parse_error.rs::tests::*`                                                       |
| §5.1 canonical display               | `src/lib.rs::tests::structure::test_display_*`                                       |
| §5.1 `From`/`Into` agree with Display [R18] | `src/lib.rs::tests::structure::test_string_from`, `test_string_from_agrees_with_display_and_into` |
| §5.2 accepted input forms            | `src/from_str.rs::tests::test_from_str_with_length_10_*` and `_12_*`                 |
| §5.3 rejected input forms            | `src/from_str.rs::tests::test_from_str_with_*` (rejection)                           |
| §5.4 round-trip property             | `src/lib.rs::tests::properties::round_trip_via_canonical_form` / `..._tight_form`    |
| §5.5 normalisation policy            | `src/from_str.rs::tests::test_from_str_with_nbsp_separators` etc.                    |
| §6.1 algorithm                       | `src/lib.rs::tests::structure::test_calculate_check_digit`                           |
| §6.1.1 sentinel `10`                 | `src/lib.rs::tests::structure::test_calculate_check_digit`                           |
| §6.2 worked example (raw ∈ 1..=9)    | `src/lib.rs::tests::structure::test_calculate_check_digit` (`943 476 5919` case)     |
| §6.2.1 worked example (raw == 11)    | `src/lib.rs::tests::boundaries::sum_mod_11_eq_0_yields_check_digit_zero`             |
| §6.2.2 worked example (raw == 10)    | `src/lib.rs::tests::boundaries::sum_mod_11_eq_1_yields_sentinel_ten`                 |
| §6.5 `sum % 11 ∈ 2..=10` branch      | `src/lib.rs::tests::boundaries::sum_mod_11_in_2_to_10_yields_eleven_minus_remainder` |
| §7.3 testable bounds (exact)         | `src/testable.rs::tests::test_min_exact_value`, `test_max_exact_value`               |
| §7.3 testable range `.contains`      | `src/testable.rs::tests::test_range_inclusive_*`                                     |
| §8.1 random sample range             | `src/testable.rs::tests::test_random_sample_in_range`                                |
| §8.1 first three digits == 9 [R14]   | `src/testable.rs::tests::test_random_sample_first_three_digits_are_999`              |
| §8.1 non-determinism                 | `src/testable.rs::tests::test_random_sample_is_non_deterministic`                    |
| §9 ordering & collection use         | `src/lib.rs::tests::ordering::*`                                                     |
| §11.1 serde default shape            | `src/lib.rs::tests::traits::nhs_number_is_copy_clone_send_sync_serde`                |
| §12 `ParseError` zero-sized          | `src/parse_error.rs::tests::test_parse_error_is_zero_sized`                          |

Whenever a clause is added, extend this table in the same PR. Any row
whose right-hand side is empty is a §17 task by definition.

### 13.2 Test sub-module layout (in `src/lib.rs::tests`)

The unit tests are organised by intent so each test's purpose is obvious
from its path:

| Sub-module   | Holds                                                            |
| ------------ | ---------------------------------------------------------------- |
| `structure`  | Inherent methods on `NHSNumber` (`new`, `check_digit`, …).       |
| `utilities`  | Free-function counterparts (`format`, `check_digit`, …).         |
| `properties` | Invariants over many inputs (round-trip, method ↔ free-fn).      |
| `boundaries` | Explicit coverage of the `sum % 11 ∈ {0, 1, 2..=10}` branches.   |
| `ordering`   | `Ord`/`Eq`, `Vec::sort`, `BTreeSet`, `BTreeMap` use cases.       |
| `traits`     | Trait-impl smoke tests (`Copy`, `Clone`, `Send`, `Sync`, serde). |

New tests should land in the sub-module that matches their concern; do
not invent a parallel layout. If a genuinely new concern appears (e.g.
benchmarks, fuzz-target shims), add a sub-module here and reference it
from this table and from §13.1.

### 13.3 Canonical test fixtures

The following `NHSNumber` values are the canonical fixtures used across
unit tests, doc-tests, and examples. They cover boundary digits, the two
Wikipedia reference numbers, and the testable-range extremes. Adding to
this list is fine; removing or changing a value requires updating every
call site.

| Digits                          | String form    | Why                                              |
| ------------------------------- | -------------- | ------------------------------------------------ |
| `[0; 10]`                       | `000 000 0000` | All-zero edge case; `sum == 0`, raw `== 11`.     |
| `[9; 10]`                       | `999 999 9999` | All-nine edge case; `TESTABLE_MAX`; raw `== 9`.  |
| `[0,1,2,3,4,5,6,7,8,9]`         | `012 345 6789` | Mixed digits; stored check digit 9.              |
| `[9,4,3,4,7,6,5,9,1,9]`         | `943 476 5919` | Wikipedia checksum example; raw `∈ 1..=9`.       |
| `[9,8,7,6,5,4,4,3,2,1]`         | `987 654 4321` | Wikipedia format example.                        |
| `[9,9,9,0,0,0,0,0,0,0]`         | `999 000 0000` | `TESTABLE_MIN`.                                  |
| `[9,9,9,1,0,0,0,0,0,3]`         | `999 100 0003` | Typical testable, valid by checksum.             |
| `[9,9,9,1,0,0,0,1,0,0]`         | `999 100 0100` | Boundary: `sum % 11 == 0` → check digit 0.       |
| `[9,9,9,1,2,3,4,5,6,0]`         | `999 123 4560` | Boundary: `sum % 11 == 1` → sentinel `10`.       |

Tests that need a non-fixture value (e.g. a random sample) should still
assert a property, not an exact value.

### 13.4 What is not tested here

- Real patient data — never. See [`AGENTS/safety.md`](AGENTS/safety.md).
- Compliance against external NHS systems — the crate models the format
  and algorithm; integration testing is the deploying organisation's job.
- Performance — there is no benchmark suite. The hot paths are ten `i8`
  multiplies; add a bench only if a real workload demonstrates a
  regression.
- Property-based / fuzz testing — not currently used. A future opt-in via
  `proptest` would be additive (§17.T7 if proposed).

---

## 14. Compatibility and versioning

The crate follows [Semantic Versioning](https://semver.org/):

| Change                                                | Bump  |
| ----------------------------------------------------- | ----- |
| Add a new public item.                                | minor |
| Add a new derived trait on `NHSNumber` (e.g. `Hash`). | minor |
| Tighten or relax a parsing rule.                      | major |
| Change the `Display`/`Into<String>` output.           | major |
| Change the serialised `digits` shape.                 | major |
| Change the value returned by `calculate_check_digit`. | major |
| Change which numbers `validate_check_digit` accepts.  | major |
| Remove or rename a public item.                       | major |
| Change a public item's signature.                     | major |
| Add a new test.                                       | patch |
| Add or expand a doc-test.                             | patch |
| Add a new `examples/*` program.                       | patch |
| Add or expand a `//!` or `///` doc comment.           | patch |
| Fix an internal bug with no observable change.        | patch |
| Documentation-only change (including this spec).      | patch |

Releases are cut per [`AGENTS/release.md`](AGENTS/release.md).

---

## 15. Dependencies and build

### 15.1 Runtime dependencies

| Crate   | Purpose                                                   |
| ------- | --------------------------------------------------------- |
| `rand`  | Random sampling for `testable_random_sample`.             |
| `serde` | Derived `Serialize`/`Deserialize` (default struct shape). |

Adding a runtime dependency that affects callers is a major-version
decision; bumping minor or patch versions of an existing dependency is
typically a patch release. The dependency surface is intentionally tiny;
new runtime deps require a justification in the PR description.

### 15.2 Build-only tooling

- `rustdoc-llms` regenerates `llms.txt` and `llms.json` at release time
  (see [`AGENTS/release.md`](AGENTS/release.md)). It is **not** a
  dev-dependency — it is a binary with no lib target, so listing it in
  `Cargo.toml` would warn and be ignored. Install with `cargo install
  rustdoc-llms`.

### 15.3 Edition and MSRV

- Rust edition: **2024**.
- MSRV: tracks the edition (the crate uses 2024-edition features such as
  `LazyLock` re-exports). If a future change raises MSRV, document the new
  minimum in this section in the same PR and decide whether the bump is
  minor or major per the project's downstream commitments.

### 15.4 Published file set

`Cargo.toml` ships only:

```toml
include = ["src/**/*", "LICENSE.md", "README.md"]
```

So `examples/`, `docs/`, `help/`, `spec.md`, `AGENTS.md`, and `AGENTS/` are
**not** uploaded to crates.io — they exist on GitHub only. If a new
top-level file becomes required at install time, extend `include` to match.

### 15.5 Required release checks

Every release must pass:

```sh
cargo build --release
cargo test            # unit + doc-tests + examples (via cargo build --examples)
cargo build --examples
cargo clippy --all-targets
cargo fmt -- --check
cargo doc --no-deps
```

Each check is the gate on a known class of regression; do not skip one
because the others pass.

---

## 16. Roadmap

The roadmap captures direction. It is intentionally short — each entry is
either being actively designed or scheduled to be. Larger / further-out
ideas live in §17 (backlog) until they earn a slot here.

### 16.1 Near-term (next minor release)

- Keep the §13.1 coverage table in lock-step with `src/`. Whenever a spec
  clause is added, the matching test must land in the same PR.

### 16.2 Mid-term (post-1.x)

- Decide whether `NHSNumber` should derive `Hash`. See §17.T1.
- Decide whether to ship an opt-in `Serialize`/`Deserialize` newtype that
  serialises as `"DDD DDD DDDD"` rather than `{ "digits": [...] }`. See
  §17.T2.
- Promote the in-tree `BTreeMap` test into a runnable example. See §17.T5.

### 16.3 Long-term / explicit non-goals

- No registry integration (§1.3).
- No localised string forms (§1.3).
- No bundled range-membership predicate beyond `TESTABLE_RANGE_INCLUSIVE`
  unless §17.T3 is accepted.

A roadmap entry is "done" only when its associated spec section, tests,
and docs all agree.

---

## 17. Open tasks (backlog)

Each task is a small, specific unit of work. A task moves into the roadmap
(§16) when it is scheduled; it leaves this section when it ships (the
change should leave the spec in a state where the task is no longer
needed). Completed tasks are deleted, not archived in place — the commit
history is the changelog.

Tasks are labelled `T<number>` so they can be referenced from commits and
issues. **IDs are never reused** even after a task ships, so future
references stay unambiguous.

### T1 — Decide whether to derive `Hash`

- **Why:** Without `Hash`, `NHSNumber` cannot be used as a `HashMap` /
  `HashSet` key. Adding `Hash` is additive and cheap.
- **Done when:** §3.3 lists `Hash`; a doctest on `NHSNumber` exercises a
  `HashMap`-keyed example; §14 confirms this is a minor bump.

### T2 — Opt-in string-form serde wrapper

- **Why:** Some callers want `"999 123 4560"` on the wire instead of the
  default struct shape. Today they must hand-roll the newtype.
- **Done when:** A new module (e.g. `nhs_number::serde_string`) exposes a
  newtype with `Serialize`/`Deserialize` impls backed by `Display` /
  `FromStr`; §11 documents it; a doctest exercises round-tripping.

### T3 — Optional `is_issuable_range` helper

- **Why:** Callers asking "is this a real issued range?" today re-derive
  the ranges from §7.1. A helper would centralise the answer.
- **Done when:** A function such as
  `NHSNumber::is_issuable_range(&self) -> bool` (or free-function
  equivalent) is added; §7 documents the predicate; tests cover each
  issued and reserved range boundary.

### T5 — Runnable `BTreeMap` example

- **Why:** §9 names `BTreeMap<NHSNumber, V>` as a supported use case and
  `src/lib.rs::tests::ordering::btreemap_use_as_key` verifies it, but
  there is no narrative example demonstrating the pattern to readers.
- **Done when:** [`examples/sorting.rs`](examples/sorting.rs) (or a
  sibling file) demonstrates building a `BTreeMap<NHSNumber, _>`; the
  examples index in [`examples/README.md`](examples/README.md) references
  it.

> T4 and T6 closed in this revision; see commit history.

---

## 18. Open questions and known divergences

This section is **not** a backlog of unrelated wishlist items (those live
in §17) — it is the list of points where the crate's behaviour, its spec,
and the published NHS specification do not fully agree, plus genuinely
unresolved design questions.

### 18.1 `Hash` is not derived

`NHSNumber` could derive `Hash` (its digits array is `Hash`). It does
not, so it cannot be used as a `HashMap` / `HashSet` key. Adding `Hash`
is a minor release; the work is tracked as §17.T1.

### 18.2 The default serde shape

The default `{ "digits": [...] }` shape exposes a Rust struct field name
on the wire. Some callers will prefer the string form `"999 100 0003"` as
the default. Switching the default would be a major release; an additive
opt-in newtype is tracked as §17.T2.

### 18.3 Range checks are not enforced

`NHSNumber::new` and `FromStr` accept ten-digit sequences outside the
issued/testable ranges (§7). This is deliberate — the crate parses
*format*, not *provenance* — but it does mean callers have to assemble
their own membership check using `TESTABLE_RANGE_INCLUSIVE` and any
production-range checks they need. Whether to add a helper is tracked as
§17.T3.

### 18.4 MSRV vs. edition 2024 features

The crate uses `std::sync::LazyLock` and the 2024 edition, which sets an
effective MSRV of approximately Rust 1.80. We have not committed to a
narrower MSRV than "current stable"; downstream projects with stricter
MSRV requirements would either need a back-ported variant or a pinned
older version of this crate. Whether to publish a formal MSRV policy is
open.

---

## 19. Glossary

| Term                | Meaning                                                                |
| ------------------- | ---------------------------------------------------------------------- |
| NHS Number          | The ten-digit identifier defined in §2.1.                              |
| Check digit         | The tenth digit, computed per §6.                                      |
| Canonical form      | `"DDD DDD DDDD"` — three groups separated by single spaces (§5.1).     |
| Tight form          | `"DDDDDDDDDD"` — ten contiguous digits, no separators (§5.2).          |
| Testable range      | `999 000 0000` – `999 999 9999`, never issued to real patients (§7.3). |
| Issued range        | Any of the ranges in §7.1.                                             |
| Validates           | `check_digit() == calculate_check_digit()`.                            |
| Strict spec         | The NHS-published check-digit algorithm in §6.1.                       |
| Sentinel `10`       | The value `calculate_check_digit` returns when `sum % 11 == 1` (§6.1.1). |
| Rule `R<n>`         | A numbered behavioural rule (§2.3).                                    |
| Task `T<n>`         | A numbered backlog item (§17).                                         |
| Canonical fixture   | An `NHSNumber` value listed in §13.3.                                  |
