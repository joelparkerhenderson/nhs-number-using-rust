# Release

How to cut a new release of the `nhs-number` crate.

The human-facing version of this checklist lives at
[`help/releasing/index.md`](../help/releasing/index.md); keep the two in
sync. This file adds the **why** behind each step.

## Versioning policy

The crate follows [Semantic Versioning](https://semver.org/).

| Change                                                       | Required bump |
| ------------------------------------------------------------ | ------------- |
| Add a new public item                                        | minor         |
| Tighten a parse rule, change the `Display` shape, change the | major         |
| serialized `digits` layout, or change the check-digit result |               |
| Fix an internal bug with no observable change                | patch         |
| Update docs only                                             | patch         |
| Bump dependencies without an API change                      | patch         |

If you cannot decide, ask: "could a downstream `cargo update` break under
this change?". If yes, it is a major release.

## Step-by-step

### 1. Bump the version

Edit `Cargo.toml`:

```toml
[package]
version = "X.Y.Z"
```

### 2. Verify locally

```sh
cargo build --release
cargo test
cargo doc
cargo clippy --all-targets
```

All four must be clean. Doc-tests are part of `cargo test`.

### 3. Regenerate LLM-friendly docs

```sh
# One-time setup if not already installed:
cargo install rustdoc-llms

rustdoc-llms
cp target/doc/nhs_number.json llms.json
cp target/doc/llms.txt llms.txt
```

`rustdoc-llms` is deliberately **not** a dev-dependency (it has no lib
target), so it does not appear in `Cargo.toml`.

### 4. Commit

```sh
git add --all
git commit -m "Release vX.Y.Z"
```

Keep the commit message short — the changelog lives in tags and the
crates.io page.

### 5. Tag

```sh
top=$(git rev-parse --show-toplevel) &&
version=$(gawk 'match($0, /^version = "([^"]*)"/, a) {print a[1]; exit;}' "$top/Cargo.toml") &&
git tag "$version"
```

The tag name is the bare version, no `v` prefix.

### 6. Push and publish

```sh
git push
git push --tags
cargo publish
```

Confirm at <https://crates.io/crates/nhs-number>.

## Post-release checklist

- [ ] The crates.io page shows the new version.
- [ ] docs.rs has rebuilt (usually within minutes).
- [ ] `llms.txt` and `llms.json` in the repo reflect the new version.
- [ ] If the change is breaking, post the major-version note in
      [`spec/14-compatibility-and-versioning.md`](../spec/14-compatibility-and-versioning.md).

## What gets included in the published crate

```toml
include = ["src/**/*", "LICENSE.md", "README.md"]
```

So `examples/`, `docs/`, `help/`, `spec/`, `AGENTS.md`, and `AGENTS/`
are **not** shipped to crates.io — they exist on GitHub only. If you add a
new top-level file that the crate needs at install time, extend
`include` in `Cargo.toml` to match.
