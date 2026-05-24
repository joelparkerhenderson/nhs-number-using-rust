# Conventions

How code in this crate is written. Match the existing style; do not invent
new ones.

## Formatting

- 4-space indentation, no tabs.
- Run `cargo fmt` before committing.
- One blank line between top-level items.
- Prefer explicit types on bindings when they aid readability:

  ```rust
  let actual: NHSNumber = NHSNumber::new([9, 9, 9, 1, 2, 3, 4, 5, 6, 0]);
  ```

## Types and data shape

- **Prefer `[i8; 10]` over `Vec<i8>`.** Digit arrays are always fixed-length;
  the type system should enforce that.
- Keep `NHSNumber`'s `digits` field `pub`. Struct-literal construction is a
  documented part of the API.
- Use the existing free-function ↔ method symmetry when adding new behaviour:
  if a method makes sense on `NHSNumber`, the matching free function on
  `[i8; 10]` should also exist (and vice-versa).

## Doc comments

Every public item carries a rustdoc comment, in this shape:

1. **One-sentence summary** on the first line.
2. Blank line.
3. **`Example:`** section containing a runnable doc-test.
4. Blank line.
5. **Cross-reference** to the equivalent method or function.

Example skeleton:

```rust
/// Calculate the NHS Number check digit using a checksum algorithm.
///
/// Example:
///
/// ```rust
/// let digits = [9, 9, 9, 1, 2, 3, 4, 5, 6, 0];
/// let check_digit = ::nhs_number::calculate_check_digit(digits);
/// assert_eq!(check_digit, 0);
/// ```
///
/// This function is called by the method
/// [NHSNumber::calculate_check_digit](NHSNumber::calculate_check_digit).
///
```

## `#[allow(dead_code)]`

Used on public items that are part of the exported surface but happen not to
be called from inside the crate (e.g. helper functions that callers use but
the crate itself does not). Do not use it as a blanket suppressor for
genuinely dead code — delete that instead.

## Module organisation

- One responsibility per file (`from_str.rs`, `parse_error.rs`, `testable.rs`).
- Tests live alongside the code they test in a `#[cfg(test)] mod tests` block
  at the bottom of the same file.
- Re-export submodules at the crate root only when the re-export shortens
  ergonomic call sites (e.g. `pub use testable::*;`).

## Naming

- Functions describe an action (`calculate_check_digit`,
  `validate_check_digit`).
- Statics in `testable` are `SCREAMING_SNAKE_CASE` because they are constants
  in spirit (wrapped in `LazyLock` only because `NHSNumber` is not `const`-
  constructable from a `RangeInclusive`).
- Use UK spelling in user-facing prose (`organisation`, `behaviour`) and US
  spelling in code identifiers (Rust convention). The existing codebase
  already follows this split — preserve it.

## Comments

- Default to no comments. The doc-comment style above already explains
  every public item.
- Only add an inline `//` comment when the **why** is non-obvious — a hidden
  constraint, a subtle invariant, a workaround.
- Do not narrate the **what**; well-named identifiers do that.

## Lints

- `cargo clippy --all-targets` must be clean.
- Do not silence clippy with `#[allow(…)]` without explaining why in the same
  line.

## Tests

Test conventions live in [`testing.md`](testing.md).
