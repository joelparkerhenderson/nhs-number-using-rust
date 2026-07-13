[`nhs-number` specification](index.md) — section 7 of 19. Section numbers (§7.x) are stable and cited from code, tests, and commit messages.

# 7. Number ranges

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

### 7.4 The crate does **not** enforce ranges at construction

`NHSNumber::new` and `FromStr` accept any ten-digit sequence regardless of
range. Range membership is a deployment-level concern (a real production
NHS Number falls into §7.1 ranges; a unit test fixture must come from
§7.3). The crate exposes the testable range, and the opt-in predicate in
§7.5, so callers can write that check without re-implementing it.

### 7.5 The issuable-range predicate [R23]

```rust
fn is_issuable_range(digits: [i8; 10]) -> bool      // free function
fn NHSNumber::is_issuable_range(&self) -> bool      // method
```

Returns `true` iff the number's **first nine digits**, read as a single
integer `n9`, fall inside a currently-issued range **net of the ranges
reserved for other systems**. Derivation from §7.1 minus §7.2:

| `n9`                          | Issuable? | Why                                          |
| ----------------------------- | --------- | -------------------------------------------- |
| `000 000 000` – `311 299 999` | no        | Not issued, or Scottish CHI reservation (the ten-digit CHI range `010 100 0000`–`311 299 9999` covers exactly first-nine `010 100 000`–`311 299 999`). |
| `311 300 000` – `320 000 000` | **yes**   | England (§7.1) net of both reservations.     |
| `320 000 001` – `399 999 999` | no        | Northern Irish reservation (§7.2).           |
| `400 000 000` – `499 999 999` | **yes**   | England, Isle of Man (§7.1).                 |
| `500 000 000` – `599 999 999` | no        | Not issued.                                  |
| `600 000 000` – `799 999 999` | **yes**   | England, Isle of Man (§7.1).                 |
| `800 000 000` – `999 999 999` | no        | Not issued; `999…` is the testable range.    |

Contract details:

- **Range check only.** The tenth (check) digit is ignored entirely. A
  number that could really be issued must *also* pass
  `validate_check_digit`, and even then only a registry lookup confirms
  issuance (§10).
- **Total.** Digits outside `0..=9` return `false` — an out-of-domain
  digit can never occur in an issued number. The function never panics.
- **Testable range** values always return `false`.
- Updating this table when the NHS opens a new allocation range is a
  behavioural change: update this section, the boundary tests in
  `src/lib.rs::tests::ranges`, and treat the release per §14.

Test fixtures for this predicate necessarily lie outside the testable
range; they must carry a deliberately **invalid** stored check digit so
they cannot denote a real issued number (see `AGENTS/safety.md` §1).
