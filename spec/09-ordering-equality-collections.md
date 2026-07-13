[`nhs-number` specification](index.md) — section 9 of 19. Section numbers (§9.x) are stable and cited from code, tests, and commit messages.

# 9. Ordering, equality, and collection use [R13]

- `PartialEq` / `Eq`: two `NHSNumber`s are equal iff their `digits` arrays
  are equal element-wise.
- `PartialOrd` / `Ord`: lexicographic comparison on the `digits` array.
  Because all values have the same length and the most-significant digit
  comes first, this matches natural numeric ordering.
- `Clone`, `Copy`: the struct is 10 bytes; copies are essentially free.

This makes `NHSNumber` directly usable as:

- a `Vec<NHSNumber>` element with `.sort()`,
- a `BTreeSet<NHSNumber>` element,
- a `BTreeMap<NHSNumber, V>` key,
- a `HashSet<NHSNumber>` element / `HashMap<NHSNumber, V>` key
  (via the derived `Hash` [R19], consistent with `Eq`).

A runnable demonstration of `Vec::sort`, `BTreeSet`, and a
`BTreeMap<NHSNumber, _>` lives in
[`examples/sorting.rs`](../examples/sorting.rs).
