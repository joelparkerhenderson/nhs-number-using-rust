# Spec-driven development

This crate uses **spec-driven development**: every behavioural change starts
in [`spec.md`](../spec.md), then propagates outward to tests, code, and
docs. The spec is the source of truth.

## What "spec-driven" means here

1. **`spec.md` is canonical.** When `spec.md` and the code disagree,
   `spec.md` is right and the code is a bug — or the spec is right and
   needs updating *before* the code changes.
2. **No silent behaviour changes.** A PR that changes observable behaviour
   without touching `spec.md` is incomplete.
3. **Tests express the spec.** A unit test or doc-test is the executable
   form of a spec clause. New clauses get new tests.
4. **Docs follow the spec.** `index.md`, `docs/**`, and the rustdoc-level
   examples are derived; they explain and illustrate `spec.md`, they do not
   define it.

## When you must touch `spec.md`

Any change to:

- The check-digit algorithm.
- The set of accepted `FromStr` formats.
- The `Display` / `Into<String>` output format.
- The public surface (adding, removing, renaming an item).
- The serialised shape (`serde`).
- The testable-range bounds.
- The `Cargo.toml` `[dependencies]` set in a way that affects callers.

If your PR touches any of the above and does not edit `spec.md`, stop and
update the spec first.

## The change loop

For a non-trivial behavioural change:

1. **Edit `spec.md`** to describe the target behaviour. Use plain prose;
   include worked examples where the behaviour is subtle.
2. **Write or update tests** that encode the new clauses. Tests should fail
   against the current implementation.
3. **Edit the code** until the new tests pass and the old tests still pass.
4. **Update derived docs** — `index.md`, `docs/**`, example files — so they
   read consistently with the new `spec.md`.
5. **Run** `cargo test && cargo clippy --all-targets && cargo doc`.
6. **Commit** with a message that calls out the spec section that changed.

For a non-behavioural change (refactor, formatting, comment clean-up), you
do not need to touch `spec.md`; the spec covers behaviour, not code shape.

## When spec and code disagree

If you discover a divergence — even an old one — record it in the
"Open questions and known divergences" section of `spec.md` *before*
deciding what to do about it. The fix may turn out to be a code change, a
spec change, or both, but in every case the divergence needs to be visible
to future readers.

## Why this discipline?

- Patient-safety changes deserve a written rationale, not just a diff.
- AI agents (and humans) joining the project should be able to read
  `spec.md` and know what the crate *should* do, without reverse-engineering
  it from the tests.
- Reviewers can compare a behavioural diff to a spec diff and immediately
  see what is and is not in scope.

## Boundary: what does **not** belong in `spec.md`

- Internal code structure (covered by [`architecture.md`](architecture.md)).
- Coding conventions (covered by [`conventions.md`](conventions.md)).
- Test patterns (covered by [`testing.md`](testing.md)).
- Release mechanics (covered by [`release.md`](release.md)).
- Day-to-day commands (covered by [`workflows.md`](workflows.md)).

`spec.md` is for the **observable behaviour** of the crate's public API.
Everything else lives in topical agent guides.
