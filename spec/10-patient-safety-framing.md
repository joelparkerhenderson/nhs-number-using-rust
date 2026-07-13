[`nhs-number` specification](index.md) — section 10 of 19. Section numbers (§10.x) are stable and cited from code, tests, and commit messages.

# 10. Patient-safety framing

This spec is a behavioural contract for a Rust library. It is not a clinical
guarantee. In particular:

- A passing `validate_check_digit` confirms only the digits' arithmetic
  self-consistency, **not** that the number identifies a real patient.
- Numbers in the testable range are syntactically valid but are not — and
  must never be — used to represent a real patient.
- Real patient matching requires far more than this crate provides
  (demographic checks, registry lookup, governance, audit).

See [`AGENTS/safety.md`](../AGENTS/safety.md) for the constraints that bind
contributors and AI agents working in this repo.
