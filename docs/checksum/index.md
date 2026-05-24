# Check-Digit Algorithm

The tenth and final digit of an NHS Number is a modulo-11 checksum over the
first nine digits. The checksum lets software detect most single-digit
transcription errors and every swap of two adjacent digits.

## Reference

- [NHS Number — Wikipedia](https://en.wikipedia.org/wiki/NHS_number)

## Algorithm

Given digits `d[0..9]` (indexed from 0), the check digit is computed as:

1. Multiply each of the first nine digits by `10 − i`, where `i` is its index.
   (So `d[0]` is multiplied by 10, `d[1]` by 9, …, `d[8]` by 2.)
2. Sum the nine products.
3. Take the sum modulo 11.
4. Subtract the remainder from 11 — this is the raw check digit in the range
   `[1, 11]`.
5. If the raw check digit is 11, record it as 0.
6. If the raw check digit is 10, the NHS Number is **not valid**; no digit
   from 0–9 can stand in.

Expressed in pseudocode:

```text
sum       = Σ d[i] × (10 − i)   for i in 0..=8
remainder = sum mod 11
raw       = 11 − remainder      // in [1, 11]
check     = match raw:
            11 → 0
            10 → INVALID
             n → n
```

## Worked example: `943 476 5919`

`digits = [9, 4, 3, 4, 7, 6, 5, 9, 1, 9]`

| i       | d[i] | weight (10 − i) | product |
| ------- | ---- | --------------- | ------- |
| 0       | 9    | 10              | 90      |
| 1       | 4    | 9               | 36      |
| 2       | 3    | 8               | 24      |
| 3       | 4    | 7               | 28      |
| 4       | 7    | 6               | 42      |
| 5       | 6    | 5               | 30      |
| 6       | 5    | 4               | 20      |
| 7       | 9    | 3               | 27      |
| 8       | 1    | 2               | 2       |
| **sum** |      |                 | **299** |

- `299 mod 11 = 2`
- `11 − 2 = 9`
- Expected check digit: `9`.
- Actual tenth digit of `943 476 5919`: `9`. ✓

## Worked example: `999 100 0003`

`digits = [9, 9, 9, 1, 0, 0, 0, 0, 0, 3]`

| i       | d[i] | weight | product |
| ------- | ---- | ------ | ------- |
| 0       | 9    | 10     | 90      |
| 1       | 9    | 9      | 81      |
| 2       | 9    | 8      | 72      |
| 3       | 1    | 7      | 7       |
| 4       | 0    | 6      | 0       |
| 5       | 0    | 5      | 0       |
| 6       | 0    | 4      | 0       |
| 7       | 0    | 3      | 0       |
| 8       | 0    | 2      | 0       |
| **sum** |      |        | **250** |

- `250 mod 11 = 8`
- `11 − 8 = 3`
- Expected check digit: `3`.
- Actual tenth digit of `999 100 0003`: `3`. ✓

## Worked example: `999 123 4560` (invalid)

`digits = [9, 9, 9, 1, 2, 3, 4, 5, 6, 0]`

`sum = 320`, `320 mod 11 = 1`, raw `= 11 − 1 = 10`.

A raw value of 10 means **no digit in `0..=9` can stand in** as the check
digit, so any ten-digit string with these first nine digits is invalid by
the NHS specification regardless of the stored tenth digit.

This crate signals that case by returning the sentinel value `10` from
`calculate_check_digit`. Because every stored digit is in `0..=9`, the
sentinel never matches, and `validate_check_digit` correctly returns
`false`.

## In this crate

Three entry points, all equivalent:

```rust
use nhs_number::NHSNumber;

let n = NHSNumber::new([9, 9, 9, 1, 0, 0, 0, 0, 0, 3]);

n.check_digit();            // 3 — reads the stored tenth digit
n.calculate_check_digit();  // 3 — computes from digits[0..9]
n.validate_check_digit();   // true — the two agree

// Or, without building an `NHSNumber`:
let d = [9, 9, 9, 1, 0, 0, 0, 0, 0, 3];
nhs_number::check_digit(d);
nhs_number::calculate_check_digit(d);
nhs_number::validate_check_digit(d);

// The "no digit fits" case:
let bad = [9, 9, 9, 1, 2, 3, 4, 5, 6, 0];
assert_eq!(nhs_number::calculate_check_digit(bad), 10); // sentinel
assert!(!nhs_number::validate_check_digit(bad));
```

## What this catches

The modulo-11 check digit catches:

- Any single-digit error (≈ 100% detection).
- Any transposition of two adjacent digits (≈ 100% detection).
- Most other common data-entry mistakes.

What it does **not** catch:

- An attacker deliberately choosing digits that produce a valid checksum.
- Errors that happen to leave the checksum invariant (rare in practice).

For safety-critical patient matching, the check digit is a first gate, not the
whole story — always combine it with other identity verification.
