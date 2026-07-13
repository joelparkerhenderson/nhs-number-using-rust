[`nhs-number` specification](index.md) — section 16 of 19. Section numbers (§16.x) are stable and cited from code, tests, and commit messages.

# 16. Roadmap

The roadmap captures direction. It is intentionally short — each entry is
either being actively designed or scheduled to be. Larger / further-out
ideas live in §17 (backlog) until they earn a slot here.

### 16.1 Near-term (next minor release)

- Keep the §13.1 coverage table in lock-step with `src/`. Whenever a spec
  clause is added, the matching test must land in the same PR.

### 16.2 Mid-term (post-2.0)

- Nothing scheduled. The former mid-term items (`Hash`, the string-form
  serde newtype, the `BTreeMap` example) all shipped in 2.0.0.

### 16.3 Long-term / explicit non-goals

- No registry integration (§1.3).
- No localised string forms (§1.3).
- No range machinery beyond `TESTABLE_RANGE_INCLUSIVE` and the §7.5
  issuable-range predicate (e.g. no per-jurisdiction classification).

A roadmap entry is "done" only when its associated spec section, tests,
and docs all agree.
