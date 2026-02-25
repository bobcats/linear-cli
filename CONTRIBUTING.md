# Contributing to linear-cli

Thanks for your interest in contributing.

## Prerequisites

- Rust 1.90+
- Git
- (Optional for live API runs) Linear API token from [linear.app/settings/api](https://linear.app/settings/api)

## Setup

```fish
git clone https://github.com/bobcats/linear-cli
cd linear-cli
cargo build
```

## Development Workflow

This repository follows test-first development:

1. Write/adjust tests
2. Implement minimal code to pass
3. Refactor while staying green
4. Run verification commands before commit

### Verification Commands

```fish
cargo test --workspace
cargo check
cargo fmt --all
cargo clippy -p linear-cli --no-deps
```

> Do not use `cargo clippy --workspace` here; see [AGENTS.md](AGENTS.md) for context.

## Testing Guidance

- Prefer deterministic tests with mocks/fakes over live API calls
- Keep tests isolated and parallel-safe
- For CLI/output behavior, cover JSON and table paths where relevant
- Avoid whitespace-sensitive JSON assertions; parse JSON and assert fields

## Architecture Pointers

- CLI command definitions: `src/cli.rs`
- Client traits + implementations: `src/client/`
- Resource command handlers/types: `src/issues`, `src/projects`, `src/teams`, etc.
- Output formatting: `src/output/`
- Integration tests: `tests/`

For agent-oriented implementation notes and workspace gotchas, see [AGENTS.md](AGENTS.md).

## Commit Messages

Use Conventional Commit style when possible:

- `feat:` new functionality
- `fix:` bug fixes
- `docs:` documentation changes
- `test:` tests only
- `refactor:` behavior-preserving restructuring
- `perf:` performance improvements
- `chore:` tooling/build/maintenance

## Pull Requests

Before opening a PR:

- [ ] `cargo test --workspace` passes
- [ ] `cargo fmt --all` applied
- [ ] `cargo clippy -p linear-cli --no-deps` passes
- [ ] Docs updated for user-facing behavior changes

Include a clear description of what changed and why.

## Security and Tokens

Do not commit real API tokens, credentials, or local environment files.

Token lookup precedence in the app is:
1. `LINEAR_TOKEN`
2. `LINEAR_API_TOKEN`
3. keyring storage

## License

By contributing, you agree contributions are dual-licensed under:

- Apache-2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT ([LICENSE-MIT](LICENSE-MIT))

unless explicitly stated otherwise.
