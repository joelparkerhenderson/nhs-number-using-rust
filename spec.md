# spec.md — `nhs-number` crate specification

**Status:** living document. Updated alongside every behavioural change.
**Audience:** maintainers, AI agents, downstream integrators reading the
crate's contract.
**Companion docs:** [`AGENTS.md`](AGENTS.md) for agent guidance,
[`index.md`](index.md) for the user-facing README,
[`docs/api/index.md`](docs/api/index.md) for the rendered API surface.

This file follows **spec-driven development** (see
[`AGENTS/spec-driven-development.md`](AGENTS/spec-driven-development.md)):
when the spec and the code disagree, the spec is the source of truth.

---

## 1. Purpose and scope

### 1.1 Purpose

Provide a small, dependable Rust value type — `NHSNumber` — that models a
National Health Service (NHS) Number across **NHS England**, **NHS Wales**,
and **NHS Isle of Man**, with first-class support for:

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

### 1.3 Out of scope (non-goals)

- Looking up whether a given NHS Number has actually been issued to a
  patient (no registry access).
- Mapping NHS Numbers to patient identities, demographics, or care records.
- Validating non-NHS-England/Wales/IoM identifiers (e.g. Scottish CHI,
  Northern Irish HCN).
- Cryptographic protection of NHS Numbers in transit or at rest.
- Localisation of the rendered string form (it is fixed: `"DDD DDD DDDD"`).

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
what the crate implements; record the disagreement in §15.

---

## 3. Data model

### 3.1 The `NHSNumber` struct

```rust
pub struct NHSNumber {
    pub digits: [i8; 10],
}
```

**Invariants:**

- `digits` is always exactly ten elements (enforced by the type).
- Each element **should** be in `0..=9`. Out-of-range values are not
  rejected at construction, but cause undefined behaviour in the check-digit
  algorithm — see §6.4. Callers that take untrusted input must parse via
  `FromStr`, which enforces the digit range.
- `digits[0]` is the most-significant digit (the leftmost in the displayed
  form).

### 3.2 Public field

The `digits` field is `pub`. Callers may construct an `NHSNumber` directly
via a struct literal:

```rust
let n = NHSNumber { digits: [9, 9, 9, 1, 2, 3, 4, 5, 6, 0] };
```

This is documented and stable; do not remove `pub` from the field.

### 3.3 Derived traits

| Trait                      | Semantics                                              |
| -------------------------- | ------------------------------------------------------ |
| `Debug`                    | Standard derive.                                       |
| `Clone`, `Copy`            | Cheap — the struct is 10 bytes.                        |
| `PartialEq`, `Eq`          | Digit-by-digit equality.                               |
| `PartialOrd`, `Ord`        | Lexicographic on the digit array (matches numeric).    |
| `Serialize`, `Deserialize` | Serde with the default struct layout (`{ "digits": […] }`). |

---

## 4. Public API surface

The full surface is fixed by this section. Adding to it is a minor-version
bump; changing or removing anything is a major-version bump (§14).

### 4.1 Methods on `NHSNumber`

| Signature                                          | Purpose                                                |
| -------------------------------------------------- | ------------------------------------------------------ |
| `NHSNumber::new(digits: [i8; 10]) -> NHSNumber`    | Construct from a ten-digit array.                      |
| `NHSNumber::check_digit(&self) -> i8`              | Return the tenth digit as stored.                      |
| `NHSNumber::calculate_check_digit(&self) -> i8`    | Compute the tenth digit from digits 0..9 (see §6).     |
| `NHSNumber::validate_check_digit(&self) -> bool`   | `check_digit() == calculate_check_digit()`.            |
| `NHSNumber::testable_random_sample() -> NHSNumber` | Random value in the testable range (see §8).           |

### 4.2 Free functions on `[i8; 10]`

| Signature                                            | Equivalent to                              |
| ---------------------------------------------------- | ------------------------------------------ |
| `fn format(digits: [i8; 10]) -> String`              | `NHSNumber::to_string()` / `Into<String>`  |
| `fn check_digit(digits: [i8; 10]) -> i8`             | `NHSNumber::check_digit`                   |
| `fn calculate_check_digit(digits: [i8; 10]) -> i8`   | `NHSNumber::calculate_check_digit`         |
| `fn validate_check_digit(digits: [i8; 10]) -> bool`  | `NHSNumber::validate_check_digit`          |
| `fn testable::testable_random_sample() -> NHSNumber` | `NHSNumber::testable_random_sample`        |

Each free function and its corresponding method **must** return the same
value on the same input. This is an enforced invariant via tests.

### 4.3 Trait implementations

| Impl                           | Behaviour                                       |
| ------------------------------ | ----------------------------------------------- |
| `Display`                      | Format as `"DDD DDD DDDD"` (see §5).            |
| `Into<String>`                 | Delegates to `to_string()`.                     |
| `FromStr`, `Err = ParseError`  | Parse `"DDDDDDDDDD"` or `"DDD DDD DDDD"` (§5).  |

### 4.4 The `testable` module

```rust
pub static TESTABLE_MIN: LazyLock<NHSNumber>;            // 999 000 0000
pub static TESTABLE_MAX: LazyLock<NHSNumber>;            // 999 999 9999
pub static TESTABLE_RANGE_INCLUSIVE: LazyLock<RangeInclusive<NHSNumber>>;
pub fn testable_random_sample() -> NHSNumber;
```

Re-exported at the crate root via `pub use testable::*;`, so callers may
write `nhs_number::testable_random_sample()` and `*nhs_number::TESTABLE_MIN`.

### 4.5 The `ParseError` type

```rust
pub struct ParseError;
```

Zero-sized unit struct. Used only as the `FromStr::Err` associated type.
The type intentionally carries no detail; callers who need richer error
reporting wrap or map it at the parse site.

---

## 5. String forms (parsing and formatting)

### 5.1 Canonical display form

`Display` and `Into<String>` always produce **exactly** twelve characters:

```
DDD DDD DDDD
```

- three digits, single space, three digits, single space, four digits;
- no leading, trailing, or doubled spaces;
- no alternative separators (hyphen, period, slash, NBSP).

### 5.2 Accepted input forms

`FromStr::from_str` accepts **exactly two** shapes:

1. **Ten contiguous digits**, no separators: `"DDDDDDDDDD"` (length 10).
2. **Canonical with single spaces**: `"DDD DDD DDDD"` (length 12, space at
   position 3 and position 7 only).

Both must be ASCII digits 0–9 in every digit position.

### 5.3 Rejected input forms

Everything else returns `Err(ParseError)`. Notable examples:

| Input             | Reason                                                   |
| ----------------- | -------------------------------------------------------- |
| `""`              | Length 0 — neither accepted length.                      |
| `"12345"`         | Length 5.                                                |
| `"01234567890"`   | Length 11.                                               |
| `"012-345-6789"`  | Length 12 but separators are hyphens.                    |
| `" 012 345 6789"` | Leading space — position 3 is a digit, not the required space. |
| `"012 345 6789 "` | Trailing space — length becomes 13.                      |
| `"012  345  6789"`| Doubled spaces shift the second group out of position.   |
| `"012 3456789"`   | One space only — length 11.                              |
| `"012345 6789"`   | One space only, wrong place — length 11.                 |
| `"abc 123 4567"`  | Non-digit characters.                                    |

Upstream normalisation (uppercasing, stripping extra whitespace, swapping
hyphens for spaces) is **caller-side**, not parser-side.

### 5.4 Round-trip property

For every `n: NHSNumber`:

```
NHSNumber::from_str(&n.to_string()).unwrap() == n
```

This is an enforced invariant and a hot test target.

---

## 6. Check-digit algorithm

### 6.1 Definition (the canonical spec)

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

### 6.1.1 Sentinel for the "invalid" case

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
- Check digit: `9`. Stored tenth digit: `9`. ✓

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

`calculate_check_digit` does no bounds checking on its `[i8; 10]` input. If
any digit is outside `0..=9`, the result is mathematically meaningless and
may overflow `i8`. Treat the function as having domain `0..=9` per digit.
The `FromStr` parser enforces this domain on caller-supplied strings.

### 6.5 Conformance summary

The implementation must agree with §6.1 / §6.1.1 across every input. In
particular, the two boundary cases are:

| `sum % 11` | Result                                                  |
| ---------- | ------------------------------------------------------- |
| 0          | check digit `0` (raw `= 11`)                            |
| 1          | sentinel `10` — invalid, no digit fits                  |
| 2..=10     | `11 − remainder` (check digit in `1..=9`)               |

These cases each have a dedicated unit test in `src/lib.rs`.

---

## 7. Number ranges

### 7.1 Currently issued

| Range                         | Jurisdictions               |
| ----------------------------- | --------------------------- |
| `300 000 000` – `399 999 999` | England                     |
| `400 000 000` – `499 999 999` | England, Wales, Isle of Man |
| `600 000 000` – `799 999 999` | England, Wales, Isle of Man |

### 7.2 Reserved for other systems (not issued as NHS Numbers)

| Range                           | Used for                            |
| ------------------------------- | ----------------------------------- |
| `320 000 001` – `399 999 999`   | Northern Irish Health & Care system |
| `010 100 0000` – `311 299 9999` | CHI numbers in Scotland             |

### 7.3 Testable range

| Range                           | Use                                       |
| ------------------------------- | ----------------------------------------- |
| `999 000 0000` – `999 999 9999` | Test data, demos, fixtures, seed data.    |

This range is **never issued** to real patients. The crate exposes it via:

- `nhs_number::testable::TESTABLE_MIN` — `999 000 0000`,
- `nhs_number::testable::TESTABLE_MAX` — `999 999 9999`,
- `nhs_number::testable::TESTABLE_RANGE_INCLUSIVE`,
- `nhs_number::testable_random_sample()` (re-exported at the crate root).

The bounds are `LazyLock<NHSNumber>`; dereference with `*` to get the
underlying value.

### 7.4 The crate does **not** validate ranges

`NHSNumber::new` and `FromStr` accept any ten-digit sequence regardless of
range. Range membership is a deployment-level concern (a real production
NHS Number falls into §7.1 ranges; a unit test fixture must come from
§7.3). The crate exposes the testable range so callers can write that check
without re-implementing it.

---

## 8. Random sampling

### 8.1 `testable_random_sample()` contract

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

## 9. Ordering, equality, and collection use

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
minor-version bump and must come with a doc-test.

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

### 11.1 Default `serde` shape

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
this wrapper — both wire shapes are reasonable, and the choice is the
caller's.

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
  before delegating to `from_str`.

The crate's own functions are otherwise **infallible** — `format`,
`check_digit`, `calculate_check_digit`, `validate_check_digit`, and the
testable-range helpers cannot fail and never panic on in-domain input
(per §6.4).

---

## 13. Testing strategy

The spec is enforced through:

1. **Unit tests** in `#[cfg(test)] mod tests` blocks alongside each module.
2. **Doc-tests** in `///` comments on every public item.
3. **Examples** under `examples/` — each is a complete program that ends
   in `assert!` / `assert_eq!` checks.

Every assertion follows the `actual = …; expect = …; assert_eq!(actual,
expect);` pattern. New public items add **both** a unit test and a
doc-test. See [`AGENTS/testing.md`](AGENTS/testing.md).

---

## 14. Compatibility and versioning

The crate follows [Semantic Versioning](https://semver.org/):

| Change                                                          | Bump  |
| --------------------------------------------------------------- | ----- |
| Add a new public item.                                          | minor |
| Tighten or relax a parsing rule.                                | major |
| Change the `Display`/`Into<String>` output.                     | major |
| Change the serialised `digits` shape.                           | major |
| Change the value returned by `calculate_check_digit`.           | major |
| Change which numbers `validate_check_digit` accepts.            | major |
| Fix an internal bug with no observable change.                  | patch |
| Documentation-only change (including this spec).                | patch |

Releases are cut per [`AGENTS/release.md`](AGENTS/release.md).

---

## 15. Open questions and known divergences

This section is **not** a backlog of unrelated wishlist items — it is the
list of points where the crate's behaviour, its spec, and the published NHS
specification do not fully agree, plus genuinely unresolved design
questions.

### 15.1 `Hash` is not derived

`NHSNumber` could derive `Hash` (its digits array is `Hash`). It does not,
so it cannot be used as a `HashMap`/`HashSet` key. Adding `Hash` is a minor
release; it should come with a doc-test that exercises a `HashMap`-keyed
use case.

### 15.2 The default serde shape

The default `{ "digits": [...] }` shape exposes a Rust struct field name
on the wire. Some callers will prefer the string form `"999 100 0003"` as
the default. Switching the default would be a major release; an
alternative is to provide an opt-in newtype in a future version (additive,
minor).

### 15.3 Range checks are not enforced

`NHSNumber::new` and `FromStr` accept ten-digit sequences outside the
issued/testable ranges (§7). This is deliberate — the crate parses
*format*, not *provenance* — but it does mean callers have to assemble
their own membership check using `TESTABLE_RANGE_INCLUSIVE` and any
production-range checks they need. A future helper (e.g.
`is_issuable_range`) would be additive; whether to add it is open.

---

## 16. Glossary

| Term                | Meaning                                                              |
| ------------------- | -------------------------------------------------------------------- |
| NHS Number          | The ten-digit identifier defined in §2.1.                            |
| Check digit         | The tenth digit, computed per §6.                                    |
| Canonical form      | `"DDD DDD DDDD"` — three groups separated by single spaces (§5.1).   |
| Tight form          | `"DDDDDDDDDD"` — ten contiguous digits, no separators (§5.2).        |
| Testable range      | `999 000 0000` – `999 999 9999`, never issued to real patients (§7.3). |
| Issued range        | Any of the ranges in §7.1.                                           |
| Validates           | `check_digit() == calculate_check_digit()`.                          |
| Strict spec         | The NHS-published check-digit algorithm in §6.1.                     |
