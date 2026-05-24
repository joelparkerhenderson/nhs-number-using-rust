# AGENTS.md

Guidance for AI coding agents (Claude Code, Copilot, Cursor, Aider, etc.)
working in this repository.

This file is the **entry point**. It is intentionally short. Drill into the
topical guides under [`AGENTS/`](AGENTS/) for the full picture, and read
[`spec.md`](spec.md) for the canonical specification that drives changes.

## Project snapshot

| Field        | Value                                                                |
| ------------ | -------------------------------------------------------------------- |
| Crate        | `nhs-number`                                                         |
| Purpose      | Model, parse, format, and validate NHS Numbers.                      |
| Jurisdiction | NHS England, NHS Wales, NHS Isle of Man.                             |
| Language     | Rust (edition 2024)                                                  |
| License      | MIT OR Apache-2.0 OR GPL-2.0 OR GPL-3.0 OR BSD-3-Clause              |
| Runtime deps | `rand`, `serde` (with `derive`)                                      |
| Repository   | https://github.com/GIG-Cymru-NHS-Wales/nhs-number-using-rust         |
| Crate        | https://crates.io/crates/nhs-number                                  |
| Docs         | https://docs.rs/nhs-number/                                          |
| Maintainer   | Joel Parker Henderson — joel.henderson@wales.nhs.uk                  |

## How this repo is documented

The documentation is layered so each reader can stop at the depth they need:

```
index.md                   ← README (user-facing introduction)
spec.md                    ← living spec-driven-development specification
AGENTS.md                  ← this file (agent entry point)
AGENTS/
├── architecture.md        ← repo layout, modules, data model, public API
├── conventions.md         ← coding style and doc-comment shape
├── testing.md             ← unit tests, doctests, the actual/expect pattern
├── safety.md              ← patient-safety constraints (NEVER invent numbers)
├── workflows.md           ← common cargo commands, examples, daily flow
├── release.md             ← versioning, llms.* regeneration, publish steps
└── spec-driven-development.md  ← how spec.md drives changes
docs/
├── api/index.md           ← full public API reference
├── checksum/index.md      ← check-digit algorithm with worked examples
├── faq/index.md           ← frequently asked questions
├── ranges/index.md        ← issued, reserved, and testable ranges
└── usage/index.md         ← tutorial-style walk-through
examples/                  ← runnable `cargo run --example <name>` programs
help/releasing/            ← release checklist (mirrors AGENTS/release.md)
```

## Five rules that bind every change

These are the load-bearing constraints. Each is expanded in the matching
topical guide.

1. **Never invent an NHS Number.** Use only Wikipedia public examples
   (`943 476 5919`, `987 654 4321`) or the testable range
   `999 000 0000` – `999 999 9999`. See [`AGENTS/safety.md`](AGENTS/safety.md).
2. **Do not weaken the check-digit validator.** Any change to the algorithm
   requires an explicit reference to the
   [NHS Number specification](https://en.wikipedia.org/wiki/NHS_number)
   and an update to [`spec.md`](spec.md).
3. **Keep the public API stable.** This crate is published on crates.io; any
   breaking change needs a major-version bump per semver.
4. **Preserve the multi-license headers.** All source files are
   multi-licensed (MIT, Apache-2.0, GPL-2.0, GPL-3.0, BSD-3-Clause).
5. **Update `spec.md` first.** Behavioural changes — even small ones — start
   by editing `spec.md`, then the code, then the tests. See
   [`AGENTS/spec-driven-development.md`](AGENTS/spec-driven-development.md).

## Quick orientation for a brand-new agent

If you have just been spawned with no prior context, do this in order:

1. Read this file (you are here).
2. Skim [`spec.md`](spec.md) — it is the source of truth.
3. Skim [`AGENTS/architecture.md`](AGENTS/architecture.md) for the layout.
4. For any task that touches behaviour, open
   [`AGENTS/safety.md`](AGENTS/safety.md) **before** writing code.
5. Run `cargo test` to confirm a green baseline before changing anything.

## Common commands

```sh
cargo build                      # Build
cargo build --release            # Release build
cargo test                       # Unit + doc-test suite
cargo test -- --nocapture        # Show println!() output
cargo doc --no-deps --open       # Build and open rustdoc
cargo run --example basic_usage  # Run an example
cargo clippy --all-targets       # Lint
cargo fmt                        # Format
```

A fuller walk-through, including release commands, lives in
[`AGENTS/workflows.md`](AGENTS/workflows.md) and
[`AGENTS/release.md`](AGENTS/release.md).
