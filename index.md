# NHS Number

**[documentation](https://docs.rs/nhs-number/)**
•
**[source](https://github.com/joelparkerhenderson/nhs-number)**
•
**[crate](https://crates.io/crates/nhs-number)**
•
**[spec](spec.md)**
•
**[llms.txt](https://raw.githubusercontent.com/joelparkerhenderson/nhs-number/refs/heads/main/llms.txt)**
•
**[email](mailto:joel@joelparkerhenderson.com)**

A National Health Service (NHS) Number is a unique identifier allocated in a
shared numbering scheme to registered users of the public health services in
**England** and the **Isle of Man**.

The NHS Number is the key to identifying patients — especially when
delivering safe care across provider organisations — and is required in all
new software deployed within NHS organisations.

References:

* [National Health Service (NHS)](https://en.wikipedia.org/wiki/National_Health_Service)

* [NHS Number](https://en.wikipedia.org/wiki/NHS_number)

## What this crate does

| Capability                          | API                                              |
| ----------------------------------- | ------------------------------------------------ |
| Wrap ten digits in a value type     | `NHSNumber { digits: [i8; 10] }`                 |
| Parse `"DDDDDDDDDD"` or `"DDD DDD DDDD"` | `NHSNumber::from_str` / `"…".parse()`        |
| Format as `"DDD DDD DDDD"`          | `Display`, `Into<String>`                        |
| Read or recompute the check digit   | `check_digit`, `calculate_check_digit`           |
| Validate the check digit            | `validate_check_digit`                           |
| Use the reserved testable range     | `TESTABLE_MIN`, `TESTABLE_MAX`, `testable_random_sample` |
| Order, equality, serialise          | `Ord`, `Eq`, `serde::{Serialize, Deserialize}`   |

For the full public surface, see [`docs/api/index.md`](docs/api/index.md).
For the canonical behavioural specification, see [`spec.md`](spec.md).

## Install

```sh
cargo add nhs-number
```

Or in `Cargo.toml`:

```toml
[dependencies]
nhs-number = "1"
```

## Syntax

The current system uses a ten-digit number in the `3 3 4` format, with the
final digit being an error-detecting checksum. An example: `943 476 5919`.

## Ranges

Currently issued ranges:

* `300 000 000` – `399 999 999` (England)

* `400 000 000` – `499 999 999` (England, Isle of Man)

* `600 000 000` – `799 999 999` (England, Isle of Man)

Unavailable (reserved for other systems):

* `320 000 001` – `399 999 999` (Northern Irish system)

* `010 100 0000` – `311 299 9999` (CHI numbers in Scotland)

Testable (valid by the checksum algorithm, never issued):

* `999 000 0000` – `999 999 9999`

For the full table and guidance on choosing test fixtures, see
[`docs/ranges/index.md`](docs/ranges/index.md).

## Checksum

The checksum is calculated by multiplying each of the first nine digits by
`11 − position`. Using `943 476 5919` as an example:

* The first digit (9) is multiplied by 10.

* The second digit (4) is multiplied by 9.

* And so on until the ninth digit (1) is multiplied by 2.

* The products are summed. Here: (9×10) + (4×9) + (3×8) + (4×7) + (7×6) +
  (6×5) + (5×4) + (9×3) + (1×2) = 299.

* The remainder when dividing by 11 yields a number in 0–10 — here, 2.

* This is subtracted from 11 to give the checksum in 1–11 — here, 9, which
  becomes the last digit of the NHS Number.

* A checksum of 11 is represented by 0 in the final NHS Number. A checksum
  of 10 means the number is **not valid**.

For two fully worked examples and the in-crate entry points, see
[`docs/checksum/index.md`](docs/checksum/index.md).

## Example

```rust
use nhs_number::NHSNumber;
use std::str::FromStr;

// A test-safe NHS Number drawn from the reserved testable range.
let input = "999 100 0003";

// Parse a string into an NHSNumber.
let nhs_number = NHSNumber::from_str(input).unwrap();

// Validate the check digit.
let is_valid: bool = nhs_number.validate_check_digit();
assert!(is_valid);

// Round-trip back to the canonical string form.
let displayed: String = nhs_number.to_string();
assert_eq!(displayed, input);
```

For a tutorial walk-through, see [`docs/usage/index.md`](docs/usage/index.md).
For runnable programs, see [`examples/`](examples/).

## Documentation map

| Where                                              | What                                      |
| -------------------------------------------------- | ----------------------------------------- |
| [`spec.md`](spec.md)                               | Canonical behavioural specification.      |
| [`docs/usage/index.md`](docs/usage/index.md)       | Tutorial-style walk-through.              |
| [`docs/api/index.md`](docs/api/index.md)           | Full public API reference.                |
| [`docs/checksum/index.md`](docs/checksum/index.md) | Check-digit algorithm with worked examples. |
| [`docs/ranges/index.md`](docs/ranges/index.md)     | Issued, reserved, and testable ranges.    |
| [`docs/faq/index.md`](docs/faq/index.md)           | Frequently asked questions.               |
| [`examples/`](examples/)                           | Runnable `cargo run --example <name>` programs. |
| [`AGENTS.md`](AGENTS.md)                           | Guidance for AI coding agents.            |

## License

Multi-licensed under MIT OR Apache-2.0 OR GPL-2.0 OR GPL-3.0 OR
BSD-3-Clause. Pick whichever fits your project.
