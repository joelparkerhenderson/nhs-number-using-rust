# `nhs-number` crate specification

**Status:** living document. Updated alongside every behavioural change.
**Audience:** maintainers, AI agents, downstream integrators reading the
crate's contract.
**Companion docs:** [`AGENTS.md`](../AGENTS.md) for agent guidance,
[`index.md`](../index.md) for the user-facing README,
[`docs/api/index.md`](../docs/api/index.md) for the rendered API surface.

This directory is the **canonical specification** — one file per section, indexed below — that drives spec-driven
development (see [`AGENTS/spec-driven-development.md`](../AGENTS/spec-driven-development.md)).
When the spec and the code disagree, the spec is the source of truth and
the code is a bug — or the spec needs updating *before* the code changes.

The discipline:

1. **Behaviour is described here first.** A PR that changes observable
   behaviour without touching the matching section file is incomplete.
2. **Every behavioural rule is testable.** Sections that state a rule
   point to the test that enforces it, either via a `[Rule …]` tag (see
   §2.3) or via the §13.1 coverage table.
3. **Plans and tasks live here too.** §16 holds the **roadmap** in
   priority order; §17 holds the **backlog of open tasks** with stable
   `T<n>` IDs; §18 holds **open questions and known divergences**. There
   is no separate `plan.md` or `tasks.md`.

---

## Table of contents

| § | Section | File |
| - | ------- | ---- |
| 1 | Purpose and scope | [01-purpose-and-scope.md](01-purpose-and-scope.md) |
| 2 | Domain model | [02-domain-model.md](02-domain-model.md) |
| 3 | Data model | [03-data-model.md](03-data-model.md) |
| 4 | Public API surface | [04-public-api-surface.md](04-public-api-surface.md) |
| 5 | String forms (parsing and formatting) | [05-string-forms.md](05-string-forms.md) |
| 6 | Check-digit algorithm | [06-check-digit-algorithm.md](06-check-digit-algorithm.md) |
| 7 | Number ranges | [07-number-ranges.md](07-number-ranges.md) |
| 8 | Random sampling | [08-random-sampling.md](08-random-sampling.md) |
| 9 | Ordering, equality, and collection use [R13] | [09-ordering-equality-collections.md](09-ordering-equality-collections.md) |
| 10 | Patient-safety framing | [10-patient-safety-framing.md](10-patient-safety-framing.md) |
| 11 | Serialisation | [11-serialisation.md](11-serialisation.md) |
| 12 | Error handling | [12-error-handling.md](12-error-handling.md) |
| 13 | Testing strategy | [13-testing-strategy.md](13-testing-strategy.md) |
| 14 | Compatibility and versioning | [14-compatibility-and-versioning.md](14-compatibility-and-versioning.md) |
| 15 | Dependencies and build | [15-dependencies-and-build.md](15-dependencies-and-build.md) |
| 16 | Roadmap | [16-roadmap.md](16-roadmap.md) |
| 17 | Open tasks (backlog) | [17-open-tasks.md](17-open-tasks.md) |
| 18 | Open questions and known divergences | [18-open-questions-and-divergences.md](18-open-questions-and-divergences.md) |
| 19 | Glossary | [19-glossary.md](19-glossary.md) |

Section numbers are stable: prose, code comments, tests, and commit
messages cite `§N.x`, and the behavioural rule index (R1–R23) lives in
[02-domain-model.md](02-domain-model.md) §2.3.
