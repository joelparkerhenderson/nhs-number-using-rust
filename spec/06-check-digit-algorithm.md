[`nhs-number` specification](index.md) — section 6 of 19. Section numbers (§6.x) are stable and cited from code, tests, and commit messages.

# 6. Check-digit algorithm

### 6.1 Definition (the canonical spec) [R9]

Given `digits[0..=8]`, the check digit is computed as follows:

1. **Weight and sum.** Multiply each of the first nine digits by
   `weight = 10 − i` (so `digits[0]` × 10, `digits[1]` × 9, …,
   `digits[8]` × 2). Sum the nine products.

2. **Take modulo 11.** Compute `remainder = sum mod 11`.
   `remainder ∈ {0, 1, 2, …, 10}`.

3. **Subtract from 11.** Compute `raw = 11 − remainder`.
   `raw ∈ {1, 2, …, 11}`.

4. **Map to the check digit:**
   - `raw == 11` → check digit is `0`.
   - `raw == 10` → the number is **invalid**; no digit in `0..=9` can stand
     in for the check digit, so such a number is not issued.
   - otherwise → check digit is `raw` (i.e. one of `1..=9`).

### 6.1.1 Sentinel for the "invalid" case [R10, R11]

`calculate_check_digit` must return an `i8` for every input, so the spec
reserves a sentinel value: when `raw == 10`, the function returns `10`.
Because every stored tenth digit is in `0..=9`, the sentinel can never
equal `check_digit()`, so `validate_check_digit` correctly returns `false`
for every number whose weighted sum is congruent to `1 (mod 11)`.

### 6.2 Worked example: `943 476 5919`

`digits = [9, 4, 3, 4, 7, 6, 5, 9, 1, 9]`

| i       | d[i] | weight | product |
| ------- | ---- | ------ | ------- |
| 0       | 9    | 10     | 90      |
| 1       | 4    | 9      | 36      |
| 2       | 3    | 8      | 24      |
| 3       | 4    | 7      | 28      |
| 4       | 7    | 6      | 42      |
| 5       | 6    | 5      | 30      |
| 6       | 5    | 4      | 20      |
| 7       | 9    | 3      | 27      |
| 8       | 1    | 2      | 2       |
| **sum** |      |        | **299** |

- `299 mod 11 = 2`
- `11 − 2 = 9`
- Check digit: `9`. Stored tenth digit: `9`. ✓ — branch `raw ∈ 1..=9`.

### 6.2.1 Worked example: `999 100 0100` (raw == 11)

`digits = [9, 9, 9, 1, 0, 0, 0, 1, 0, 0]`

- weighted sum: 90 + 81 + 72 + 7 + 0 + 0 + 0 + 3 + 0 = **253**.
- `253 mod 11 = 0`, `raw = 11 − 0 = 11`.
- Mapped to check digit: `0`.
- Stored tenth digit: `0`. ✓ — branch `raw == 11`.

### 6.2.2 Worked example: `999 123 4560` (raw == 10, sentinel)

`digits = [9, 9, 9, 1, 2, 3, 4, 5, 6, 0]`

- weighted sum: 90 + 81 + 72 + 7 + 12 + 15 + 16 + 15 + 12 = **320**.
- `320 mod 11 = 1`, `raw = 11 − 1 = 10`.
- No digit in `0..=9` fits, so `calculate_check_digit` returns the
  sentinel `10`.
- `validate_check_digit` returns `false` for every stored tenth digit
  `0..=9` because none of them can equal `10`.

### 6.3 What the check digit catches (and what it does not)

Detects:

- Every single-digit error.
- Every transposition of two adjacent digits.
- Most other common data-entry mistakes.

Does **not** detect:

- An attacker deliberately choosing digits that produce a valid checksum.
- The (rare) error patterns that happen to leave the checksum invariant.
- Whether a given number has been issued to a real patient — this is a
  syntactic check, not a registry lookup. See §10 for the patient-safety
  framing.

### 6.4 Behaviour on out-of-range digits

`calculate_check_digit` does no bounds checking on its `[i8; 10]` input.
If any digit is outside `0..=9`, the numerical result is mathematically
meaningless at the spec level.

The functions are nevertheless **total**: `calculate_check_digit`,
`validate_check_digit`, and `check_digit` never panic, and
`calculate_check_digit` always returns a value in `0..=10`, for **every**
possible `[i8; 10]` — including hostile values reachable via
`NHSNumber::new`, a struct literal, or serde deserialisation (§18.5).
The implementation widens digits to `i64` and uses a Euclidean remainder,
so no overflow or negative-modulo path exists. Totality is enforced by
`src/lib.rs::tests::adversarial` and
`src/lib.rs::tests::fuzz::check_digit_functions_are_total_on_any_i8`.

The `FromStr` parser remains the only supported entry point for
caller-supplied data — it enforces `0..=9` per digit.

### 6.5 Conformance summary

The implementation must agree with §6.1 / §6.1.1 across every input. In
particular, the three boundary cases are:

| `sum % 11` | `raw` | Result                                    |
| ---------- | ----- | ----------------------------------------- |
| 0          | 11    | check digit `0`                           |
| 1          | 10    | sentinel `10` — invalid, no digit fits    |
| 2..=10     | 9..=1 | `11 − remainder` (check digit in `1..=9`) |

Each case has a dedicated unit test in `src/lib.rs::tests::boundaries`.
