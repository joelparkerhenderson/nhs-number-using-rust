# Testing

How tests are organised, what they cover, and what to add when you introduce
a new public API.

## Two layers

1. **Unit tests** live alongside the code they test, inside a
   `#[cfg(test)] mod tests` block at the bottom of the same file.
2. **Doc-tests** live in the `///` rustdoc comments above public items.

Both run under `cargo test`. There is intentionally no `tests/` integration
directory — the examples under [`examples/`](../examples/) play that role
because each one is a complete program that exercises the public API.

### Sub-module organisation in `src/lib.rs::tests`

The `mod tests` block in `src/lib.rs` is split into named sub-modules so
each test's purpose is obvious from its path:

| Sub-module   | Holds                                                            |
| ------------ | ---------------------------------------------------------------- |
| `structure`  | Inherent methods on `NHSNumber` (`new`, `check_digit`, …).       |
| `utilities`  | Free-function counterparts (`format`, `check_digit`, …).         |
| `properties` | Invariants over many inputs (round-trip, method ↔ free-fn).      |
| `boundaries` | Explicit coverage of the `sum % 11 ∈ {0, 1, 2..=10}` branches.   |
| `ordering`   | `Ord`/`Eq`, `Vec::sort`, `BTreeSet`, `BTreeMap` use cases.       |
| `traits`     | Trait-impl smoke tests (`Copy`, `Clone`, `Send`, `Sync`, serde). |
| `adversarial`| Hostile digit arrays that bypass the parser; totality (§6.4).    |
| `serialisation` | Exact serde wire shape (R16) and untrusted-payload behaviour. |
| `concurrency`| Multithreaded use of parsing, statics, and the random sampler.   |
| `ranges`     | The issuable-range predicate boundaries (§7.5).                  |
| `fuzz`       | Property-based tests (`proptest`) over generated digit arrays.   |

The parser has its own adversarial rejection tests and a property-based
`fuzz` sub-module in `src/from_str.rs::tests`: the parser must never
panic on arbitrary strings, must reject every corrupted canonical form,
and anything it accepts must be exactly one of the two documented
shapes. Property tests use the `proptest` dev-dependency and run a few
hundred generated cases per property under plain `cargo test` — no
nightly toolchain or separate fuzz harness required.

New tests should land in the sub-module that matches their concern; do
not invent a parallel layout. If a genuinely new concern appears (e.g.
benchmarks, fuzz-target shims), add a sub-module here and reference it
from `spec/13-testing-strategy.md` §13.1.

## The actual / expect pattern

Every assertion follows the same skeleton:

```rust
let actual = ...;
let expect = ...;
assert_eq!(actual, expect);
```

Reasons:

- `actual` and `expect` make the role of each side obvious without re-reading
  the assertion macro arguments.
- `assert_eq!` error messages then read naturally: `assertion `actual ==
  expect` failed`.
- The pattern survives refactoring: rename one side, the other is unaffected.

## When you add a new public API

You must add **both**:

1. A **unit test** in the appropriate `#[cfg(test)] mod tests` block.
2. A **doc-test** in the `///` comment above the item.

The doc-test serves two purposes — it tests the example *and* it ensures the
example in the rendered docs compiles.

## What to test

- Happy path: typical input, expected output.
- Edge cases: `[0; 10]`, `[9; 10]`, every boundary of the testable range.
- Error cases for parsers: wrong length, wrong separator, doubled spaces,
  leading/trailing whitespace, non-digit characters. See
  [`src/from_str.rs`](../src/from_str.rs) for the existing pattern.
- Round-trips: `parse → display → parse` must recover the original value.
- Equivalence: every free function must match its `NHSNumber` method
  counterpart on identical input.
- Adversarial input: NUL and control bytes, bidi overrides, lookalike and
  non-ASCII digits, oversized payloads, out-of-range digit arrays that
  bypass the parser (via `new` or serde). Nothing may panic; the parser
  must reject; the check-digit functions must stay total (spec §6.4).

## Examples are tests too

Every file under [`examples/`](../examples/) ends with `assert_eq!` /
`assert!` checks (or, when output is the point, an explicit "ok" print plus
asserts on the accumulated state). A successful run leaves a clear trace.

Running examples is part of release sign-off (see
[`release.md`](release.md)).

## Running the suite

```sh
cargo test                        # All unit and doc-tests.
cargo test -- --nocapture         # Show `println!` from inside tests.
cargo test --doc                  # Just the doc-tests.
cargo test <pattern>              # Only tests whose name matches.

cargo run --example basic_usage   # Run a single example.
cargo build --examples            # Compile every example without running.
```

If you add an example, it must compile and run end-to-end before the change
lands.

## What is *not* tested here

- Real patient data — never. See [`safety.md`](safety.md).
- Compliance against external NHS systems — the crate models the format and
  algorithm; integration testing is the deploying organisation's job.

## Performance tests

There is no benchmark suite. The hot paths are trivially fast (ten `i8`
multiplies); add one only if a real workload demonstrates a regression.
