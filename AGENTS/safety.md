# Safety

Patient-safety constraints. **Read this before writing code that touches NHS
Numbers.**

This document is load-bearing: every rule below has a real-world reason and
must be respected even when it is inconvenient.

## 1. Never invent an NHS Number

Do not generate a ten-digit number off the top of your head, even one that
"looks fake". Even an obviously-made-up number could collide with a real
patient record somewhere.

**Use only:**

- The **testable range** `999 000 0000` – `999 999 9999`. This range is
  reserved by the NHS as never-issued; the crate exposes it via
  `TESTABLE_MIN`, `TESTABLE_MAX`, `TESTABLE_RANGE_INCLUSIVE`, and
  `testable_random_sample()`.
- The **public reference numbers** from the
  [Wikipedia NHS Number article](https://en.wikipedia.org/wiki/NHS_number):
  - `943 476 5919` (the worked checksum example).
  - `987 654 4321` (the format example).

When writing a doc-test, an example, or a unit test, draw from those sources
and nowhere else.

**One narrow exception** — range-boundary fixtures. Testing the
issuable-range predicate (`is_issuable_range`) requires digit arrays at the
boundaries of the issued ranges, which necessarily lie outside the testable
range. Such fixtures are permitted **only if** the stored tenth digit is
deliberately chosen to make the check digit *invalid*, so the fixture cannot
denote any real issued NHS Number (issued numbers always carry a valid check
digit). The `ranges` test sub-module builds every such fixture through a
helper that asserts this property; follow the same pattern.

## 2. Never weaken the check-digit validator

Patient safety depends on correct check-digit validation. A weakened
validator would silently accept transcription errors.

Changes to `calculate_check_digit` or `validate_check_digit` require:

1. A direct citation of the
   [NHS Number specification](https://en.wikipedia.org/wiki/NHS_number).
2. A corresponding update to [`spec/index.md`](../spec/index.md), in the same change.
3. New tests that cover the changed behaviour, including the boundary
   cases at `sum % 11 ∈ {0, 1}`.

If a change relaxes a check, it must be justified as conformance with the
spec, not as a convenience.

## 3. Never log, embed, or transmit an NHS Number from caller code

Inside the crate this is enforced trivially — the crate never reaches out
to the network or filesystem. When recommending integration patterns to
users, prefer code that:

- Treats `NHSNumber` as sensitive data.
- Does not include the full number in log lines, error messages, or
  telemetry that leaves the patient record's trust boundary.
- Hashes or redacts the number when it must appear in audit trails.

## 4. Source of truth for examples

The testable range is the **first** place to look for examples. The two
Wikipedia reference numbers are acceptable in documentation because they
are already public. Anything else is unacceptable.

## 5. The check digit is a transcription-error catcher, not a registry lookup

A passing `validate_check_digit` does **not** mean the number belongs to a
real patient — it means the digits are arithmetically self-consistent. Do
not communicate (in docs, comments, or error messages) that a valid checksum
implies a real patient. The [`docs/faq/index.md`](../docs/faq/index.md)
already states this; new docs should match the same framing.

## 6. Preserve the multi-license headers

All source files in this crate are multi-licensed under MIT, Apache-2.0,
GPL-2.0, GPL-3.0, and BSD-3-Clause. Do not remove or alter the license
headers; do not introduce single-license files.

## 7. Random samples are not guaranteed valid

`testable_random_sample()` draws the tenth digit randomly. Roughly nine in
ten random samples will have an invalid check digit. That is **intentional**
— it lets tests exercise the invalid branch. If a test needs a valid
sample, either:

- Loop on `validate_check_digit()`, or
- Pick the first nine digits yourself and compute the tenth with
  `calculate_check_digit` (see
  [`examples/generate_valid.rs`](../examples/generate_valid.rs)).

## Quick decision table

| Task                                       | Allowed source                            |
| ------------------------------------------ | ----------------------------------------- |
| Unit test fixture                          | Testable range `999 000 0000 – 999 999 9999` |
| Doc-test example                           | Testable range or Wikipedia reference numbers |
| README / tutorial code                     | Testable range or Wikipedia reference numbers |
| Performance benchmark fixture              | Testable range only                       |
| Anything written for a real deployment     | Caller's own data; never hard-code         |
