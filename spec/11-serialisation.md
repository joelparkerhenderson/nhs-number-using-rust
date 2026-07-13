[`nhs-number` specification](index.md) — section 11 of 19. Section numbers (§11.x) are stable and cited from code, tests, and commit messages.

# 11. Serialisation

### 11.1 Default `serde` shape [R16, R20]

`Serialize` is derived and uses the struct's default layout. In JSON:

```json
{ "digits": [9, 9, 9, 1, 2, 3, 4, 5, 6, 0] }
```

This shape is stable; changing it is a major-version bump.

`Deserialize` is a **hand-written impl with the same wire shape** that
additionally **validates every digit is in `0..=9`** [R20]. Arity and
integer width were always enforced by the `[i8; 10]` layout; the digit
range check closes the gap that previously let untrusted payloads
construct out-of-range digit values (the former §18.5 divergence). The
rejection message is fixed and echoes no payload data
(`AGENTS/safety.md` §3).

> **History:** through 1.0.3 the impl was derived and accepted
> out-of-range digits. Tightening it is a breaking change for callers
> who relied on that; it shipped as a major bump (§14).

### 11.2 String-form serialisation [R22]

Callers who want the human-readable `"999 100 0003"` form on the wire
wrap the value in `serde_string::NHSNumberString`:

- `Serialize` renders via `Display` — always the canonical
  twelve-character form (§5.1).
- `Deserialize` parses via `FromStr` — exactly the two accepted shapes
  (§5.2), which also guarantees the digit range.
- The newtype's inner field is `pub`; `From` conversions exist in both
  directions; `Display`/`FromStr` delegate to the wrapped value.

The **default** wire shape for `NHSNumber` itself remains the §11.1
struct layout; switching the default would be a further major bump.
