# Examples

Runnable programs demonstrating the `nhs-number` crate. Each example is a
single file, with no setup, invoked via `cargo run --example <name>`.

| Example            | Purpose                                                               |
|--------------------|-----------------------------------------------------------------------|
| [basic_usage](basic_usage.rs)       | Construction, display, and the free-function form.  |
| [parsing](parsing.rs)               | `FromStr` across the two accepted formats; error cases. |
| [validation](validation.rs)         | Check-digit reading, calculation, and validation.   |
| [testable](testable.rs)             | Working with the 999 000 0000 – 999 999 9999 range. |
| [sorting](sorting.rs)               | `Ord`/`Eq` in `Vec::sort`, `BTreeSet`, and `BTreeMap` keys. |
| [generate_valid](generate_valid.rs) | Build a valid testable number from nine seed digits. |
| [bulk_processing](bulk_processing.rs) | Parse + classify a batch of candidate inputs.     |

## Running

```sh
# Build them all, don't run.
cargo build --examples

# Run one.
cargo run --example basic_usage

# Run with release optimisations.
cargo run --release --example bulk_processing
```

## Guarantees

- Every example uses only the published public API of the crate — no
  `pub(crate)` items, no internals.
- Every NHS Number hard-coded into an example is either from the testable
  range (`999 000 0000` – `999 999 9999`) or one of the public reference
  numbers from the
  [Wikipedia article](https://en.wikipedia.org/wiki/NHS_number)
  (`943 476 5919`, `987 654 4321`). No example uses a number that could
  collide with a real patient record.
- Every example ends with either a full set of `assert_eq!` / `assert!`
  checks or prints its result, so a successful run is easy to verify.

See also: [../docs/usage/index.md](../docs/usage/index.md) for a
tutorial-shape walk-through, and [../docs/api/index.md](../docs/api/index.md)
for the full public surface.
