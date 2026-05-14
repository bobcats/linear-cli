# Milestone CRUD and Issue Assignment

## Summary

Add top-level Linear project milestone support to `linear-cli`, including full CRUD commands and issue create/update integration. Milestones map to Linear's `ProjectMilestone` GraphQL resource, not deprecated roadmap APIs.

## Goals

- Provide automation-friendly CRUD for project milestones.
- Keep command patterns consistent with existing resources such as projects, cycles, labels, and issues.
- Let users assign or clear an issue's milestone through normal issue create/update workflows.
- Preserve deterministic, machine-readable output across JSON, CSV, Markdown, and table formats.

## Non-goals

- Bulk milestone issue management commands such as `milestone issue add/remove/list`.
- Embedding or paginating all milestone issues in `milestone view`.
- Deprecated roadmap support.
- Interactive disambiguation prompts.

## CLI Shape

Milestones are a top-level resource:

```fish
linear milestone list [--project <project-ref>] [--limit N]
linear milestone view <milestone-ref> [--project <project-ref>]
linear milestone create --project <project-ref> --name "Beta" [--description "..."] [--target-date YYYY-MM-DD]
linear milestone update <milestone-ref> [--project <project-ref>] [--name "..."] [--description "..."] [--target-date YYYY-MM-DD]
linear milestone delete <milestone-ref> [--project <project-ref>]
```

Issue commands gain milestone assignment:

```fish
linear issue create --team ENG --title "Ship beta" --project APP --milestone "Beta"
linear issue update ENG-123 --milestone "Beta"
linear issue update ENG-123 --milestone null
```

`--milestone null` clears an issue milestone on update. `null` is not accepted as a milestone target for CRUD commands.

## Resource Boundaries

Add a new milestone domain module with the same broad shape as existing resource modules:

- `src/milestones/types.rs` for domain types and output formatters.
- `src/milestones/commands/` for command handlers.
- `src/client/milestones.rs` for the client trait and production implementation.
- `crates/linear-queries/src/lib.rs` additions for milestone GraphQL queries, inputs, and mutations.

Issue create/update remain issue commands. They should only consume milestone resolution output so they can send `projectMilestoneId` in existing issue mutations.

## Data Model and Output

The milestone domain type should expose:

- `id`
- `name`
- `description`
- `status` (`unstarted`, `next`, `overdue`, `done`)
- `progress`
- `sort_order`
- `target_date`
- `project` (`id`, `name`, `slug_id`)
- `created_at`
- `updated_at`
- `archived_at`

All milestone commands support standard output flags: `--json`, `--csv`, `--markdown`, and `--table`.

Issue output gains a lightweight nested milestone field when present:

```json
"milestone": {
  "id": "...",
  "name": "Beta",
  "target_date": "2026-06-30"
}
```

For compact/list-oriented formats, expose `milestone_name` and `milestone_id` columns where appropriate. Use null/empty fields when no milestone is assigned.

## Reference Resolution

Add milestone reference resolution with a lookup trait implemented by `LinearClient`.

Rules:

1. UUIDs are treated as direct milestone IDs and resolved with `projectMilestone(id)`.
2. Linear milestone URLs are accepted by parsing the URL's final identifier segment, then attempting direct lookup with that identifier.
3. Non-ID, non-URL references are treated as milestone names.
4. Names resolve globally if exactly one milestone matches.
5. If `--project` is supplied, name search is scoped to that project.
6. On `issue create` and `issue update`, the issue command's `--project` value also scopes `--milestone` name resolution. If the issue project is being set and the milestone is a name, resolve the project first, then resolve the milestone within that project.
7. Zero matches return `NotFound`.
8. Multiple matches return `InvalidArgs` with matching project names and an instruction to pass `--project`.
9. `null` is accepted only for `issue update --milestone null` to clear the field.

Milestone name ambiguity is expected because names are only naturally unique within a project. The CLI should stay non-interactive and report ambiguity instead of prompting.

Project references for milestone CRUD should use existing project reference conventions where possible. If a project lookup helper is not already reusable, introduce a small shared resolver instead of duplicating raw lookup logic in handlers.

## GraphQL Operations

Use Linear `ProjectMilestone` APIs:

- `projectMilestones(first, filter)` for list/search.
- `projectMilestone(id)` for view and direct lookup.
- `projectMilestoneCreate(input)` with `projectId`, `name`, optional `description`, optional `targetDate`.
- `projectMilestoneUpdate(id, input)` with only provided patch fields.
- `projectMilestoneDelete(id)` for delete.

Issue create/update inputs gain `projectMilestoneId` support.

Create has two states for this field: set a resolved milestone ID, or omit the field.

Update has three states and the implementation must preserve them explicitly:

- leave unchanged: omit `projectMilestoneId` from the mutation input;
- set: send the resolved milestone ID;
- clear: send the API-supported clear value for `projectMilestoneId`.

The root crate should model this as an explicit patch/tri-state value before converting to the `cynic::InputObject`; a plain `Option<String>` is not enough for update because it cannot distinguish “clear” from “unchanged.” Because optional `cynic::InputObject` fields serialize as explicit `null` unless configured otherwise, unset optional mutation fields must use `#[cynic(skip_serializing_if = "Option::is_none")]`, and the clear case must be intentionally represented rather than accidentally produced by an absent CLI argument.

## Error Handling

- `milestone update` requires at least one patch field.
- `milestone create` requires `--project` and `--name`.
- Invalid dates should fail argument parsing or handler validation before GraphQL submission.
- Ambiguous milestone references should include enough context for the user to resolve them, especially project names.
- Delete output should match existing delete conventions: structured JSON when JSON is selected, concise human text otherwise.

## Testing Strategy

Add coverage at the same layers used by existing resources:

- CLI structure tests for `milestone` commands and issue `--milestone` flags.
- Command handler tests using mock milestone clients/resolvers.
- Query and mutation serialization tests for milestone CRUD and issue `projectMilestoneId` fields.
- Formatter tests for milestone single/list output.
- Updated issue formatter tests for the new lightweight milestone field.
- Resolver tests covering direct ID, scoped name, global unique name, no matches, ambiguous matches, and `null` clearing behavior.

Verification before implementation completion should include:

```fish
cargo test --workspace
cargo check
cargo fmt --all
cargo clippy -p linear-cli --no-deps
```

## Open Follow-ups

These are intentionally out of scope for the first implementation:

- `milestone view --with-issues` with pagination.
- Bulk issue assignment/removal commands.
- `issue list --milestone` filtering.
- Milestone move support using Linear's internal `projectMilestoneMove` mutation.
