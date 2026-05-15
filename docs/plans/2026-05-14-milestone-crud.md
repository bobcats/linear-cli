# Milestone CRUD Implementation Plan

> REQUIRED: Use the `executing-plans` skill to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add top-level Linear project milestone CRUD commands and issue create/update milestone assignment using strict TDD.

**Architecture:** Add a focused `milestones` domain alongside existing `projects`, `cycles`, and `labels`, backed by a new `MilestoneClient` trait and Linear `ProjectMilestone` GraphQL operations in `crates/linear-queries`. Issue commands keep their existing boundaries but gain a milestone patch value resolved before sending `projectMilestoneId` to issue mutations.

**Tech Stack:** Rust 2024, `clap` derive, `cynic` GraphQL query/mutation derives, `serde`, existing `Formattable` output traits, integration tests under `tests/`.

---

## Source Design

Implement against `docs/design/2026-05-14-milestone-crud.md`.

Key constraints from the design:

- Use top-level `linear milestone ...`, not `linear project milestone ...`.
- Use Linear `ProjectMilestone` APIs, not deprecated roadmap APIs.
- Add `--milestone` to `issue create` and `issue update`.
- Resolve milestone names globally when unique; error on ambiguity with guidance to pass `--project`.
- On issue create/update, issue `--project` scopes milestone name resolution.
- Preserve update tri-state semantics for `projectMilestoneId`: omit, set, clear.
- Do not add bulk milestone issue subcommands or `--with-issues`.

## File Structure

Create:

- `src/milestones/mod.rs` — domain module exports.
- `src/milestones/types.rs` — `Milestone`, `MilestoneProject`, `MilestoneList`, formatter impls, conversion from query nodes.
- `src/milestones/resolver.rs` — milestone reference parsing and resolution.
- `src/milestones/commands/mod.rs` — command exports.
- `src/milestones/commands/list.rs` — list handler.
- `src/milestones/commands/view.rs` — view handler.
- `src/milestones/commands/create.rs` — create handler.
- `src/milestones/commands/update.rs` — update handler.
- `src/milestones/commands/delete.rs` — delete handler.
- `src/client/milestones.rs` — `MilestoneClient`, request structs, production `LinearClient` impl, mock client.
- `tests/milestone_cli_tests.rs` — clap coverage.
- `tests/milestone_formatting_tests.rs` — output formatter coverage.
- `tests/milestone_resolver_tests.rs` — resolver behavior.
- `tests/milestone_command_tests.rs` — handler behavior.
- `tests/milestone_mutation_tests.rs` — GraphQL serialization tests.

Modify:

- `src/lib.rs` — export `milestones` module.
- `src/client/mod.rs` — export `client::milestones`.
- `src/client/reference_lookup.rs` — add production milestone lookup implementation if the resolver uses a lookup trait implemented by `LinearClient`.
- `src/cli.rs` — add `Commands::Milestone`, `MilestoneCommands`, patch args, and issue `--milestone` flags.
- `src/main.rs` — dispatch milestone commands and pass milestone resolution dependencies to issue create/update.
- `crates/linear-queries/src/lib.rs` — add milestone fragments, queries, inputs, and mutations; add `projectMilestone` to issue fragments and `projectMilestoneId` to issue create/update inputs.
- `src/client/issues.rs` — add create/update milestone fields and tri-state update handling.
- `src/issues/resolver.rs` — include raw milestone value in `ResolveIssueRefsInput` and resolved patch output.
- `src/issues/commands/create.rs` — accept and resolve `milestone`.
- `src/issues/commands/update.rs` — accept and resolve `milestone`, including `null` clear.
- `src/issues/types.rs` — add lightweight issue milestone output.
- `README.md` — add milestone examples and status line.
- `benches/command_paths.rs` — update compile-time call sites for issue create/update handler signature changes.
- Existing issue fixture tests with `Issue { ... }` literals — add `milestone: None` unless testing milestone output.

## TDD Rules for This Plan

- [x] Before implementation, read the `test-driven-development` skill and follow RED → GREEN → REFACTOR for every behavior change.
- [ ] Never add production code for a task until its failing test has been written and run.
- [ ] After each GREEN step, run the smallest relevant test first, then the task-level test group.
- [ ] Commit each task after tests pass; do not batch all work into one final commit.

## Task 1: Add CLI command shape tests

**Files:**
- Modify: `tests/cli_structure_tests.rs`
- Modify after RED: `src/cli.rs`

- [x] **Step 1: Write failing clap tests for milestone commands**

Add tests like:

```rust
#[test]
fn test_parse_milestone_list() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("milestone")
        .arg("list")
        .arg("--project")
        .arg("APP")
        .arg("--limit")
        .arg("10")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_milestone_crud() {
    for command in ["view", "create", "update", "delete"] {
        Command::cargo_bin("linear-cli")
            .unwrap()
            .arg("milestone")
            .arg(command)
            .arg("--help")
            .assert()
            .success();
    }
}

#[test]
fn test_parse_issue_create_with_milestone() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("create")
        .arg("--team")
        .arg("ENG")
        .arg("--title")
        .arg("Ship beta")
        .arg("--milestone")
        .arg("Beta")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_issue_update_with_milestone_clear() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("update")
        .arg("ENG-123")
        .arg("--milestone")
        .arg("null")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_milestone_create_rejects_invalid_target_date_shape() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("milestone")
        .arg("create")
        .arg("--project")
        .arg("APP")
        .arg("--name")
        .arg("Beta")
        .arg("--target-date")
        .arg("06/30/2026")
        .assert()
        .failure()
        .stderr(predicate::str::contains("YYYY-MM-DD"));
}

#[test]
fn test_milestone_create_rejects_invalid_calendar_date() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("milestone")
        .arg("create")
        .arg("--project")
        .arg("APP")
        .arg("--name")
        .arg("Beta")
        .arg("--target-date")
        .arg("2026-02-31")
        .assert()
        .failure()
        .stderr(predicate::str::contains("valid calendar date"));
}
```

- [x] **Step 2: Run RED command**

Run:

```bash
cargo test --test cli_structure_tests milestone -- --nocapture
```

Expected: FAIL because `milestone` command and/or `--milestone` flags do not exist.

- [x] **Step 3: Add minimal CLI types**

In `src/cli.rs`:

- Add `Commands::Milestone { action: MilestoneCommands }`.
- Add `MilestoneCommands::{List, View, Create, Update, Delete}`.
- Add `#[arg(long)] milestone: Option<String>` to issue create.
- Add `milestone: Option<String>` to `IssueUpdatePatchArgs` and `has_any_field()`.
- Each milestone subcommand should flatten `FormatFlags`.
- `create` requires `--project` and `--name` through non-optional fields.
- `update` accepts optional `--name`, `--description`, `--target-date`, and `--project`.
- Add a small `parse_timeless_date(value: &str) -> Result<String, String>` helper in `src/cli.rs` and use it as the clap `value_parser` for every `--target-date` flag. It must reject malformed values before handlers run. Keep validation dependency-free: require `YYYY-MM-DD`, digit positions, dash positions, month `01..=12`, and a real calendar day for that month. Implement leap-year handling so `YYYY-02-29` is accepted only when the year is divisible by 4 and not by 100 unless also divisible by 400. Error text should mention `YYYY-MM-DD` for shape failures and `valid calendar date` for impossible dates.

Use `String` after parsing for dates in CLI args; GraphQL conversion can wrap validated strings in `TimelessDate` later.

- [x] **Step 4: Run GREEN command**

Run:

```bash
cargo test --test cli_structure_tests milestone -- --nocapture
```

Expected: PASS for new CLI tests.

- [x] **Step 5: Commit**

```bash
git add src/cli.rs tests/cli_structure_tests.rs
git commit -m "feat(cli): add milestone command shape"
```

## Task 2: Add milestone domain type and formatters

**Files:**
- Create: `src/milestones/mod.rs`
- Create: `src/milestones/types.rs`
- Modify: `src/lib.rs`
- Create: `tests/milestone_formatting_tests.rs`

- [x] **Step 1: Write failing formatter tests**

Create `tests/milestone_formatting_tests.rs` with a sample milestone and assertions for JSON, CSV, Markdown, and table output:

```rust
use linear_cli::milestones::types::{Milestone, MilestoneList, MilestoneProject};
use linear_cli::output::Formattable;

fn sample_milestone() -> Milestone {
    Milestone {
        id: "milestone-1".to_string(),
        name: "Beta".to_string(),
        description: Some("Beta readiness".to_string()),
        status: "next".to_string(),
        progress: 0.5,
        sort_order: 1000.0,
        target_date: Some("2026-06-30".to_string()),
        project: MilestoneProject {
            id: "project-1".to_string(),
            name: "App".to_string(),
            slug_id: "app".to_string(),
        },
        created_at: "2026-05-01T00:00:00Z".to_string(),
        updated_at: "2026-05-02T00:00:00Z".to_string(),
        archived_at: None,
    }
}

#[test]
fn milestone_json_includes_project_and_status() {
    let json = sample_milestone().to_json().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["name"], "Beta");
    assert_eq!(parsed["status"], "next");
    assert_eq!(parsed["project"]["slug_id"], "app");
}

#[test]
fn milestone_csv_includes_core_fields() {
    let csv = sample_milestone().to_csv().unwrap();

    assert!(csv.contains("id,name,project"));
    assert!(csv.contains("milestone-1"));
    assert!(csv.contains("Beta"));
    assert!(csv.contains("App"));
}

#[test]
fn milestone_markdown_includes_target_date() {
    let markdown = sample_milestone().to_markdown().unwrap();

    assert!(markdown.contains("# Beta"));
    assert!(markdown.contains("**Project:** App"));
    assert!(markdown.contains("2026-06-30"));
}

#[test]
fn milestone_list_table_includes_progress() {
    let table = MilestoneList(vec![sample_milestone()]).to_table().unwrap();

    assert!(table.contains("Beta"));
    assert!(table.contains("50%"));
}
```

- [x] **Step 2: Run RED command**

Run:

```bash
cargo test --test milestone_formatting_tests -- --nocapture
```

Expected: FAIL because `linear_cli::milestones` does not exist.

- [x] **Step 3: Implement minimal milestone types and formatters**

Create `src/milestones/mod.rs`:

```rust
pub mod types;
```

Add `pub mod milestones;` to `src/lib.rs`.

Create `src/milestones/types.rs` with:

- `MilestoneProject` derive `Debug, Clone, PartialEq, Eq, Serialize, Deserialize`.
- `Milestone` derive `Debug, Clone, Serialize, Deserialize`.
- `MilestoneList(pub Vec<Milestone>)`.
- `TableFormatter` + `MarkdownFormatter` for `Milestone`.
- `Formattable` impls for `Milestone` and `MilestoneList`.

Follow `src/projects/types.rs` style. Keep list CSV/table compact with columns: `id`, `name`, `project`, `status`, `progress`, `target_date`.

- [x] **Step 4: Run GREEN command**

Run:

```bash
cargo test --test milestone_formatting_tests -- --nocapture
```

Expected: PASS.

- [x] **Step 5: Commit**

```bash
git add src/lib.rs src/milestones tests/milestone_formatting_tests.rs
git commit -m "feat(milestone): add milestone output types"
```

## Task 3: Add milestone GraphQL query and mutation serialization

**Files:**
- Modify: `crates/linear-queries/src/lib.rs`
- Create: `tests/milestone_mutation_tests.rs`

- [x] **Step 1: Write failing GraphQL serialization tests**

Create `tests/milestone_mutation_tests.rs` covering create/update/delete and list variables:

```rust
use cynic::{MutationBuilder, QueryBuilder};
use linear_cli::client::queries::{
    ProjectMilestoneCreateInput, ProjectMilestoneCreateMutation,
    ProjectMilestoneCreateMutationVariables, ProjectMilestoneDeleteMutation,
    ProjectMilestoneDeleteMutationVariables, ProjectMilestoneUpdateInput,
    ProjectMilestoneUpdateMutation, ProjectMilestoneUpdateMutationVariables,
    ProjectMilestonesQuery, ProjectMilestonesQueryVariables,
};

#[test]
fn project_milestone_create_serializes_required_fields() {
    let operation = ProjectMilestoneCreateMutation::build(ProjectMilestoneCreateMutationVariables {
        input: ProjectMilestoneCreateInput {
            project_id: "project-1".to_string(),
            name: "Beta".to_string(),
            description: None,
            target_date: Some("2026-06-30".to_string()),
        },
    });
    let json = serde_json::to_value(&operation).unwrap();
    let input = &json["variables"]["input"];

    assert_eq!(input["projectId"], "project-1");
    assert_eq!(input["name"], "Beta");
    assert_eq!(input["targetDate"], "2026-06-30");
    assert!(input.get("description").is_none());
}

#[test]
fn project_milestone_update_omits_unset_patch_fields() {
    let operation = ProjectMilestoneUpdateMutation::build(ProjectMilestoneUpdateMutationVariables {
        id: "milestone-1".to_string(),
        input: ProjectMilestoneUpdateInput {
            name: None,
            description: Some("Updated".to_string()),
            project_id: None,
            target_date: None,
        },
    });
    let json = serde_json::to_value(&operation).unwrap();
    let input = &json["variables"]["input"];

    assert_eq!(json["variables"]["id"], "milestone-1");
    assert_eq!(input["description"], "Updated");
    assert!(input.get("name").is_none());
    assert!(input.get("projectId").is_none());
    assert!(input.get("targetDate").is_none());
}

#[test]
fn project_milestone_delete_serializes_id() {
    let operation = ProjectMilestoneDeleteMutation::build(ProjectMilestoneDeleteMutationVariables {
        id: "milestone-1".to_string(),
    });
    let json = serde_json::to_value(&operation).unwrap();

    assert_eq!(json["variables"]["id"], "milestone-1");
}

#[test]
fn project_milestones_query_serializes_limit() {
    let operation = ProjectMilestonesQuery::build(ProjectMilestonesQueryVariables {
        first: Some(25),
        name: None,
    });
    let json = serde_json::to_value(&operation).unwrap();

    assert_eq!(json["variables"]["first"], 25);
}
```

- [x] **Step 2: Run RED command**

Run:

```bash
cargo test --test milestone_mutation_tests -- --nocapture
```

Expected: FAIL because milestone query types are missing.

- [x] **Step 3: Add milestone query types**

In `crates/linear-queries/src/lib.rs`, near project types, add:

- `ProjectMilestoneStatus` enum with cynic renames: `done`, `next`, `overdue`, `unstarted`.
- `ProjectMilestoneProject` fragment for `Project { id name slugId }`.
- `ProjectMilestoneNode` fragment for `ProjectMilestone` fields in the design.
- `ProjectMilestoneConnection`.
- `ProjectMilestoneQuery` + variables for direct `projectMilestone(id)`.
- `ProjectMilestonesQuery` + variables for `projectMilestones(first, filter: { name: { eq: $name } })`.
- `ProjectMilestonesForProjectQuery` + variables for `project(id) { projectMilestones(first, filter: { name: { eq: $name } }) }`.
- `ProjectMilestoneCreateInput`, `ProjectMilestoneUpdateInput`, payload, and create/update/delete mutation structs.

Important details:

- Use `TimelessDate` for `target_date` if cynic accepts `TimelessDate`; otherwise use `String` only if the existing scalar mapping permits it. Prefer `TimelessDate` and convert at the client boundary.
- Add `#[cynic(skip_serializing_if = "Option::is_none")]` to optional input fields that should be omitted.
- Reuse existing `DeletePayload` for delete.

- [x] **Step 4: Add conversion from query node to domain type**

In `src/milestones/types.rs`, add `impl From<crate::client::queries::ProjectMilestoneNode> for Milestone` converting:

- `node.id.inner().to_string()`
- `target_date.map(|d| d.0)`
- `archived_at.map(|d| d.0)`
- status enum to lowercase string.

- [x] **Step 5: Run GREEN command**

Run:

```bash
cargo test --test milestone_mutation_tests -- --nocapture
cargo test --test milestone_formatting_tests -- --nocapture
```

Expected: PASS.

- [x] **Step 6: Commit**

```bash
git add crates/linear-queries/src/lib.rs src/milestones/types.rs tests/milestone_mutation_tests.rs
git commit -m "feat(milestone): add GraphQL operations"
```

## Task 4: Add milestone client and command handlers

**Files:**
- Create: `src/client/milestones.rs`
- Modify: `src/client/mod.rs`
- Create: `src/milestones/commands/mod.rs`
- Create: `src/milestones/commands/list.rs`
- Create: `src/milestones/commands/view.rs`
- Create: `src/milestones/commands/create.rs`
- Create: `src/milestones/commands/update.rs`
- Create: `src/milestones/commands/delete.rs`
- Modify: `src/milestones/mod.rs`
- Create: `tests/milestone_command_tests.rs`

- [x] **Step 1: Write failing handler tests**

Create `tests/milestone_command_tests.rs` with local `TestConfigProvider`, `MockStorage`, and `CapturingIo` copied from project/issue command tests. Add tests for:

- list prints milestone JSON/table from mock client;
- view prints one milestone;
- create passes project ID/name/description/target date to mock client;
- update errors when no patch fields are provided;
- delete prints JSON success payload when JSON format selected.

Use a mock client that records inputs:

```rust
#[derive(Clone, Default, Debug, PartialEq)]
struct RecordedCreate {
    project_id: String,
    name: String,
    description: Option<String>,
    target_date: Option<String>,
}
```

Assert the recorded create/update input, not only printed output.

- [x] **Step 2: Run RED command**

Run:

```bash
cargo test --test milestone_command_tests -- --nocapture
```

Expected: FAIL because client and handlers do not exist.

- [x] **Step 3: Implement `MilestoneClient`**

Create `src/client/milestones.rs`:

- `CreateMilestoneInput { project_id, name, description, target_date }`
- `UpdateMilestoneInput { name, description, project_id, target_date }`
- `MilestoneClient` trait:
  - `get_milestone(token, id) -> Result<Milestone, CliError>`
  - `list_milestones(token, project_id: Option<&str>, name: Option<&str>, limit) -> Result<Vec<Milestone>, CliError>`
  - `create_milestone(token, input) -> Result<Milestone, CliError>`
  - `update_milestone(token, id, input) -> Result<Milestone, CliError>`
  - `delete_milestone(token, id) -> Result<(), CliError>`
- Production `impl MilestoneClient for LinearClient` using query types from Task 3.
- `MockMilestoneClient` for simple tests if useful.

Add `pub mod milestones;` to `src/client/mod.rs`.

- [x] **Step 4: Implement handlers**

Implement handlers mirroring project and issue handler patterns:

- Fetch token with `get_token_with_provider`.
- Determine format with `get_format_with_provider`.
- JSON fast path may use direct `serde_json::to_vec` like project handlers, but keep it simple unless tests require provider-driven pretty style.
- `update` validates at least one patch field.
- `delete` prints `{"deleted": true, "id": "..."}` for JSON and `Deleted milestone <id>` otherwise.

At this task stage, handlers can accept already-resolved project/milestone IDs. Name resolution is added in the resolver task.

- [x] **Step 5: Run GREEN command**

Run:

```bash
cargo test --test milestone_command_tests -- --nocapture
cargo test --test milestone_mutation_tests -- --nocapture
```

Expected: PASS.

- [x] **Step 6: Commit**

```bash
git add src/client/mod.rs src/client/milestones.rs src/milestones tests/milestone_command_tests.rs
git commit -m "feat(milestone): add client and handlers"
```

## Task 5: Add milestone reference resolver

**Files:**
- Create: `src/milestones/resolver.rs`
- Modify: `src/milestones/mod.rs`
- Create: `tests/milestone_resolver_tests.rs`
- Modify: `src/client/reference_lookup.rs`

- [x] **Step 1: Write failing resolver tests**

Create `tests/milestone_resolver_tests.rs` with a mock lookup and tests for:

- UUID resolves via direct lookup.
- Linear URL parses final segment and resolves direct lookup.
- Global unique name resolves.
- Scoped name uses project ID and resolves.
- Missing name returns `CliError::NotFound`.
- Ambiguous global name returns `CliError::InvalidArgs` and includes `--project` plus project names.
- `null` returns a clear patch only when `allow_null_clear` is true.
- `null` returns `InvalidArgs` when used for CRUD target resolution.

Use types like:

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResolvedMilestoneRef {
    Unchanged,
    Set(String),
    Clear,
}
```

- [x] **Step 2: Run RED command**

Run:

```bash
cargo test --test milestone_resolver_tests -- --nocapture
```

Expected: FAIL because resolver does not exist.

- [x] **Step 3: Implement resolver interfaces**

In `src/milestones/resolver.rs`, add:

- `MilestoneReferenceLookup` trait:
  - `get_milestone_by_id(token, id) -> Result<Option<Milestone>, CliError>`
  - `find_milestones_by_name(token, name, project_id: Option<&str>) -> Result<Vec<Milestone>, CliError>`
  - `resolve_project_id_by_slug(token, slug) -> Result<Option<String>, CliError>` or reuse the existing issue lookup method through a shared trait if cleaner.
- `MilestoneReferenceResolver`.
- `ResolveMilestoneInput { reference: Option<String>, project: Option<String>, allow_null_clear: bool }`.
- `ResolvedMilestonePatch` enum: `Unchanged`, `Set(String)`, `Clear`.
- `resolve_patch()` for issue create/update.
- `resolve_required_id()` for milestone CRUD commands.

Keep UUID detection local or move to a shared helper only if it avoids duplication without broad refactoring.

- [x] **Step 4: Implement production lookup**

In `src/client/reference_lookup.rs`, implement `MilestoneReferenceLookup for LinearClient` using `MilestoneClient` methods:

- Direct ID: `get_milestone`, returning `Ok(None)` on `CliError::NotFound`.
- Name: `list_milestones(token, project_id, Some(name), 250)`.
- Project slug: existing `list_projects` lookup by `slug_id`.

- [x] **Step 5: Run GREEN command**

Run:

```bash
cargo test --test milestone_resolver_tests -- --nocapture
```

Expected: PASS.

- [x] **Step 6: Commit**

```bash
git add src/milestones/resolver.rs src/milestones/mod.rs src/client/reference_lookup.rs tests/milestone_resolver_tests.rs
git commit -m "feat(milestone): resolve milestone references"
```

## Task 6: Wire milestone CRUD handlers through main dispatch

**Files:**
- Modify: `src/main.rs`
- Modify: `src/milestones/commands/*.rs` if handler signatures need lookup/project resolver dependencies
- Modify: `tests/milestone_command_tests.rs`

- [x] **Step 1: Add failing handler tests for name resolution paths**

Extend `tests/milestone_command_tests.rs` so handlers take user-facing refs and resolve them:

- `handle_view("Beta", Some("APP"), ...)` resolves project slug then milestone name.
- `handle_delete("Beta", Some("APP"), ...)` resolves before deleting.
- `handle_create("APP", ...)` resolves project slug before creating.
- Ambiguous milestone name propagates `InvalidArgs`.

- [x] **Step 2: Run RED command**

Run:

```bash
cargo test --test milestone_command_tests -- --nocapture
```

Expected: FAIL because handlers either expect raw IDs or do not accept lookup dependencies.

- [x] **Step 3: Update milestone handlers to use resolver**

Adjust handler signatures to accept `&dyn MilestoneReferenceLookup` when they need project/milestone resolution.

Expected behavior:

- `list`: resolve `--project` to project ID if supplied; call `list_milestones(token, Some(project_id), None, limit)`.
- `view`: resolve required milestone ID from ref and optional project; then call `get_milestone`.
- `create`: resolve project ref; call `create_milestone`.
- `update`: resolve milestone ref with optional project; resolve new `--project` if supplied; require at least one update field.
- `delete`: resolve milestone ref with optional project; call `delete_milestone`.

- [x] **Step 4: Add `main.rs` dispatch**

In `src/main.rs`:

- Import `MilestoneCommands`, `MilestoneClient`, milestone handlers, and `MilestoneReferenceLookup`.
- Add `Commands::Milestone` match arm.
- Instantiate storage/config/io/client consistently with other resource arms.
- Pass `&client as &dyn MilestoneClient` and `&client as &dyn MilestoneReferenceLookup`.

- [x] **Step 5: Run GREEN command**

Run:

```bash
cargo test --test milestone_command_tests -- --nocapture
cargo check
```

Expected: PASS.

- [x] **Step 6: Commit**

```bash
git add src/main.rs src/milestones/commands tests/milestone_command_tests.rs
git commit -m "feat(milestone): wire milestone commands"
```

## Task 7: Add issue milestone output

**Files:**
- Modify: `crates/linear-queries/src/lib.rs`
- Modify: `src/issues/types.rs`
- Modify: issue tests with `Issue { ... }` literals
- Modify: `tests/issue_formatting_tests.rs`

- [x] **Step 1: Write failing issue formatter test**

In `tests/issue_formatting_tests.rs`, add:

```rust
#[test]
fn test_issue_with_milestone_shows_milestone_name() {
    use linear_cli::issues::types::IssueMilestone;

    let mut issue = create_test_issue_full();
    issue.milestone = Some(IssueMilestone {
        id: "milestone-1".to_string(),
        name: "Beta".to_string(),
        target_date: Some("2026-06-30".to_string()),
    });

    let table = issue.to_table().unwrap();
    assert!(table.contains("Milestone"));
    assert!(table.contains("Beta"));

    let md = issue.to_markdown().unwrap();
    assert!(md.contains("**Milestone:** Beta"));

    let json = issue.to_json().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["milestone"]["name"], "Beta");
}
```

- [x] **Step 2: Run RED command**

Run:

```bash
cargo test --test issue_formatting_tests test_issue_with_milestone_shows_milestone_name -- --nocapture
```

Expected: FAIL because `IssueMilestone` and `Issue::milestone` do not exist.

- [x] **Step 3: Add issue milestone domain type and formatting**

In `src/issues/types.rs`:

- Add `IssueMilestone { id, name, target_date }`.
- Add `pub milestone: Option<IssueMilestone>` to `Issue` with `#[serde(skip_serializing_if = "Option::is_none")]`.
- Update `TableFormatter`, Markdown, CSV, and list CSV/table to include milestone. For CSV/list use `milestone_name` and `milestone_id`.
- Update all `Issue { ... }` literals in tests to include `milestone: None`.

- [x] **Step 4: Add issue GraphQL fragment field and conversion**

In `crates/linear-queries/src/lib.rs`:

- Add lightweight `IssueProjectMilestone` fragment for `ProjectMilestone { id name targetDate }`.
- Add `#[cynic(rename = "projectMilestone")] pub project_milestone: Option<IssueProjectMilestone>` to `IssueNode` and `SearchIssueNode` if search returns issue output.

In `src/issues/types.rs` conversion from query nodes:

- Map `project_milestone` to `IssueMilestone`.

- [x] **Step 5: Run GREEN command**

Run:

```bash
cargo test --test issue_formatting_tests -- --nocapture
cargo test --test issue_list_formatting_tests -- --nocapture
cargo test --test issue_search_formatting_tests -- --nocapture
```

Expected: PASS.

- [x] **Step 6: Commit**

```bash
git add crates/linear-queries/src/lib.rs src/issues/types.rs tests
git commit -m "feat(issue): show milestone in issue output"
```

## Task 8: Add issue create `--milestone` assignment

**Files:**
- Modify: `src/issues/resolver.rs`
- Modify: `src/issues/commands/create.rs`
- Modify: `src/client/issues.rs`
- Modify: `crates/linear-queries/src/lib.rs`
- Modify: `src/main.rs`
- Modify: `tests/issue_create_command_tests.rs`
- Modify: `tests/issue_create_mutation_tests.rs`
- Modify: `benches/command_paths.rs`

- [x] **Step 1: Write failing create mutation test**

In `tests/issue_create_mutation_tests.rs`, add:

```rust
#[test]
fn test_issue_create_mutation_serializes_project_milestone_id() {
    let operation = IssueCreateMutation::build(IssueCreateMutationVariables {
        input: IssueCreateInput {
            team_id: "team-123".to_string(),
            title: Some("Ship beta".to_string()),
            description: None,
            assignee_id: None,
            project_id: Some("project-1".to_string()),
            state_id: None,
            priority: None,
            parent_id: None,
            project_milestone_id: Some("milestone-1".to_string()),
        },
    });
    let json = serde_json::to_value(&operation).unwrap();
    let input = &json["variables"]["input"];

    assert_eq!(input["projectMilestoneId"], "milestone-1");
}
```

Update existing `IssueCreateInput` literals in this file with `project_milestone_id: None`.

- [x] **Step 2: Run RED command**

Run:

```bash
cargo test --test issue_create_mutation_tests test_issue_create_mutation_serializes_project_milestone_id -- --nocapture
```

Expected: FAIL because `project_milestone_id` is missing.

- [x] **Step 3: Add create mutation field**

In `crates/linear-queries/src/lib.rs`, add to `IssueCreateInput`:

```rust
#[cynic(rename = "projectMilestoneId", skip_serializing_if = "Option::is_none")]
pub project_milestone_id: Option<String>,
```

In `src/client/issues.rs`, add `project_milestone_id: Option<String>` to public `CreateIssueInput` and pass it through to the query input.

- [x] **Step 4: Run mutation GREEN command**

Run:

```bash
cargo test --test issue_create_mutation_tests -- --nocapture
```

Expected: PASS.

- [x] **Step 5: Write failing create handler test**

In `tests/issue_create_command_tests.rs`, add a mock that records `CreateIssueInput`, and assert:

- raw `--project app --milestone Beta` resolves project first;
- milestone lookup is scoped to the resolved project;
- client receives `project_milestone_id: Some("milestone-1")`.

- [x] **Step 6: Run RED command**

Run:

```bash
cargo test --test issue_create_command_tests milestone -- --nocapture
```

Expected: FAIL because handler signature/logic does not resolve milestone.

- [x] **Step 7: Update create resolution flow**

In `src/issues/resolver.rs`:

- Add `milestone: Option<String>` to `ResolveIssueRefsInput`.
- Add `project_milestone_id: Option<String>` to `ResolvedIssueRefs` for create set-only behavior.
- Make `IssueReferenceResolver` also able to use `MilestoneReferenceLookup`, or keep existing resolver for existing refs and call `MilestoneReferenceResolver` from `handle_create`. Prefer the latter to avoid bloating `IssueReferenceLookup`.

In `src/issues/commands/create.rs`:

- Add `milestone: Option<String>` and `milestone_lookup: &dyn MilestoneReferenceLookup` parameters.
- Clone the raw issue project argument before moving it into the existing issue reference resolver: `let raw_project_ref = project.clone();`.
- Resolve existing issue refs first to get `resolved.project_id`.
- Use milestone scope in this order:
  - if `resolved.project_id` is `Some`, pass that project ID as the milestone scope;
  - else if `raw_project_ref` is `Some`, resolve it through the milestone/project lookup before milestone name lookup;
  - else do global milestone name resolution.
- Resolve milestone with that explicit `Option<String>` scope and `allow_null_clear: false`.
- Pass `project_milestone_id` to `CreateIssueInput`.

In `src/main.rs`, pass `milestone` from CLI and `&client as &dyn MilestoneReferenceLookup`.

Update `benches/command_paths.rs` and all handler test call sites for the new parameter.

- [x] **Step 8: Run GREEN command**

Run:

```bash
cargo test --test issue_create_command_tests -- --nocapture
cargo check
```

Expected: PASS.

- [x] **Step 9: Commit**

```bash
git add crates/linear-queries/src/lib.rs src/client/issues.rs src/issues/commands/create.rs src/issues/resolver.rs src/main.rs tests/issue_create_* benches/command_paths.rs
git commit -m "feat(issue): assign milestone on create"
```

## Task 9: Add issue update `--milestone` tri-state assignment and clearing

**Files:**
- Modify: `src/client/issues.rs`
- Modify: `crates/linear-queries/src/lib.rs`
- Modify: `src/issues/commands/update.rs`
- Modify: `src/main.rs`
- Modify: `tests/issue_update_mutation_tests.rs`
- Modify: `tests/issue_update_command_tests.rs`
- Modify: `benches/command_paths.rs`

- [x] **Step 1: Write failing update mutation tests for set, omit, and clear**

In `tests/issue_update_mutation_tests.rs`, add tests:

- set serializes `projectMilestoneId: "milestone-1"`;
- unchanged omits `projectMilestoneId`;
- clear serializes `projectMilestoneId: null`.

Use the public API chosen for tri-state. Target shape in root crate:

```rust
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum IssueFieldPatch<T> {
    #[default]
    Unchanged,
    Set(T),
    Clear,
}
```

If `IssueUpdateInput` in `crates/linear-queries` cannot derive `cynic::InputObject` with a tri-state field, implement manual `cynic::InputObject` + `cynic::serde::Serialize` for that query input instead of weakening the tri-state requirement.

- [x] **Step 2: Run RED command**

Run:

```bash
cargo test --test issue_update_mutation_tests project_milestone -- --nocapture
```

Expected: FAIL because update tri-state is missing.

- [x] **Step 3: Implement tri-state update serialization**

In `src/client/issues.rs`:

- Add `IssueFieldPatch<T>`.
- Add `project_milestone_id: IssueFieldPatch<String>` to public `UpdateIssueInput`.
- Convert resolver output to this patch.

In `crates/linear-queries/src/lib.rs`:

- Preserve omit/set/clear at the GraphQL variable serialization layer.
- Do not represent update milestone as a plain skipped `Option<String>` because it cannot serialize clear.
- If manual serialization is needed, serialize existing update fields exactly as before and add:
  - no map entry for `Unchanged`;
  - string entry for `Set(id)`;
  - null entry for `Clear`.

Keep all other optional update fields skip-on-none.

- [x] **Step 4: Run mutation GREEN command**

Run:

```bash
cargo test --test issue_update_mutation_tests -- --nocapture
```

Expected: PASS.

- [x] **Step 5: Write failing update handler tests**

In `tests/issue_update_command_tests.rs`, add tests:

- `--milestone Beta --project app` sends `IssueFieldPatch::Set("milestone-1")`.
- `--milestone null` sends `IssueFieldPatch::Clear` and satisfies the “at least one patch field” validation.
- no `--milestone` sends `IssueFieldPatch::Unchanged`.

- [x] **Step 6: Run RED command**

Run:

```bash
cargo test --test issue_update_command_tests milestone -- --nocapture
```

Expected: FAIL because update handler does not accept or resolve milestone.

- [x] **Step 7: Update handler and dispatch**

In `src/issues/commands/update.rs`:

- Add `milestone: Option<String>` and `milestone_lookup: &dyn MilestoneReferenceLookup` parameters.
- Include `milestone.is_some()` in patch validation.
- Clone the raw issue project argument before moving it into the existing issue reference resolver: `let raw_project_ref = project.clone();`.
- Resolve existing refs first.
- Use the same explicit milestone scope order as create: resolved project ID first, otherwise resolved raw project ref, otherwise global.
- Resolve milestone patch with `allow_null_clear: true`.
- Send tri-state patch to `UpdateIssueInput`.

In `src/main.rs`:

- Pass `patch.milestone` and `&client as &dyn MilestoneReferenceLookup`.

Update `benches/command_paths.rs` and test call sites.

- [x] **Step 8: Run GREEN command**

Run:

```bash
cargo test --test issue_update_command_tests -- --nocapture
cargo check
```

Expected: PASS.

- [x] **Step 9: Commit**

```bash
git add crates/linear-queries/src/lib.rs src/client/issues.rs src/issues/commands/update.rs src/main.rs tests/issue_update_* benches/command_paths.rs
git commit -m "feat(issue): update issue milestones"
```

## Task 10: Add full CLI integration and docs

**Files:**
- Modify: `README.md`
- Modify: `ROADMAP.md` if status text needs parity update
- Modify: `tests/cli_structure_tests.rs`
- Modify: any remaining compile-failing tests from changed structs/signatures

- [ ] **Step 1: Write failing README/status expectation if existing docs tests exist**

Search for docs tests:

```bash
rg -n "README|Quick Start|Status" tests
```

If no docs tests exist, make this a documentation-only RED by running:

```bash
rg -n "milestone" README.md ROADMAP.md
```

Expected: no milestone mentions before docs update.

- [ ] **Step 2: Update README examples**

In `README.md`:

- Add milestones to Highlights/Status.
- Add quick-start examples:

```fish
linear-cli milestone list --project APP
linear-cli milestone create --project APP --name "Beta" --target-date 2026-06-30
linear-cli issue update ENG-123 --milestone "Beta"
```

In `ROADMAP.md`, update current state to include milestone CRUD if the feature is complete.

- [ ] **Step 3: Run docs/check command**

Run:

```bash
rg -n "milestone" README.md ROADMAP.md
```

Expected: output includes the new examples/status lines.

- [ ] **Step 4: Run focused compile/test sweep**

Run:

```bash
cargo test --test cli_structure_tests -- --nocapture
cargo test --test milestone_formatting_tests -- --nocapture
cargo test --test milestone_resolver_tests -- --nocapture
cargo test --test milestone_command_tests -- --nocapture
cargo test --test milestone_mutation_tests -- --nocapture
cargo test --test issue_create_command_tests -- --nocapture
cargo test --test issue_update_command_tests -- --nocapture
cargo test --test issue_create_mutation_tests -- --nocapture
cargo test --test issue_update_mutation_tests -- --nocapture
cargo test --test issue_formatting_tests -- --nocapture
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add README.md ROADMAP.md tests src crates benches
git commit -m "docs: document milestone workflows"
```

## Task 11: Final verification and cleanup

**Files:**
- Potentially any file touched by formatting or final fixes.

- [ ] **Step 1: Run formatter**

Run:

```bash
cargo fmt --all
```

Expected: command exits 0.

- [ ] **Step 2: Run full workspace tests**

Run:

```bash
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 3: Run check**

Run:

```bash
cargo check
```

Expected: PASS.

- [ ] **Step 4: Run clippy with project gotcha command**

Run:

```bash
cargo clippy -p linear-cli --no-deps
```

Expected: PASS. Do not run `cargo clippy --workspace`; repository guidance says it can hang on cynic proc macros.

- [ ] **Step 5: Scan for accidental debug/slop markers**

Run:

```bash
rg -n "dbg!|println!|T[O]DO|F[I]XME|X[X]X|H[A]CK" src tests crates/linear-queries/src README.md ROADMAP.md
```

Expected: no accidental debug output or unresolved task markers introduced by this work.

- [ ] **Step 6: Commit final formatting/fixes if needed**

If `cargo fmt` or cleanup changed files:

```bash
git add -A
git commit -m "chore: clean up milestone implementation"
```

If there are no changes, do not create an empty commit.

## Follow-ups

These are intentionally not executable tasks in this plan:

- `milestone view --with-issues` with pagination.
- Bulk `milestone issue add/remove/list` commands.
- `issue list --milestone` filtering.
- Milestone move support using Linear's internal `projectMilestoneMove` mutation.
