[`nhs-number` specification](index.md) — section 14 of 19. Section numbers (§14.x) are stable and cited from code, tests, and commit messages.

# 14. Compatibility and versioning

The crate follows [Semantic Versioning](https://semver.org/):

| Change                                                | Bump  |
| ----------------------------------------------------- | ----- |
| Add a new public item.                                | minor |
| Add a new derived trait on `NHSNumber` (e.g. `Hash`). | minor |
| Tighten or relax a parsing rule.                      | major |
| Tighten or relax deserialisation validation.          | major |
| Change the `Display`/`Into<String>` output.           | major |
| Change the serialised `digits` shape.                 | major |
| Change the value returned by `calculate_check_digit`. | major |
| Change which numbers `validate_check_digit` accepts.  | major |
| Remove or rename a public item.                       | major |
| Change a public item's signature.                     | major |
| Add a new test.                                       | patch |
| Add or expand a doc-test.                             | patch |
| Add a new `examples/*` program.                       | patch |
| Add or expand a `//!` or `///` doc comment.           | patch |
| Fix an internal bug with no observable change.        | patch |
| Documentation-only change (including this spec).      | patch |

Releases are cut per [`AGENTS/release.md`](../AGENTS/release.md).

> **Version 2.0.0 decision record:** the R20 deserialisation tightening
> (§11.1) rejects payloads that 1.0.3 accepted, so the release that
> ships it is a **major** bump even though every other change in the
> same batch (R19 `Hash`, R21 `ParseError` impls, R22 `serde_string`,
> R23 `is_issuable_range`) would alone have been minor.
