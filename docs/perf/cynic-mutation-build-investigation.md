# Cynic Mutation Build Regression Investigation

**Date:** 2026-03-12  
**Commit:** a15e6398 (main)

## Findings

All `MutationBuilder::build()` calls for issue mutations are ~2× slower in v0.2.0 vs v0.1.0:

| Benchmark | v0.1.0 (cynic 3.12, reqwest 0.12) | v0.2.0 (cynic 3.13, reqwest 0.13) | Δ |
|---|---:|---:|---:|
| `write_mutation_build/issue_create` | 5.63 µs | 11.81 µs | +110% |
| `write_mutation_build/issue_update` | 5.57 µs | 11.83 µs | +112% |
| `write_mutation_build/issue_archive` | 5.49 µs | 11.78 µs | +115% |
| `write_mutation_build/issue_unarchive` | 5.49 µs | 11.50 µs | +110% |
| `write_mutation_build/issue_relation_create` | 7.68 µs | 16.08 µs | +109% |
| `write_mutation_build/comment_create` | 2.68 µs | 2.74 µs | +2% ✅ |

`comment_create` is unaffected, so the regression scales with query/selection-set complexity.

## Root Cause

**cynic 3.12.0 requires reqwest 0.12** (confirmed from its `Cargo.toml`). We upgraded to reqwest 0.13 in commit `758655d7` to fix a compilation error (`ReqwestBlockingExt` trait incompatibility). cynic 3.13.x is the first version that supports reqwest 0.13.

Pinning cynic back to 3.12.0 is not an option — it would require downgrading reqwest to 0.12, reintroducing the original compilation error.

## Decision

**Accept the regression.** Rationale:
- Mutation build at ~12µs is invisible next to network RTT (200ms+)
- The regression is in the cynic 3.13 proc-macro-generated code, not our application code
- No actionable path to recover it without breaking reqwest 0.13 compatibility
- If cynic upstream addresses it, we'll pick up the fix automatically on next version bump

## Deferred

File an upstream issue with cynic if the overhead becomes relevant in a future tighter performance budget.
