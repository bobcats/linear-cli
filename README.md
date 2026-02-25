# linear-cli

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)

A fast, automation-friendly CLI for Linear issue tracking.

> Unofficial tool. Not affiliated with Linear.

## Highlights

- LLM/script-friendly output (JSON by default in non-TTY contexts)
- Read + core write workflows for issues
- Projects, teams, cycles, labels, users, and search commands
- Multiple output formats: JSON, CSV, Markdown, table
- Secure token storage in system keyring (with env-var override)

## Status

Current release line includes:
- Authentication (`auth login|status|logout|token`)
- Issue read and write operations (create/update/comment/lifecycle/relation/delete)
- Project/team/cycle read operations
- Semantic search

See [ROADMAP.md](ROADMAP.md) for upcoming work.

## Installation

Build from source:

```fish
git clone https://github.com/bobcats/linear-cli
cd linear-cli
cargo build --release
# binary: ./target/release/linear-cli
```

Optional local install to PATH:

```fish
cargo install --path .
# binary name remains: linear-cli
```

## Quick Start

```fish
# authenticate
linear-cli auth login

# issue workflows
linear-cli issue view ENG-123
linear-cli issue list --assignee @me --limit 10
linear-cli issue create --team ENG --title "Fix login bug"
linear-cli issue update ENG-123 --priority 2 --state "In Progress"
linear-cli issue comment add ENG-123 --body "Started investigation"
linear-cli issue lifecycle archive ENG-123
linear-cli issue relation link ENG-123 ENG-456

# other resources
linear-cli project list
linear-cli team list
linear-cli cycle current

# output format selection
linear-cli issue list --json | jq '.[0].identifier'
linear-cli project list --csv > projects.csv
linear-cli cycle current --markdown
```

## Authentication

Get a token from [linear.app/settings/api](https://linear.app/settings/api).

```fish
# interactive
linear-cli auth login

# non-interactive
echo "lin_api_xxxxx" | linear-cli auth login --with-token
```

Token resolution order:
1. `LINEAR_TOKEN`
2. `LINEAR_API_TOKEN`
3. keyring storage

Headless Linux note: in CI/containers/SSH-only environments, keyring may be unavailable. Use environment variables.

## Output Formats

All list/view-style commands support:
- `--json`
- `--csv`
- `--markdown`
- `--table`

If no explicit format is passed, output is auto-selected by TTY detection.

JSON style control:
- `LINEAR_CLI_JSON_STYLE=compact`
- `LINEAR_CLI_JSON_STYLE=pretty`

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md) for contributor workflow.

Core commands:

```fish
cargo check
cargo test --workspace
cargo fmt --all
cargo clippy -p linear-cli --no-deps
```

## Documentation

- Contributor guide: [CONTRIBUTING.md](CONTRIBUTING.md)
- Agent-specific repository context: [AGENTS.md](AGENTS.md)
- Future direction: [ROADMAP.md](ROADMAP.md)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you is dual-licensed as above without additional terms.
