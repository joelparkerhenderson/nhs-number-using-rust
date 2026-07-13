# Workflows

Common day-to-day commands when working on this crate.

For release-specific commands, see [`release.md`](release.md).
For test-specific commands, see [`testing.md`](testing.md).

## Build and test

```sh
cargo build                       # Debug build.
cargo build --release             # Optimised build.
cargo test                        # Unit + doc-tests.
cargo test -- --nocapture         # Show println! from tests.
cargo clippy --all-targets        # Lints (errors fail CI).
cargo fmt                         # Format.
cargo fmt -- --check              # Verify formatting without changing files.
```

## Documentation

```sh
cargo doc --no-deps --open        # Build and open this crate's docs only.
cargo doc --document-private-items  # Include private items in the rendered docs.
```

The rendered docs mirror what `docs.rs` will publish; if it looks wrong
locally, it will look wrong on docs.rs too.

## Examples

Every example is a standalone `cargo run --example <name>` program. List
them with:

```sh
cargo run --example                # Lists examples, then exits.
```

| Example         | What it shows                                                   |
| --------------- | --------------------------------------------------------------- |
| basic_usage     | Construction, display, and the free-function form.              |
| parsing         | `FromStr` across both accepted formats, plus rejection cases.   |
| validation      | Reading, calculating, and validating the check digit.           |
| testable        | Working with the `999 000 0000 – 999 999 9999` range.           |
| sorting         | `Ord` / `Eq` driving `Vec::sort` and `BTreeSet`.                |
| generate_valid  | Building a valid testable number from nine seed digits.         |
| bulk_processing | Three-way classification (valid / bad checksum / unparseable).  |

Run a single example:

```sh
cargo run --example basic_usage
cargo run --release --example bulk_processing
```

Compile every example without running any:

```sh
cargo build --examples
```

## Common loops while making a change

**Single change, single file:**

```sh
cargo test && cargo clippy --all-targets && cargo fmt -- --check
```

**Touching the parser or validator (behavioural change):**

1. Update [`spec/index.md`](../spec/index.md) to describe the new behaviour.
2. Update tests to encode the new behaviour.
3. Update the implementation.
4. `cargo test` — should be green.
5. `cargo run --example bulk_processing` and `cargo run --example validation`
   — visually confirm behaviour at the boundary.
6. `cargo doc --no-deps` — confirm doc-tests still pass.

**Adding a new public API:**

1. Update [`spec/index.md`](../spec/index.md) to add the entry to the public surface.
2. Add the implementation.
3. Add a unit test and a doc-test (see [`testing.md`](testing.md)).
4. Add or update an entry in [`docs/api/index.md`](../docs/api/index.md).
5. If a new ergonomic story is worth showing, add an example in
   [`examples/`](../examples/) and reference it from the example README.

## Git etiquette

- One topic per commit; subject in the imperative ("Add", "Fix", "Update").
- Keep the commit body terse — the spec and the diff explain the **what**.
  Use the body for the **why**.
- Do not commit `target/` or any output from `rustdoc-llms`. The `llms.txt`
  / `llms.json` files at the repo root are regenerated at release time only
  (see [`release.md`](release.md)).
