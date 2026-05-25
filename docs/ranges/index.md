# NHS Number Ranges

NHS Numbers are ten-digit identifiers shared across NHS England and NHS Isle
of Man. The numeric range determines where and whether a number may have
been issued.

## Reference

- [NHS Number — Wikipedia](https://en.wikipedia.org/wiki/NHS_number)

## Currently issued ranges

| Range                         | Jurisdictions        |
| ----------------------------- | -------------------- |
| `300 000 000` – `399 999 999` | England              |
| `400 000 000` – `499 999 999` | England, Isle of Man |
| `600 000 000` – `799 999 999` | England, Isle of Man |

## Unavailable ranges

These ranges exist but are **not** used for NHS (England / Isle of Man)
numbers:

| Range                           | Used for                            |
| ------------------------------- | ----------------------------------- |
| `320 000 001` – `399 999 999`   | Northern Irish Health & Care system |
| `010 100 0000` – `311 299 9999` | CHI numbers in Scotland             |

## Testable range

Reserved for **test and demo data only** — valid by the checksum algorithm,
but guaranteed to never be issued to a real patient:

| Range                           | Use                                 |
| ------------------------------- | ----------------------------------- |
| `999 000 0000` – `999 999 9999` | Testing, demos, fixtures, seed data |

The crate exposes this range via three `LazyLock` statics in the
[`testable`](../../src/testable.rs) module:

```rust
use nhs_number::testable::{TESTABLE_MIN, TESTABLE_MAX, TESTABLE_RANGE_INCLUSIVE};

assert_eq!(TESTABLE_MIN.to_string(), "999 000 0000");
assert_eq!(TESTABLE_MAX.to_string(), "999 999 9999");
```

…and a helper that returns a random sample from within it:

```rust
let sample = nhs_number::testable_random_sample();
assert!(nhs_number::testable::TESTABLE_RANGE_INCLUSIVE.contains(&sample));
```

## Guidance for code and test data

- **Never** hard-code a real NHS Number in source, tests, commit messages, or
  issue trackers — even a made-up ten-digit number could collide with a real
  patient's.
- **Always** pick fixtures from the testable range `999 000 0000` –
  `999 999 9999`.
- The two canonical public examples — `987 654 4321` (NHS syntax example) and
  `943 476 5919` (Wikipedia checksum example) — are safe to use in
  documentation because they are already public.
