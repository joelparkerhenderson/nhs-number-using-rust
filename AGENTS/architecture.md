# Architecture

Where things live in this crate and how they fit together.

For the formal public API contract, read [`spec.md`](../spec.md) and
[`docs/api/index.md`](../docs/api/index.md). This file is the **map**, not
the contract.

## Repository layout

```
.
├── AGENTS.md              # Agent entry point
├── AGENTS/                # Topical agent guides (this directory)
├── CITATION.cff           # Citation metadata
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── Cargo.lock
├── Cargo.toml
├── README.md -> index.md  # Symlink
├── cspell.json            # Spell-check dictionary
├── docs/                  # Long-form reference documentation
│   ├── api/index.md
│   ├── checksum/index.md
│   ├── faq/index.md
│   ├── ranges/index.md
│   └── usage/index.md
├── examples/              # Runnable `cargo run --example <name>` programs
│   ├── README.md
│   ├── basic_usage.rs
│   ├── bulk_processing.rs
│   ├── generate_valid.rs
│   ├── parsing.rs
│   ├── sorting.rs
│   ├── testable.rs
│   └── validation.rs
├── help/
│   └── releasing/         # Release checklist (mirrors AGENTS/release.md)
├── index.md               # Crate-level README
├── llms.json              # Machine-readable crate docs (generated)
├── llms.txt               # LLM-friendly crate docs (generated)
├── spec.md                # Living specification (spec-driven development)
└── src/
    ├── lib.rs             # Crate root: `NHSNumber` struct, free functions
    ├── from_str.rs        # `FromStr` parser
    ├── parse_error.rs     # `ParseError` type
    └── testable.rs        # Testable range constants and sampler
```

## Module map

| Module                      | Owns                                                              |
| --------------------------- | ----------------------------------------------------------------- |
| `nhs_number` (`lib.rs`)     | `NHSNumber` struct, `Display`/`Into<String>`, free functions.     |
| `nhs_number::from_str`      | `FromStr` impl (the only parser).                                 |
| `nhs_number::parse_error`   | The unit struct `ParseError`.                                     |
| `nhs_number::testable`      | `TESTABLE_MIN`, `TESTABLE_MAX`, `TESTABLE_RANGE_INCLUSIVE`, sampler. |

The `testable` module is re-exported at the crate root
(`pub use testable::*;`), so callers can write `nhs_number::testable_random_sample()`
without spelling out the module path.

## Data model

```rust
pub struct NHSNumber {
    pub digits: [i8; 10],
}
```

- A single decimal digit (`0..=9`) per element, stored as `i8`.
- Always exactly ten digits — enforced at the type level.
- The `digits` field is `pub`, so callers may construct with struct-literal
  syntax as well as via `NHSNumber::new`.
- Canonical display: `"DDD DDD DDDD"` (three, three, four, single spaces).

## Public surface (at a glance)

Methods on `NHSNumber`:

- `NHSNumber::new(digits: [i8; 10]) -> NHSNumber`
- `NHSNumber::check_digit(&self) -> i8`
- `NHSNumber::calculate_check_digit(&self) -> i8`
- `NHSNumber::validate_check_digit(&self) -> bool`
- `NHSNumber::testable_random_sample() -> NHSNumber`

Trait implementations:

- `Display`, `Into<String>` — format as `"DDD DDD DDDD"`.
- `FromStr` — parse `"DDDDDDDDDD"` or `"DDD DDD DDDD"`; errors as `ParseError`.
- `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`,
  `Serialize`, `Deserialize`.

Free functions on `[i8; 10]` (equivalent to the methods above):

- `format(digits) -> String`
- `check_digit(digits) -> i8`
- `calculate_check_digit(digits) -> i8`
- `validate_check_digit(digits) -> bool`
- `testable_random_sample() -> NHSNumber`

Constants in `testable`:

- `TESTABLE_MIN` — `999 000 0000`
- `TESTABLE_MAX` — `999 999 9999`
- `TESTABLE_RANGE_INCLUSIVE`

## Why two equivalent APIs?

Some callers already have a `[i8; 10]` and do not want to construct an
`NHSNumber`; others already have an `NHSNumber` and prefer method syntax. The
two forms always agree, and each method delegates to the matching free
function. See [`docs/faq/index.md`](../docs/faq/index.md) for the rationale.

## Dependencies

| Crate   | Purpose                                                  |
| ------- | -------------------------------------------------------- |
| `rand`  | Random sampling for `testable_random_sample`.            |
| `serde` | Derived `Serialize`/`Deserialize` (default struct shape). |

`rustdoc-llms` is a binary used during release to regenerate `llms.txt` and
`llms.json`; it is intentionally **not** a dev-dependency (it has no lib
target). See [`release.md`](release.md).
