# v0.2.0 Performance Pass Evidence

**Date:** 2026-03-12  
**Branch:** main (`456cfeb8`)  
**Benchmarks:** --quick, CARGO_TARGET_DIR=target_latest

---

## Optimization 1: Table preset — UTF8_FULL → ASCII_MARKDOWN

**Change:** `src/output/generic_formatters.rs`, `src/issues/types.rs`, `src/comments/types.rs`, `src/auth/output.rs`

| Benchmark ID | Before (ns) | After (ns) | Delta (%) | Notes |
|---|---:|---:|---:|---|
| table_hotspot_large_rows/project_list_table/1000 | 2365663 | 1976000 | -16.5% | ASCII_MARKDOWN skips Unicode width calc |
| table_hotspot_large_rows/team_list_table/1000 | 1795889 | 1412000 | -21.4% | |
| table_hotspot_large_rows/project_list_table/500 | 1185317 | 986000 | -16.8% | |
| table_hotspot_wide_cells/team_list_table_wide | 1002175 | 819000 | -18.3% | |
| issue_list/table/500 | 980766 | 788000 | -19.7% | |
| table_hotspot_large_rows/team_list_table/500 | 895821 | 706000 | -21.2% | |
| table_hotspot_large_rows/project_list_table/250 | 586937 | 490000 | -16.5% | |
| table_hotspot_large_rows/team_list_table/250 | 456166 | 357000 | -21.7% | |
| cycle_list/table/100 | 261938 | 210000 | -19.8% | |
| issue_list/table/100 | 193051 | 154000 | -20.2% | |
| team_list/table/100 | 181647 | 140000 | -22.9% | |
| comment_list/table/100 | 187195 | 143000 | -23.6% | |
| issue_table | 14031 | 10505 | -25.1% | Single-item vertical table |

---

## Optimization 2: Single-issue CSV allocation reduction

**Change:** `src/issues/types.rs` — `Issue::to_csv`

Key changes:
- `parent_str`: borrow `identifier` directly instead of `.clone()`
- `children_str`: only allocate joined string when children exist (None → `""`)
- `comment_str`: avoid `to_string()` when `None` (use literal `"0"`)

| Benchmark ID | Before (ns) | After (ns) | Delta (%) | Notes |
|---|---:|---:|---:|---|
| issue_csv | 382 | 358 | -6.3% | Baseline case (no parent/children/comments) |
| issue_with_comments/csv/0 | 394 | 350 | -11.2% | |
| issue_with_comments/csv/10 | 399 | 363 | -9.0% | |
| issue_with_comments/csv/50 | 400 | 354 | -11.5% | |

---

## Accepted regressions (documented separately)

See `docs/perf/cynic-mutation-build-investigation.md`:
- All issue mutation builds are ~110% slower vs v0.1.0 due to cynic 3.12→3.13 upgrade (required for reqwest 0.13 compatibility). Not actionable without downgrading reqwest.
