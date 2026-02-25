# AGENTS.md

Guidance for AI coding agents working in this repository.

This file intentionally contains **agent-focused implementation context** that is not duplicated in `README.md`.

## Critical Build/Lint Gotcha

`cargo clippy --workspace` can hang indefinitely due to a `cynic` proc-macro + clippy interaction on the large Linear schema.

Use this instead:

```fish
cargo clippy -p linear-cli --no-deps
```

## Workspace Architecture (3 crates)

The workspace is split to isolate expensive schema/query macro compilation:

- `crates/linear-schema`
  - Registers the GraphQL schema and scalar types
  - Changes rarely
- `crates/linear-queries`
  - Holds `cynic` `QueryFragment`/`InputObject`/enum derives
  - Changes when query shapes change
- `linear-cli` (root crate)
  - CLI, handlers, formatters, domain mapping, command wiring
  - Frequent iteration target

Why this split exists:
- touching app code in the root crate stays fast (`cargo check` incremental is typically sub-second)
- schema/query proc-macro work is mostly isolated to dedicated crates

## Important Paths

- Schema SDL: `schema/linear.graphql`
- Query/type definitions: `crates/linear-queries/src/lib.rs`
- Schema module/scalars: `crates/linear-schema/src/lib.rs`
- CLI command definitions: `src/cli.rs`
- Client interfaces/implementations: `src/client/`
- Output formatting: `src/output/`
- Integration tests: `tests/`

## Implementation Gotchas

### 1) `cynic::InputObject` optional fields

For optional GraphQL input fields, add:

```rust
#[cynic(skip_serializing_if = "Option::is_none")]
```

Without it, `Option<T>` may serialize as explicit `null`, which can cause Linear API filter errors for some inputs.

### 2) JSON formatting assertions in tests

JSON style (pretty vs compact) may vary by environment/TTY/config. Avoid whitespace-sensitive string assertions in tests.

Prefer:
- parse with `serde_json::Value`
- assert semantic fields

### 3) Handler-level config in tests

If a handler must honor JSON style deterministically via injected config, resolve style in handler code before serialization.

## Verification Commands

Use these before claiming task completion:

```fish
cargo test --workspace
cargo check
cargo fmt --all
cargo clippy -p linear-cli --no-deps
```

## Design Constraints

- Non-interactive CLI behavior (no required prompts for normal automation paths)
- LLM-friendly structured output (machine-readable by default in non-TTY contexts)
- Predictable command/flag patterns across resources
- Explicit, typed error handling with stable exit-code semantics

## Authentication Token Precedence

Resolution order:
1. `LINEAR_TOKEN`
2. `LINEAR_API_TOKEN`
3. Keyring storage

## Notes for Agents

- Prefer minimal, surgical edits unless a full-file rewrite is clearer.
- Keep tests deterministic and isolated; avoid shared mutable global state.
- For CLI behavior changes, update tests first (TDD style used across the codebase).
