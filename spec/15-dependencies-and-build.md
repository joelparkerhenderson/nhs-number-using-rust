[`nhs-number` specification](index.md) — section 15 of 19. Section numbers (§15.x) are stable and cited from code, tests, and commit messages.

# 15. Dependencies and build

### 15.1 Runtime dependencies

| Crate   | Purpose                                                   |
| ------- | --------------------------------------------------------- |
| `rand`  | Random sampling for `testable_random_sample`.             |
| `serde` | Derived `Serialize`/`Deserialize` (default struct shape). |

Adding a runtime dependency that affects callers is a major-version
decision; bumping minor or patch versions of an existing dependency is
typically a patch release. The dependency surface is intentionally tiny;
new runtime deps require a justification in the PR description.

### 15.1.1 Dev-dependencies (tests only)

| Crate        | Purpose                                                       |
| ------------ | ------------------------------------------------------------- |
| `proptest`   | Property-based / fuzz-style tests (§13.2 `fuzz` sub-modules). |
| `serde_json` | Exercises the serde wire shape (§11.1, R16) in tests.         |

Dev-dependencies never ship to callers; adding one is a patch-level
decision.

### 15.2 Build-only tooling

- `rustdoc-llms` regenerates `llms.txt` and `llms.json` at release time
  (see [`AGENTS/release.md`](../AGENTS/release.md)). It is **not** a
  dev-dependency — it is a binary with no lib target, so listing it in
  `Cargo.toml` would warn and be ignored. Install with `cargo install
  rustdoc-llms`.

### 15.3 Edition and MSRV

- Rust edition: **2024**.
- MSRV: tracks the edition (the crate uses 2024-edition features such as
  `LazyLock` re-exports). If a future change raises MSRV, document the new
  minimum in this section in the same PR and decide whether the bump is
  minor or major per the project's downstream commitments.

### 15.4 Published file set

`Cargo.toml` ships only:

```toml
include = ["src/**/*", "LICENSE.md", "README.md"]
```

So `examples/`, `docs/`, `help/`, `spec/`, `AGENTS.md`, and `AGENTS/` are
**not** uploaded to crates.io — they exist on GitHub only. If a new
top-level file becomes required at install time, extend `include` to match.

### 15.5 Required release checks

Every release must pass:

```sh
cargo build --release
cargo test            # unit + doc-tests + examples (via cargo build --examples)
cargo build --examples
cargo clippy --all-targets
cargo fmt -- --check
cargo doc --no-deps
```

Each check is the gate on a known class of regression; do not skip one
because the others pass.
