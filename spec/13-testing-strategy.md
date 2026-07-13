[`nhs-number` specification](index.md) — section 13 of 19. Section numbers (§13.x) are stable and cited from code, tests, and commit messages.

# 13. Testing strategy

The spec is enforced through three layers:

1. **Unit tests** in `#[cfg(test)] mod tests` blocks alongside each module.
2. **Doc-tests** in `///` comments on every public item.
3. **Examples** under [`examples/`](../examples/) — each is a complete program
   that ends in `assert!` / `assert_eq!` checks.

Every assertion follows the `actual = …; expect = …; assert_eq!(actual,
expect);` pattern. New public items add **both** a unit test and a
doc-test. See [`AGENTS/testing.md`](../AGENTS/testing.md).

### 13.1 Coverage targets

Every clause of this spec must map to at least one executable test:

| Spec clause                          | Tested in                                                                            |
| ------------------------------------ | ------------------------------------------------------------------------------------ |
| §3.1 invariants                      | `src/lib.rs::tests::structure::test_new_preserves_digits`                            |
| §3.2 struct-literal construction     | `src/lib.rs::tests::structure::test_struct_literal_construction`                     |
| §3.3 derived traits                  | `src/lib.rs::tests::traits::*`                                                       |
| §4.1 method ↔ §4.2 free-fn agreement | `src/lib.rs::tests::properties::method_and_free_fn_*_agree`                          |
| §4.5 ParseError shape                | `src/parse_error.rs::tests::*`                                                       |
| §5.1 canonical display               | `src/lib.rs::tests::structure::test_display_*`                                       |
| §5.1 `From`/`Into` agree with Display [R18] | `src/lib.rs::tests::structure::test_string_from`, `test_string_from_agrees_with_display_and_into` |
| §5.2 accepted input forms            | `src/from_str.rs::tests::test_from_str_with_length_10_*` and `_12_*`                 |
| §5.3 rejected input forms            | `src/from_str.rs::tests::test_from_str_with_*` (rejection)                           |
| §5.3 adversarial / hostile inputs    | `src/from_str.rs::tests` (NUL, control, bidi, huge, lookalike cases) and `::fuzz`     |
| §5.4 round-trip property             | `src/lib.rs::tests::properties::round_trip_via_canonical_form` / `..._tight_form`    |
| §5.5 normalisation policy            | `src/from_str.rs::tests::test_from_str_with_nbsp_separators` etc.                    |
| §6.1 algorithm                       | `src/lib.rs::tests::structure::test_calculate_check_digit`                           |
| §6.1.1 sentinel `10`                 | `src/lib.rs::tests::structure::test_calculate_check_digit`                           |
| §6.2 worked example (raw ∈ 1..=9)    | `src/lib.rs::tests::structure::test_calculate_check_digit` (`943 476 5919` case)     |
| §6.2.1 worked example (raw == 11)    | `src/lib.rs::tests::boundaries::sum_mod_11_eq_0_yields_check_digit_zero`             |
| §6.2.2 worked example (raw == 10)    | `src/lib.rs::tests::boundaries::sum_mod_11_eq_1_yields_sentinel_ten`                 |
| §6.3 error-detection properties      | `src/lib.rs::tests::fuzz::single_digit_errors_are_detected`, `adjacent_transpositions_are_detected` |
| §6.4 totality on any `[i8; 10]`      | `src/lib.rs::tests::adversarial::*`, `src/lib.rs::tests::fuzz::check_digit_functions_are_total_on_any_i8` |
| §6.5 `sum % 11 ∈ 2..=10` branch      | `src/lib.rs::tests::boundaries::sum_mod_11_in_2_to_10_yields_eleven_minus_remainder` |
| §7.3 testable bounds (exact)         | `src/testable.rs::tests::test_min_exact_value`, `test_max_exact_value`               |
| §7.3 testable range `.contains`      | `src/testable.rs::tests::test_range_inclusive_*`                                     |
| §8.1 random sample range             | `src/testable.rs::tests::test_random_sample_in_range`                                |
| §8.1 first three digits == 9 [R14]   | `src/testable.rs::tests::test_random_sample_first_three_digits_are_999`              |
| §8.1 non-determinism                 | `src/testable.rs::tests::test_random_sample_is_non_deterministic`                    |
| §9 ordering & collection use         | `src/lib.rs::tests::ordering::*`                                                     |
| §11.1 serde default shape [R16]      | `src/lib.rs::tests::serialisation::serialize_json_shape_is_digits_array` etc.        |
| §11.1 digit-range validation [R20]   | `src/lib.rs::tests::serialisation::deserialize_rejects_out_of_range_digits`, `deserialize_error_does_not_echo_payload`, `deserialize_accepts_every_in_range_digit_value` |
| §11.1 serde untrusted payloads       | `src/lib.rs::tests::serialisation::deserialize_rejects_*`                             |
| §11.2 string-form wrapper [R22]      | `src/serde_string.rs::tests::*`                                                       |
| §3.3 `Hash` [R19]                    | `src/lib.rs::tests::traits::hash_is_consistent_with_eq`, `hashmap_use_as_key`         |
| §3.3 `Send`/`Sync` under contention  | `src/lib.rs::tests::concurrency::*`                                                  |
| §4.5 `ParseError` `Display`/`Error` [R21] | `src/parse_error.rs::tests::test_parse_error_display_is_fixed_message` etc.     |
| §7.5 issuable-range predicate [R23]  | `src/lib.rs::tests::ranges::*`                                                        |
| §12 `ParseError` zero-sized          | `src/parse_error.rs::tests::test_parse_error_is_zero_sized`                          |

Whenever a clause is added, extend this table in the same PR. Any row
whose right-hand side is empty is a §17 task by definition.

### 13.2 Test sub-module layout (in `src/lib.rs::tests`)

The unit tests are organised by intent so each test's purpose is obvious
from its path:

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

The parser has its own property-based suite in
`src/from_str.rs::tests::fuzz` (never-panics, accepted-shape, and
corruption-rejection properties over generated strings).

New tests should land in the sub-module that matches their concern; do
not invent a parallel layout. If a genuinely new concern appears (e.g.
benchmarks, fuzz-target shims), add a sub-module here and reference it
from this table and from §13.1.

### 13.3 Canonical test fixtures

The following `NHSNumber` values are the canonical fixtures used across
unit tests, doc-tests, and examples. They cover boundary digits, the two
Wikipedia reference numbers, and the testable-range extremes. Adding to
this list is fine; removing or changing a value requires updating every
call site.

| Digits                          | String form    | Why                                              |
| ------------------------------- | -------------- | ------------------------------------------------ |
| `[0; 10]`                       | `000 000 0000` | All-zero edge case; `sum == 0`, raw `== 11`.     |
| `[9; 10]`                       | `999 999 9999` | All-nine edge case; `TESTABLE_MAX`; raw `== 9`.  |
| `[0,1,2,3,4,5,6,7,8,9]`         | `012 345 6789` | Mixed digits; stored check digit 9.              |
| `[9,4,3,4,7,6,5,9,1,9]`         | `943 476 5919` | Wikipedia checksum example; raw `∈ 1..=9`.       |
| `[9,8,7,6,5,4,4,3,2,1]`         | `987 654 4321` | Wikipedia format example.                        |
| `[9,9,9,0,0,0,0,0,0,0]`         | `999 000 0000` | `TESTABLE_MIN`.                                  |
| `[9,9,9,1,0,0,0,0,0,3]`         | `999 100 0003` | Typical testable, valid by checksum.             |
| `[9,9,9,1,0,0,0,1,0,0]`         | `999 100 0100` | Boundary: `sum % 11 == 0` → check digit 0.       |
| `[9,9,9,1,2,3,4,5,6,0]`         | `999 123 4560` | Boundary: `sum % 11 == 1` → sentinel `10`.       |

Tests that need a non-fixture value (e.g. a random sample) should still
assert a property, not an exact value.

### 13.4 What is not tested here

- Real patient data — never. See [`AGENTS/safety.md`](../AGENTS/safety.md).
- Compliance against external NHS systems — the crate models the format
  and algorithm; integration testing is the deploying organisation's job.
- Performance — there is no benchmark suite. The hot paths are ten `i8`
  multiplies; add a bench only if a real workload demonstrates a
  regression.

Property-based / fuzz testing **is** used, via the `proptest`
dev-dependency: see `src/lib.rs::tests::fuzz` and
`src/from_str.rs::tests::fuzz`. These run a few hundred generated cases
per property under plain `cargo test`; there is no separate
nightly-only `cargo fuzz` harness.
