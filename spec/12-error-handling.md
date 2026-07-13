[`nhs-number` specification](index.md) — section 12 of 19. Section numbers (§12.x) are stable and cited from code, tests, and commit messages.

# 12. Error handling

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
testable-range helpers cannot fail. The check-digit functions never
panic on **any** `[i8; 10]` input, in-domain or not (per §6.4).
