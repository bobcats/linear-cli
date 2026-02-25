# Roadmap

## Current State

`linear-cli` supports:

- Authentication (`auth login|status|logout|token`)
- Issues: view/list/create/update/delete, comments, lifecycle actions, relations
- Projects: list/view
- Teams: list/view
- Cycles: list/view/current
- Labels and users: list
- Search (including semantic search)
- Output formats: JSON, CSV, Markdown, table

## Next Priorities

### 1) Automation Ergonomics

- Field selection (`--fields`) for smaller machine-readable payloads
- Better batch workflows (multiple identifiers / stdin-driven operations)
- Additional command parity improvements across resources

### 2) Performance and Caching

- User-facing cache controls (`--no-cache`, configurable TTL)
- Continued formatter and command-path benchmarking
- Regression gates for common CLI paths

### 3) Distribution

- Prebuilt binaries for macOS/Linux/Windows
- Release automation and artifact publishing
- Optional package-manager distribution

## Backlog Themes

- More write operations beyond current issue coverage
- Browser integration (`--web`) where it improves workflow speed
- Additional quality-of-life flags for scripting and CI usage

## Guiding Principles

- LLM-first and script-first UX
- Predictable command patterns
- Non-interactive, automation-safe behavior
- Strong test coverage and typed API boundaries
