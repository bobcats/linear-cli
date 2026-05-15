# Milestone Performance Coverage Implementation Plan

> REQUIRED: Use the `executing-plans` skill to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add benchmark coverage for milestone workflows and refresh performance evidence so future optimizations are driven by measured hotspots, not intuition.

**Architecture:** Extend the existing Criterion benchmark matrix instead of adding new tooling. Milestone formatter benchmarks live beside other resource formatters, milestone command handler/parse benchmarks live in `command_paths`, and milestone GraphQL mutation build benchmarks live in `write_operations`. Update perf docs only after benchmarks compile and run so recorded evidence matches the current code.

**Tech Stack:** Rust 2024, Criterion, cynic GraphQL mutation builders, existing `linear-cli` formatter/handler traits, existing perf docs under `docs/perf/`.

---

## Context

Previous performance work showed table rendering dominates CLI latency and that measured evidence matters more than speculative micro-optimizations. This plan intentionally adds coverage and refreshes evidence first. Do not optimize production code unless a benchmark in this plan exposes a clear regression or compile failure.

Relevant existing files:

- `benches/formatters.rs` — Criterion formatter benchmarks for issues, projects, teams, cycles, comments, and table hotspots.
- `benches/command_paths.rs` — CLI parse and handler path benchmarks using mock clients/lookups.
- `benches/write_operations.rs` — cynic mutation build and serialize benchmarks plus issue reference resolver benchmark.
- `tests/perf_benchmark_matrix_tests.rs` — static tests proving key benchmark groups exist.
- `docs/perf/baseline-hotspots.md` — ranked Criterion hotspot snapshot.
- `docs/perf/latency-targets.json` — regression-gate target set.
- `docs/perf/hotspot-priority.json` — machine-readable optimization queue.
- `docs/perf/evidence-v0.2.0-perf-pass.md` — historical evidence for prior optimizations.

## File Structure

Modify:

- `tests/perf_benchmark_matrix_tests.rs`
  - Add static assertions requiring milestone formatter, handler, parse, and write benchmark groups.
- `benches/formatters.rs`
  - Import `Milestone`, `MilestoneList`, and `MilestoneProject`.
  - Add milestone fixture helpers.
  - Add `bench_milestone_list` for JSON, CSV, Markdown, and table at sizes matching other list resources.
  - Register the benchmark in `criterion_group!`.
- `benches/command_paths.rs`
  - Import milestone command handlers and `MilestoneClient` input types.
  - Add parse benchmarks for `milestone list/view/create/update/delete`.
  - Add handler benchmarks for milestone list/view/create/update/delete using existing `PassthroughLookup` and a mock milestone client.
- `benches/write_operations.rs`
  - Import project milestone query mutation types.
  - Add variables for create/update/delete milestone operations.
  - Add milestone create/update/delete to mutation build and build+serialize groups.
- `docs/perf/baseline-hotspots.md`
  - Refresh after running benchmarks, using `cargo run --bin perf-hotspots -- --from target/criterion` output.
- `docs/perf/latency-targets.json`
  - Refresh target metadata and add newly relevant high-ranking milestone benchmark targets after observing current results.
- `docs/perf/hotspot-priority.json`
  - Update the description for this coverage pass and keep `top_hotspots` empty.
- `docs/perf/evidence-milestone-performance-coverage.md`
  - Create a short evidence note recording what benchmark coverage was added, benchmark command used, and whether milestone paths appear in the top hotspots.

## TDD and Benchmark Discipline

- Use the `test-driven-development` skill for benchmark matrix changes: write failing static coverage tests before changing benchmark files.
- Use the `systematic-debugging` skill for any benchmark compile or verification failure.
- Use the `verification-before-completion` skill before claiming the plan is complete.
- Do not change production performance behavior unless required to make benchmarks compile.
- Keep commits at task boundaries.

## Task 1: Add failing benchmark matrix coverage tests

**Files:**
- Modify: `tests/perf_benchmark_matrix_tests.rs`

- [ ] **Step 1: Add failing static assertions for milestone formatter coverage**

Add assertions to `test_matrix_includes_format_bench_groups_for_all_non_auth_command_families`:

```rust
assert!(FORMATTERS_BENCH.contains("bench_milestone_list"));
assert!(FORMATTERS_BENCH.contains("milestone_list"));
```

- [ ] **Step 2: Add failing static assertions for milestone command path coverage**

Add assertions to `test_matrix_includes_parse_bench_groups_for_all_non_auth_command_families`:

```rust
assert!(COMMAND_PATHS_BENCH.contains("cli_parse_milestone_paths"));
```

Add assertions to `test_matrix_includes_handler_bench_groups_for_all_non_auth_command_families`:

```rust
assert!(COMMAND_PATHS_BENCH.contains("milestone_handlers_json"));
```

- [ ] **Step 3: Add failing static assertions for milestone write operation coverage**

Add assertions to `test_matrix_includes_write_path_bench_groups`:

```rust
assert!(WRITE_OPS_BENCH.contains("project_milestone_create"));
assert!(WRITE_OPS_BENCH.contains("project_milestone_update"));
assert!(WRITE_OPS_BENCH.contains("project_milestone_delete"));
```

- [ ] **Step 4: Run RED command**

Run:

```bash
cargo test --test perf_benchmark_matrix_tests -- --nocapture
```

Expected: FAIL because the benchmark files do not yet contain the milestone benchmark groups/functions.

- [ ] **Step 5: Commit RED tests**

```bash
git add tests/perf_benchmark_matrix_tests.rs
git commit -m "test(perf): require milestone benchmark coverage"
```

## Task 2: Add milestone formatter benchmarks

**Files:**
- Modify: `benches/formatters.rs`

- [ ] **Step 1: Import milestone formatter types**

Add near the other type imports:

```rust
use linear_cli::milestones::types::{Milestone, MilestoneList, MilestoneProject};
```

- [ ] **Step 2: Add milestone fixture helpers**

Add after existing fixture helpers and before benchmark functions:

```rust
fn create_test_milestone(id: usize) -> Milestone {
    Milestone {
        id: format!("milestone-{id}"),
        name: format!("Milestone {id}"),
        description: Some(format!("Milestone {id} for coordinated delivery.")),
        status: match id % 4 {
            0 => "unstarted",
            1 => "next",
            2 => "overdue",
            _ => "done",
        }
        .to_string(),
        progress: (id % 101) as f64 / 100.0,
        sort_order: id as f64 * 1000.0,
        target_date: (!id.is_multiple_of(3)).then(|| "2026-06-30".to_string()),
        project: MilestoneProject {
            id: format!("project-{}", id % 10),
            name: format!("Project {}", id % 10),
            slug_id: format!("project-{}", id % 10),
        },
        created_at: "2026-01-01T00:00:00Z".to_string(),
        updated_at: "2026-01-02T00:00:00Z".to_string(),
        archived_at: None,
    }
}

fn create_test_milestones(count: usize) -> Vec<Milestone> {
    (0..count).map(create_test_milestone).collect()
}
```

- [ ] **Step 3: Add milestone list formatter benchmark group**

Add near the other list benchmark functions:

```rust
fn bench_milestone_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("milestone_list");

    for size in [10, 50, 100, 500] {
        let milestones = MilestoneList(create_test_milestones(size));

        group.bench_with_input(BenchmarkId::new("json", size), &milestones, |b, list| {
            b.iter(|| black_box(list.to_json().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("csv", size), &milestones, |b, list| {
            b.iter(|| black_box(list.to_csv().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("markdown", size), &milestones, |b, list| {
            b.iter(|| black_box(list.to_markdown().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("table", size), &milestones, |b, list| {
            b.iter(|| black_box(list.to_table().unwrap()))
        });
    }

    group.finish();
}
```

- [ ] **Step 4: Register formatter benchmark**

Add `bench_milestone_list` to the existing `criterion_group!` list in `benches/formatters.rs`.

- [ ] **Step 5: Run GREEN command for formatter matrix**

Run:

```bash
cargo test --test perf_benchmark_matrix_tests test_matrix_includes_format_bench_groups_for_all_non_auth_command_families -- --nocapture
cargo check --benches
```

Expected: the formatter matrix test passes; bench targets compile or reveal the next missing benchmark group from Task 1.

- [ ] **Step 6: Commit formatter benchmark coverage**

```bash
git add benches/formatters.rs
git commit -m "perf: add milestone formatter benchmarks"
```

## Task 3: Add milestone command path benchmarks

**Files:**
- Modify: `benches/command_paths.rs`

- [ ] **Step 1: Add parse benchmark group for milestone CLI paths**

Add a function near the existing CLI parse benchmark functions:

```rust
fn bench_cli_parse_milestone_paths(c: &mut Criterion) {
    let mut group = c.benchmark_group("cli_parse_milestone_paths");

    group.bench_function("milestone_list", |b| {
        b.iter(|| black_box(Cli::parse_from(["linear-cli", "milestone", "list", "--project", "APP", "--limit", "50", "--json"])))
    });

    group.bench_function("milestone_view", |b| {
        b.iter(|| black_box(Cli::parse_from(["linear-cli", "milestone", "view", "Beta", "--project", "APP", "--json"])))
    });

    group.bench_function("milestone_create", |b| {
        b.iter(|| black_box(Cli::parse_from(["linear-cli", "milestone", "create", "--project", "APP", "--name", "Beta", "--target-date", "2026-06-30", "--json"])))
    });

    group.bench_function("milestone_update", |b| {
        b.iter(|| black_box(Cli::parse_from(["linear-cli", "milestone", "update", "Beta", "--project", "APP", "--name", "GA", "--json"])))
    });

    group.bench_function("milestone_delete", |b| {
        b.iter(|| black_box(Cli::parse_from(["linear-cli", "milestone", "delete", "Beta", "--project", "APP", "--json"])))
    });

    group.finish();
}
```

- [ ] **Step 2: Add milestone mock client**

Add this mock client to `benches/command_paths.rs`:

```rust
struct BenchmarkMilestoneClient;

impl MilestoneClient for BenchmarkMilestoneClient {
    fn get_milestone(&self, _token: &str, id: &str) -> Result<Milestone, CliError> {
        Ok(benchmark_milestone(id, "Beta"))
    }

    fn list_milestones(
        &self,
        _token: &str,
        _project_id: Option<&str>,
        _name: Option<&str>,
        limit: usize,
    ) -> Result<Vec<Milestone>, CliError> {
        Ok((0..limit.min(50))
            .map(|idx| benchmark_milestone(&format!("milestone-{idx}"), "Beta"))
            .collect())
    }

    fn create_milestone(
        &self,
        _token: &str,
        _input: CreateMilestoneInput,
    ) -> Result<Milestone, CliError> {
        Ok(benchmark_milestone("milestone-1", "Beta"))
    }

    fn update_milestone(
        &self,
        _token: &str,
        _id: &str,
        _input: UpdateMilestoneInput,
    ) -> Result<Milestone, CliError> {
        Ok(benchmark_milestone("milestone-1", "GA"))
    }

    fn delete_milestone(&self, _token: &str, _id: &str) -> Result<(), CliError> {
        Ok(())
    }
}
```

Add these imports near the existing imports:

```rust
use linear_cli::client::milestones::{CreateMilestoneInput, MilestoneClient, UpdateMilestoneInput};
use linear_cli::milestones::commands::{
    handle_create as handle_milestone_create,
    handle_delete as handle_milestone_delete,
    handle_list as handle_milestone_list,
    handle_update as handle_milestone_update,
    handle_view as handle_milestone_view,
};
```

- [ ] **Step 3: Add milestone fixture helper for command handlers**

Add:

```rust
fn benchmark_milestone(id: &str, name: &str) -> Milestone {
    Milestone {
        id: id.to_string(),
        name: name.to_string(),
        description: Some("Benchmark milestone".to_string()),
        status: "next".to_string(),
        progress: 0.5,
        sort_order: 1000.0,
        target_date: Some("2026-06-30".to_string()),
        project: MilestoneProject {
            id: "project-1".to_string(),
            name: "Project".to_string(),
            slug_id: "project".to_string(),
        },
        created_at: "2026-01-01T00:00:00Z".to_string(),
        updated_at: "2026-01-02T00:00:00Z".to_string(),
        archived_at: None,
    }
}
```

- [ ] **Step 4: Add milestone handler benchmark group**

Add near existing handler benchmark functions:

```rust
fn bench_milestone_handlers(c: &mut Criterion) {
    let config = TestConfigProvider::with_token("test-token");
    let storage = MockTokenStorage::new();
    let io = NoopIo;
    let lookup = PassthroughLookup;
    let client = BenchmarkMilestoneClient;

    let mut group = c.benchmark_group("milestone_handlers_json");

    group.bench_function("list", |b| {
        b.iter(|| {
            black_box(
                handle_milestone_list(
                    Some("APP"),
                    50,
                    &client,
                    &lookup,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    group.bench_function("view", |b| {
        b.iter(|| {
            black_box(
                handle_milestone_view(
                    "Beta",
                    Some("APP"),
                    &client,
                    &lookup,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    group.bench_function("create", |b| {
        b.iter(|| {
            black_box(
                handle_milestone_create(
                    "APP",
                    "Beta",
                    Some("Benchmark milestone".to_string()),
                    Some("2026-06-30".to_string()),
                    &client,
                    &lookup,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    group.bench_function("update", |b| {
        b.iter(|| {
            black_box(
                handle_milestone_update(
                    "Beta",
                    Some("APP"),
                    UpdateMilestoneInput {
                        name: Some("GA".to_string()),
                        description: None,
                        project_id: None,
                        target_date: None,
                    },
                    &client,
                    &lookup,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    group.bench_function("delete", |b| {
        b.iter(|| {
            black_box(
                handle_milestone_delete(
                    "Beta",
                    Some("APP"),
                    &client,
                    &lookup,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    group.finish();
}
```

- [ ] **Step 5: Register command path benchmarks**

Add `bench_cli_parse_milestone_paths` and `bench_milestone_handlers` to the existing `criterion_group!` list in `benches/command_paths.rs`.

- [ ] **Step 6: Run GREEN command for command path matrix**

Run:

```bash
cargo test --test perf_benchmark_matrix_tests test_matrix_includes_parse_bench_groups_for_all_non_auth_command_families -- --nocapture
cargo test --test perf_benchmark_matrix_tests test_matrix_includes_handler_bench_groups_for_all_non_auth_command_families -- --nocapture
cargo check --benches
```

Expected: parse and handler matrix tests pass; bench targets compile or reveal the next missing benchmark group from Task 1.

- [ ] **Step 7: Commit command path benchmark coverage**

```bash
git add benches/command_paths.rs
git commit -m "perf: add milestone command path benchmarks"
```

## Task 4: Add milestone mutation build benchmarks

**Files:**
- Modify: `benches/write_operations.rs`

- [ ] **Step 1: Import milestone mutation types**

Extend the `linear_cli::client::queries` import with:

```rust
ProjectMilestoneCreateInput,
ProjectMilestoneCreateMutation,
ProjectMilestoneCreateMutationVariables,
ProjectMilestoneDeleteMutation,
ProjectMilestoneDeleteMutationVariables,
ProjectMilestoneUpdateInput,
ProjectMilestoneUpdateMutation,
ProjectMilestoneUpdateMutationVariables,
TimelessDate,
```

- [ ] **Step 2: Add milestone mutation variable helpers**

Add after existing issue/comment variable helpers:

```rust
fn project_milestone_create_vars() -> ProjectMilestoneCreateMutationVariables {
    ProjectMilestoneCreateMutationVariables {
        input: ProjectMilestoneCreateInput {
            project_id: "project-1".to_string(),
            name: "Beta".to_string(),
            description: Some("Beta readiness".to_string()),
            target_date: Some(TimelessDate("2026-06-30".to_string())),
        },
    }
}

fn project_milestone_update_vars() -> ProjectMilestoneUpdateMutationVariables {
    ProjectMilestoneUpdateMutationVariables {
        id: "milestone-1".to_string(),
        input: ProjectMilestoneUpdateInput {
            name: Some("GA".to_string()),
            description: Some("General availability".to_string()),
            project_id: None,
            target_date: Some(TimelessDate("2026-07-31".to_string())),
        },
    }
}

fn project_milestone_delete_vars() -> ProjectMilestoneDeleteMutationVariables {
    ProjectMilestoneDeleteMutationVariables {
        id: "milestone-1".to_string(),
    }
}
```

- [ ] **Step 3: Add milestone mutation build benchmark cases**

Inside `bench_write_mutation_build`, add:

```rust
group.bench_function("project_milestone_create", |b| {
    b.iter(|| black_box(ProjectMilestoneCreateMutation::build(project_milestone_create_vars())))
});

group.bench_function("project_milestone_update", |b| {
    b.iter(|| black_box(ProjectMilestoneUpdateMutation::build(project_milestone_update_vars())))
});

group.bench_function("project_milestone_delete", |b| {
    b.iter(|| black_box(ProjectMilestoneDeleteMutation::build(project_milestone_delete_vars())))
});
```

- [ ] **Step 4: Add milestone mutation build+serialize benchmark cases**

Inside `bench_write_mutation_serialize`, add:

```rust
group.bench_function("project_milestone_create", |b| {
    b.iter(|| {
        let operation = ProjectMilestoneCreateMutation::build(project_milestone_create_vars());
        black_box(serde_json::to_string(&operation).unwrap())
    })
});

group.bench_function("project_milestone_update", |b| {
    b.iter(|| {
        let operation = ProjectMilestoneUpdateMutation::build(project_milestone_update_vars());
        black_box(serde_json::to_string(&operation).unwrap())
    })
});

group.bench_function("project_milestone_delete", |b| {
    b.iter(|| {
        let operation = ProjectMilestoneDeleteMutation::build(project_milestone_delete_vars());
        black_box(serde_json::to_string(&operation).unwrap())
    })
});
```

- [ ] **Step 5: Run GREEN command for write operation matrix**

Run:

```bash
cargo test --test perf_benchmark_matrix_tests test_matrix_includes_write_path_bench_groups -- --nocapture
cargo check --benches
```

Expected: write path matrix test passes and benchmarks compile.

- [ ] **Step 6: Run full benchmark matrix test**

Run:

```bash
cargo test --test perf_benchmark_matrix_tests -- --nocapture
```

Expected: PASS.

- [ ] **Step 7: Commit write operation benchmark coverage**

```bash
git add benches/write_operations.rs tests/perf_benchmark_matrix_tests.rs
git commit -m "perf: add milestone mutation benchmarks"
```

## Task 5: Run benchmark pass and refresh perf evidence docs

**Files:**
- Modify: `docs/perf/baseline-hotspots.md`
- Modify: `docs/perf/latency-targets.json`
- Modify: `docs/perf/hotspot-priority.json`
- Create: `docs/perf/evidence-milestone-performance-coverage.md`

- [ ] **Step 1: Run quick Criterion benchmark pass**

Run:

```bash
CARGO_TARGET_DIR=target_latest cargo bench --bench formatters -- --quick
CARGO_TARGET_DIR=target_latest cargo bench --bench command_paths -- --quick
CARGO_TARGET_DIR=target_latest cargo bench --bench write_operations -- --quick
```

Expected: all three bench commands exit 0 and write results under `target_latest/criterion`.

- [ ] **Step 2: Generate hotspot ranking output**

Run:

```bash
cargo run --bin perf-hotspots -- --from target_latest/criterion > /tmp/linear-cli-hotspots.md
```

Expected: `/tmp/linear-cli-hotspots.md` contains a Markdown table headed `# Baseline Hotspots`.

- [ ] **Step 3: Refresh `docs/perf/baseline-hotspots.md`**

Replace `docs/perf/baseline-hotspots.md` with `/tmp/linear-cli-hotspots.md`, then edit the intro to include:

```markdown
Generated from post-milestone-performance-coverage benchmarks (main `<current-short-sha>`, 2026-05-14).

Coverage added in this pass:
- Milestone formatter benchmarks.
- Milestone command parse/handler benchmarks.
- Milestone mutation build and build+serialize benchmarks.
```

Use:

```bash
git rev-parse --short HEAD
```

Expected: file clearly identifies the current benchmark source. The full ranking file contains the measured benchmark rows emitted by `perf-hotspots`.

- [ ] **Step 4: Update latency target metadata and targets**

Open `docs/perf/latency-targets.json` and update:

```json
"generated_from": "target_latest/criterion (post-milestone-performance-coverage, 2026-05-14)",
"generated_at": "2026-05-14T00:00:00Z"
```

Then add exactly one new target for the highest-ranking benchmark row in `docs/perf/baseline-hotspots.md` whose benchmark ID contains `milestone`. Use that row's estimate as both `baseline_ns` and `target_ns`, with `"tolerance_percent": 10.0`. Keep the existing targets unchanged.

Use this command to identify the row:

```bash
rg -n "milestone" docs/perf/baseline-hotspots.md | head -1
```

Expected: the command prints one milestone benchmark row. If it prints nothing, stop because the benchmark coverage did not run correctly.

- [ ] **Step 5: Update hotspot priority JSON**

Update `docs/perf/hotspot-priority.json` to mention this pass:

```json
{
  "description": "Hotspot priority list generated from post-milestone-performance-coverage ranking. Keep empty when no measured hotspot is selected for immediate optimization.",
  "top_hotspots": []
}
```

Keep `top_hotspots` empty in this pass. This plan adds coverage and evidence only; it does not select an optimization.

- [ ] **Step 6: Create evidence note**

Create `docs/perf/evidence-milestone-performance-coverage.md`:

```markdown
# Milestone Performance Coverage Evidence

**Date:** 2026-05-14  
**Branch:** main (`<current-short-sha>`)  
**Benchmarks:** `CARGO_TARGET_DIR=target_latest cargo bench --bench <name> -- --quick`

## Coverage added

- Milestone list formatter benchmarks for JSON, CSV, Markdown, and table.
- Milestone CLI parse benchmarks for list/view/create/update/delete.
- Milestone handler benchmarks for list/view/create/update/delete JSON paths.
- Project milestone mutation build and build+serialize benchmarks.

## Findings

- Record whether top measured hotspots remain table-formatting dominated, based on the refreshed ranking.
- Milestone benchmarks are now covered by the benchmark matrix and regression evidence.

## Action

No production optimization was made in this pass. Future optimization should start from `docs/perf/baseline-hotspots.md` and `docs/perf/latency-targets.json`.
```

Replace `<current-short-sha>` with `git rev-parse --short HEAD` output.

- [ ] **Step 7: Verify perf docs and regression gate**

Run:

```bash
cargo run --bin perf-regression-gate -- \
  --from target_latest/criterion \
  --targets docs/perf/latency-targets.json
```

Expected: PASS. On failure, stop and use the `systematic-debugging` skill to identify the specific stale or missing target before editing the target file.

- [ ] **Step 8: Commit refreshed perf evidence**

```bash
git add docs/perf/baseline-hotspots.md docs/perf/latency-targets.json docs/perf/hotspot-priority.json docs/perf/evidence-milestone-performance-coverage.md
git commit -m "docs(perf): refresh milestone benchmark evidence"
```

## Task 6: Final verification

**Files:**
- Potentially any file changed by formatting or benchmark doc corrections.

- [ ] **Step 1: Run formatter**

Run:

```bash
cargo fmt --all
```

Expected: exits 0.

- [ ] **Step 2: Run benchmark matrix tests**

Run:

```bash
cargo test --test perf_benchmark_matrix_tests -- --nocapture
```

Expected: PASS.

- [ ] **Step 3: Run bench compile check**

Run:

```bash
cargo check --benches
```

Expected: PASS.

- [ ] **Step 4: Run full test suite**

Run:

```bash
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 5: Run clippy using repository-safe command**

Run:

```bash
cargo clippy -p linear-cli --no-deps
```

Expected: PASS. Do not run `cargo clippy --workspace`; repository guidance says it can hang on cynic proc macros.

- [ ] **Step 6: Scan for accidental debug/slop markers**

Run:

```bash
rg -n "dbg!|println!|T[O]DO|F[I]XME|X[X]X|H[A]CK" benches tests src crates/linear-queries/src docs/perf
```

Expected: only intentional existing CLI/bin output matches, no newly introduced debug markers or unresolved task comments.

- [ ] **Step 7: Check final git status and commit cleanup changes**

Run:

```bash
git status --short
```

Expected: either no output or only cleanup changes from Task 6. When cleanup changes exist, run:

```bash
git add -A
git commit -m "chore(perf): clean up milestone benchmark coverage"
```

When there is no output, record in the execution notes that no cleanup commit was needed.

## Out of Scope

- Production formatter optimizations.
- Replacing comfy-table or changing table output style.
- Adding live Linear API performance tests.
- Reworking regression-gate semantics.
