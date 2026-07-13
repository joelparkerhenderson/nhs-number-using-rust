[`nhs-number` specification](index.md) — section 8 of 19. Section numbers (§8.x) are stable and cited from code, tests, and commit messages.

# 8. Random sampling

### 8.1 `testable_random_sample()` contract [R14, R15]

```rust
fn testable_random_sample() -> NHSNumber
```

- The first three digits are always `9, 9, 9` — every sample lands in the
  testable range.
- The remaining seven digits are drawn uniformly at random from `0..=9`
  using `rand::rng()`.
- The tenth (check) digit is **drawn randomly**, *not* computed. ≈90% of
  samples have an invalid check digit.
- Non-deterministic: two calls return different values (with very high
  probability).

### 8.2 Why not always-valid?

Returning invalid-checksum samples is **intentional**: it lets tests
exercise the rejection branch of `validate_check_digit`. Callers who need a
valid-by-checksum random sample have two options:

1. Loop:

   ```rust
   loop {
       let n = NHSNumber::testable_random_sample();
       if n.validate_check_digit() { break n; }
   }
   ```

2. Pick the first nine digits and compute the tenth with
   `calculate_check_digit`. See
   [`examples/generate_valid.rs`](../examples/generate_valid.rs).
