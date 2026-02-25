# Compile Time Baseline

**Date:** 2026-02-24
**Rust:** 1.90.0+
**Machine:** macOS ARM64 (Apple Silicon)
**Schema:** schema/linear.graphql (36,187 lines)
**Crate:** single crate, no workspace

## Clean Build (no rkyv)

```
cargo clean && time cargo clippy --no-deps
```

| Metric | Value |
|--------|-------|
| Total wall time | 4m 09s |
| User time | 4m 27s |
| Sys time | 0m 08s |

## Incremental Build — cargo check (warm cache)

```
touch src/issues/commands/list.rs && time cargo check
```

| Metric | With rkyv | Without rkyv |
|--------|-----------|--------------|
| Wall time | 1.8s | ~4s |

## Clippy Bug

**clippy hangs indefinitely on incremental builds** with cynic proc macros
against the 36k-line Linear schema. This happens both with and without rkyv.
Clean builds work (clippy compiles everything from scratch), but incremental
clippy never completes — 0% CPU, ~24MB RSS, stuck forever.

This is the primary motivation for workspace splitting: moving proc-macro
code into separate crates means clippy only re-checks the CLI crate
(no proc-macro expansion needed).

## Analysis

`cargo check` is fast (1.8-4s incremental) because rustc's incremental
compilation handles the proc-macro output well. The problem is specifically
clippy hanging on reanalysis of proc-macro-generated code.

**Target after workspace split:**
- `cargo check -p linear-cli` after handler change: < 2s (already fast)
- `cargo clippy -p linear-cli --no-deps` after handler change: < 10s (currently hangs)
