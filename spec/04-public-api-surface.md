[`nhs-number` specification](index.md) — section 4 of 19. Section numbers (§4.x) are stable and cited from code, tests, and commit messages.

# 4. Public API surface

The full surface is fixed by this section. Adding to it is a minor-version
bump; changing or removing anything is a major-version bump (§14).

### 4.1 Methods on `NHSNumber`

| Signature                                          | Purpose                                            |
| -------------------------------------------------- | -------------------------------------------------- |
| `NHSNumber::new(digits: [i8; 10]) -> NHSNumber`    | Construct from a ten-digit array.                  |
| `NHSNumber::check_digit(&self) -> i8`              | Return the tenth digit as stored.                  |
| `NHSNumber::calculate_check_digit(&self) -> i8`    | Compute the tenth digit from digits 0..9 (see §6). |
| `NHSNumber::validate_check_digit(&self) -> bool`   | `check_digit() == calculate_check_digit()`.        |
| `NHSNumber::is_issuable_range(&self) -> bool`      | First nine digits in an issued range (see §7.5).   |
| `NHSNumber::testable_random_sample() -> NHSNumber` | Random value in the testable range (see §8).       |

### 4.2 Free functions on `[i8; 10]` [R3]

| Signature                                            | Equivalent to                             |
| ---------------------------------------------------- | ----------------------------------------- |
| `fn format(digits: [i8; 10]) -> String`              | `NHSNumber::to_string()` / `Into<String>` |
| `fn check_digit(digits: [i8; 10]) -> i8`             | `NHSNumber::check_digit`                  |
| `fn calculate_check_digit(digits: [i8; 10]) -> i8`   | `NHSNumber::calculate_check_digit`        |
| `fn validate_check_digit(digits: [i8; 10]) -> bool`  | `NHSNumber::validate_check_digit`         |
| `fn is_issuable_range(digits: [i8; 10]) -> bool`     | `NHSNumber::is_issuable_range`            |
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
| Impls [R21]      | `Display` (fixed message `"invalid NHS Number string"`, never echoes input) and `std::error::Error`. |
| Returned by      | `<NHSNumber as FromStr>::from_str` and `serde_string::NHSNumberString::from_str`. |
| Payload          | None — by design. See §12.                                      |

The type intentionally carries no detail; callers who need richer error
reporting wrap or map it at the parse site (see §12 for the recommended
pattern). The `Error` impl means it also flows through `?` into
`Box<dyn Error>` / `anyhow`-style stacks unmapped.
