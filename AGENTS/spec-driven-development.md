# Spec-driven development

This crate uses **spec-driven development**: every behavioural change starts
in the [`spec/`](../spec/) directory, then propagates outward to tests,
code, and docs. The spec is the source of truth.

The spec is split **one file per section** — start at
[`spec/index.md`](../spec/index.md), which holds the table of contents.
Section numbers (`§N.x`) are stable across the split: `§6.4` always means
section 6 (`spec/06-check-digit-algorithm.md`), subsection 4, and that is
how code comments, tests, and commit messages cite the spec.

## What "spec-driven" means here

1. **The `spec/` files are canonical.** When the spec and the code disagree,
   the spec is right and the code is a bug — or the spec is right and
   needs updating *before* the code changes.
2. **No silent behaviour changes.** A PR that changes observable behaviour
   without touching the matching `spec/` section file is incomplete.
3. **Tests express the spec.** A unit test or doc-test is the executable
   form of a spec clause. New clauses get new tests.
4. **Docs follow the spec.** `index.md`, `docs/**`, and the rustdoc-level
   examples are derived; they explain and illustrate the spec, they do not
   define it.

## When you must touch the spec

Any change to:

- The check-digit algorithm (`spec/06-check-digit-algorithm.md`).
- The set of accepted `FromStr` formats (`spec/05-string-forms.md`).
- The `Display` / `Into<String>` output format (`spec/05-string-forms.md`).
- The public surface — adding, removing, renaming an item
  (`spec/04-public-api-surface.md`).
- The serialised shape or its validation (`spec/11-serialisation.md`).
- The number ranges or the issuable-range predicate
  (`spec/07-number-ranges.md`).
- The testable-range bounds (`spec/07-number-ranges.md`).
- The `Cargo.toml` `[dependencies]` set in a way that affects callers
  (`spec/15-dependencies-and-build.md`).

If your PR touches any of the above and does not edit the matching spec
file, stop and update the spec first. Behavioural rules carry stable IDs
(`R<n>`, indexed in `spec/02-domain-model.md` §2.3); adding or changing a
rule updates that index in the same PR.

## The change loop

For a non-trivial behavioural change:

1. **Edit the matching `spec/` file** to describe the target behaviour.
   Use plain prose; include worked examples where the behaviour is subtle.
2. **Write or update tests** that encode the new clauses. Tests should fail
   against the current implementation.
3. **Edit the code** until the new tests pass and the old tests still pass.
4. **Update derived docs** — `index.md`, `docs/**`, example files — so they
   read consistently with the new spec text.
5. **Run** `cargo test && cargo clippy --all-targets && cargo doc`.
6. **Commit** with a message that calls out the spec section that changed.

For a non-behavioural change (refactor, formatting, comment clean-up), you
do not need to touch the spec; it covers behaviour, not code shape.

## When spec and code disagree

If you discover a divergence — even an old one — record it in
[`spec/18-open-questions-and-divergences.md`](../spec/18-open-questions-and-divergences.md)
*before* deciding what to do about it. The fix may turn out to be a code
change, a spec change, or both, but in every case the divergence needs to
be visible to future readers.

## Why this discipline?

- Patient-safety changes deserve a written rationale, not just a diff.
- AI agents (and humans) joining the project should be able to read the
  spec and know what the crate *should* do, without reverse-engineering
  it from the tests.
- Reviewers can compare a behavioural diff to a spec diff and immediately
  see what is and is not in scope.

## Boundary: what does **not** belong in the spec

- Internal code structure (covered by [`architecture.md`](architecture.md)).
- Coding conventions (covered by [`conventions.md`](conventions.md)).
- Test patterns (covered by [`testing.md`](testing.md)).
- Release mechanics (covered by [`release.md`](release.md)).
- Day-to-day commands (covered by [`workflows.md`](workflows.md)).

The `spec/` files are for the **observable behaviour** of the crate's
public API. Everything else lives in topical agent guides.

## Planning and tasks live in the spec

This project does not maintain a separate `plan.md` or `tasks.md`. Instead:

- The **roadmap** lives in [`spec/16-roadmap.md`](../spec/16-roadmap.md).
- The **open task backlog** lives in
  [`spec/17-open-tasks.md`](../spec/17-open-tasks.md), with stable `T<n>`
  identifiers callers can reference from commits.
- **Open questions and known divergences** live in
  [`spec/18-open-questions-and-divergences.md`](../spec/18-open-questions-and-divergences.md).

When you finish a task, edit §17 in the same PR that ships the work so the
backlog reflects reality. When you take on a task, link the commit message
to its `T<n>` identifier.
