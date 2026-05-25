# API

This page describes the full public surface of the `nhs-number` crate.

For runnable programs demonstrating each API, see
[`../../examples/`](../../examples/). For a usage-flavour walk-through, see
[usage](../usage/index.md).

## Crate layout

| Item path                                        | Kind       |
| ------------------------------------------------ | ---------- |
| `nhs_number::NHSNumber`                          | struct     |
| `nhs_number::format`                             | fn         |
| `nhs_number::check_digit`                        | fn         |
| `nhs_number::calculate_check_digit`              | fn         |
| `nhs_number::validate_check_digit`               | fn         |
| `nhs_number::testable_random_sample`             | fn (re-ex) |
| `nhs_number::parse_error::ParseError`            | struct     |
| `nhs_number::testable::TESTABLE_MIN`             | static     |
| `nhs_number::testable::TESTABLE_MAX`             | static     |
| `nhs_number::testable::TESTABLE_RANGE_INCLUSIVE` | static     |
| `nhs_number::testable::testable_random_sample`   | fn         |

## `struct NHSNumber`

```rust
pub struct NHSNumber {
    pub digits: [i8; 10],
}
```

### Derived traits

| Trait                      | Notes                                       |
| -------------------------- | ------------------------------------------- |
| `Debug`                    | Standard derive.                            |
| `Clone`, `Copy`            | Cheap — struct is 10 bytes.                 |
| `PartialEq`, `Eq`          | Digit-by-digit equality.                    |
| `PartialOrd`, `Ord`        | Lexicographic, matches numeric ordering.    |
| `Serialize`, `Deserialize` | Via `serde`. Serializes the `digits` field. |

### Hand-written impls

| Impl                           | Format / behaviour                                                                |
| ------------------------------ | --------------------------------------------------------------------------------- |
| `Display`                      | `"DDD DDD DDDD"` — 3, 3, 4 digits separated by spaces.                            |
| `From<NHSNumber> for String`   | Delegates to `to_string()`. Provides `Into<String>` via the std blanket impl.     |
| `FromStr` (err = `ParseError`) | Accepts `"DDDDDDDDDD"` or `"DDD DDD DDDD"`.                                       |

### Inherent methods

| Signature                                          | Purpose                                     |
| -------------------------------------------------- | ------------------------------------------- |
| `NHSNumber::new(digits: [i8; 10]) -> NHSNumber`    | Construct from a ten-digit array.           |
| `NHSNumber::check_digit(&self) -> i8`              | Return the tenth digit as stored.           |
| `NHSNumber::calculate_check_digit(&self) -> i8`    | Compute the tenth digit from digits 0..9.   |
| `NHSNumber::validate_check_digit(&self) -> bool`   | `check_digit() == calculate_check_digit()`. |
| `NHSNumber::testable_random_sample() -> NHSNumber` | Random value in the testable range.         |

## `struct ParseError`

```rust
pub struct ParseError;
```

Zero-sized unit marker. Returned by `<NHSNumber as FromStr>::from_str` for any
input that is not one of the two accepted shapes. No `Display` is implemented —
callers typically map it to their own richer error type at the parse site.

## Free functions

Each free function is a thin wrapper around the equivalent `NHSNumber` method,
and is useful when you already have a `[i8; 10]` and do not want the wrapper
struct.

| Signature                                            | Equivalent method                         |
| ---------------------------------------------------- | ----------------------------------------- |
| `fn format(digits: [i8; 10]) -> String`              | `NHSNumber::to_string()` / `Into<String>` |
| `fn check_digit(digits: [i8; 10]) -> i8`             | `NHSNumber::check_digit`                  |
| `fn calculate_check_digit(digits: [i8; 10]) -> i8`   | `NHSNumber::calculate_check_digit`        |
| `fn validate_check_digit(digits: [i8; 10]) -> bool`  | `NHSNumber::validate_check_digit`         |
| `fn testable::testable_random_sample() -> NHSNumber` | `NHSNumber::testable_random_sample`       |

## `testable` module

```rust
pub static TESTABLE_MIN: LazyLock<NHSNumber>;            // 999 000 0000
pub static TESTABLE_MAX: LazyLock<NHSNumber>;            // 999 999 9999
pub static TESTABLE_RANGE_INCLUSIVE: LazyLock<RangeInclusive<NHSNumber>>;
pub fn testable_random_sample() -> NHSNumber;
```

- `TESTABLE_MIN` / `TESTABLE_MAX` are lazily initialised static `NHSNumber`s —
  dereference (`*TESTABLE_MIN`) to get the value.
- `TESTABLE_RANGE_INCLUSIVE` supports `.contains(&nhs_number)`.
- `testable_random_sample` returns an `NHSNumber` with the first three digits
  pinned to `9, 9, 9` and the remaining seven drawn from `rand::rng()`. The
  returned number is **not** guaranteed to have a valid check digit.

See [ranges](../ranges/index.md) for the broader set of issuable and
reserved ranges.

## Stability

This crate follows [Semantic Versioning](https://semver.org/). Breaking
changes — including changes to the serialized `digits` shape, the
`Display`/`FromStr` formats, or the set of accepted parser inputs — require a
major-version bump.
