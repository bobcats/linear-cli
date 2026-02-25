# Compile Time After Workspace Split

**Date:** 2026-02-24
**Rust:** 1.90.0+
**Machine:** macOS ARM64 (Apple Silicon)
**Schema:** schema/linear.graphql (36,187 lines)
**Layout:** 3-crate workspace (linear-schema, linear-queries, linear-cli)
**cynic features:** rkyv enabled

## Incremental Build — cargo check (touch handler file)

```
touch src/issues/commands/list.rs && time cargo check
```

| Metric | Value |
|--------|-------|
| Wall time | 0.35s |

## Incremental Build — cargo clippy (touch handler file)

```
touch src/issues/commands/list.rs && time cargo clippy -p linear-cli --no-deps
```

| Metric | Value |
|--------|-------|
| Wall time | 4.3s |

## Comparison to Baseline (single crate, no rkyv)

| Command | Before | After | Improvement |
|---------|--------|-------|-------------|
| cargo check (incremental) | ~4s | 0.35s | **11x** |
| cargo clippy (incremental) | ∞ (hung) | 4.3s | **Fixed** |
| cargo check (incremental, pre-rkyv) | 3m 43s | 0.35s | **630x** |

## Notes

- `cargo clippy --workspace` still hangs when linting the linear-queries crate
  (known cynic proc-macro + clippy interaction bug on large schemas)
- Use `cargo clippy -p linear-cli --no-deps` for daily development
- Schema/queries crates rebuild in ~4-5s on first clippy run but this is a
  one-time cost per session
