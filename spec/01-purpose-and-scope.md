[`nhs-number` specification](index.md) — section 1 of 19. Section numbers (§1.x) are stable and cited from code, tests, and commit messages.

# 1. Purpose and scope

### 1.1 Purpose

Provide a small, dependable Rust value type — `NHSNumber` — that models a
National Health Service (NHS) Number across **NHS England** and
**NHS Isle of Man**, with first-class support for:

- parsing the two canonical string forms,
- formatting back to a canonical string form,
- reading and recomputing the modulo-11 check digit,
- expressing the reserved testable range as types and constants,
- ordering, equality, and `serde` serialisation.

### 1.2 In scope

- The structure, parsing, formatting, ordering, and check-digit validation
  of a ten-digit NHS Number.
- A reserved testable range (`999 000 0000` – `999 999 9999`) exposed as
  typed constants and a random sampler.
- A documented sentinel for the "no digit fits" case (raw check value
  equals 10) so the API stays infallible at the type level while still
  catching the case at runtime.

### 1.3 Out of scope (non-goals)

- Looking up whether a given NHS Number has actually been issued to a
  patient (no registry access).
- Mapping NHS Numbers to patient identities, demographics, or care records.
- Validating non-NHS-England/IoM identifiers (e.g. Scottish CHI, Northern
  Irish HCN).
- Cryptographic protection of NHS Numbers in transit or at rest.
- Localisation of the rendered string form (it is fixed: `"DDD DDD DDDD"`).
- Range membership checks beyond the §7 constants and the §7.5
  issuable-range predicate (e.g. no per-jurisdiction classification).
