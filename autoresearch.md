# Autoresearch Rules

## Target
Optimize performance of the new long-form text command paths:
- `issue create --description-file`
- `issue update --description-file`
- `issue comment add --body-file`

## Primary metric
- **speed** (lower is better), measured as wall-clock seconds per experiment run.

## Secondary metric
- **memory** (lower is better), tracked as max RSS (KB) from `/usr/bin/time -l` output.
- Secondary metric is monitoring-only unless degradation is catastrophic.

## Benchmark workload
Use focused CLI parse and command tests for the new paths:
- `cargo test --test issue_create_cli_tests --test issue_update_cli_tests --test issue_comment_add_cli_tests`

When needed, run broader checks:
- `cargo check`
- `cargo clippy -p linear-cli --no-deps`

## Loop discipline
1. Run experiment command.
2. Record primary metric in autoresearch log.
3. Keep only if primary metric improves.
4. If a promising idea is deferred, append it to `autoresearch.ideas.md`.
