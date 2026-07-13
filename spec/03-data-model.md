[`nhs-number` specification](index.md) — section 3 of 19. Section numbers (§3.x) are stable and cited from code, tests, and commit messages.

# 3. Data model

### 3.1 The `NHSNumber` struct [R1]

```rust
pub struct NHSNumber {
    pub digits: [i8; 10],
}
```

**Invariants:**

- `digits` is always exactly ten elements (enforced by the type system).
- Each element **should** be in `0..=9`. Out-of-range values are not
  rejected at construction, but the check-digit algorithm is only defined
  on this domain (§6.4). Callers handling untrusted input must parse via
  [`FromStr`](#5-string-forms-parsing-and-formatting), which enforces the
  digit range.
- `digits[0]` is the most-significant digit (leftmost in the displayed
  form).

### 3.2 Public field [R2]

The `digits` field is `pub`. Callers may construct an `NHSNumber` directly
via a struct literal:

```rust
let n = NHSNumber { digits: [9, 9, 9, 1, 2, 3, 4, 5, 6, 0] };
```

This is documented and stable; do not remove `pub` from the field.

### 3.3 Derived traits [R12]

| Trait                      | Semantics                                                   |
| -------------------------- | ----------------------------------------------------------- |
| `Debug`                    | Standard derive.                                            |
| `Clone`, `Copy`            | Cheap — the struct is 10 bytes.                             |
| `PartialEq`, `Eq`          | Digit-by-digit equality.                                    |
| `PartialOrd`, `Ord`        | Lexicographic on the digit array (matches numeric).         |
| `Hash` [R19]               | Derived; consistent with `Eq`. `HashMap`/`HashSet` keys work. |
| `Serialize`                | Serde derive with the default struct layout (`{ "digits": […] }`). |
| `Deserialize` [R20]        | Hand-written: same wire shape as the derive, **plus** digit-range validation (`0..=9`). See §11.1. |
