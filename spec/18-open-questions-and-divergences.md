[`nhs-number` specification](index.md) — section 18 of 19. Section numbers (§18.x) are stable and cited from code, tests, and commit messages.

# 18. Open questions and known divergences

This section is **not** a backlog of unrelated wishlist items (those live
in §17) — it is the list of points where the crate's behaviour, its spec,
and the published NHS specification do not fully agree, plus genuinely
unresolved design questions.

### 18.1 `Hash` is not derived — **resolved in 2.0.0**

`NHSNumber` now derives `Hash` [R19], so `HashMap` / `HashSet` keys work.
Kept here (rather than deleted) because older code and docs referenced
this section number.

### 18.2 The default serde shape

The default `{ "digits": [...] }` shape exposes a Rust struct field name
on the wire. Some callers will prefer the string form `"999 100 0003"` as
the default. Switching the default would be a (further) major release.
Since 2.0.0 the opt-in newtype `serde_string::NHSNumberString` [R22]
covers the string-form use case without changing the default.

### 18.3 Range checks are not enforced at construction

`NHSNumber::new` and `FromStr` accept ten-digit sequences outside the
issued/testable ranges (§7). This is deliberate — the crate parses
*format*, not *provenance*. Since 2.0.0 callers can ask the question
explicitly with `is_issuable_range` (§7.5, R23) and
`TESTABLE_RANGE_INCLUSIVE`; neither is enforced during construction or
parsing, and that remains intentional.

### 18.4 MSRV vs. edition 2024 features

The crate uses `std::sync::LazyLock` and the 2024 edition, which sets an
effective MSRV of approximately Rust 1.80. We have not committed to a
narrower MSRV than "current stable"; downstream projects with stricter
MSRV requirements would either need a back-ported variant or a pinned
older version of this crate. Whether to publish a formal MSRV policy is
open.

### 18.5 Out-of-range digits via `new` / struct literals

**Partially resolved in 2.0.0:** `Deserialize` now validates the digit
range [R20], so untrusted *serialised* payloads can no longer construct
out-of-range digits. What remains — deliberately, per R2 — is the
in-process path: `NHSNumber::new` and struct-literal construction still
accept any `[i8; 10]`. Consequences, all covered by tests:

- The check-digit functions remain total on such values (§6.4) — they
  never panic and `calculate_check_digit` stays in `0..=10`.
- One semantic corner exists: a stored tenth digit of `10` (impossible
  via the parser or serde) compares equal to the sentinel `10`, so
  `validate_check_digit` returns `true` for a number that has **no**
  valid check digit. See
  `src/lib.rs::tests::adversarial::validate_rejects_hostile_stored_check_digit`.
- `Display`/`format` on out-of-range digits produces a non-canonical
  string (e.g. `-1` renders as two characters), breaking R4's
  twelve-character guarantee for such values.
- `is_issuable_range` returns `false` for any out-of-domain digit (R23).

Closing the remaining path would mean removing the `pub` field or
making `new` fallible — both breaking (R2) and currently non-goals.
