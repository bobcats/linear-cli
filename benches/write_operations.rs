use criterion::{Criterion, black_box, criterion_group, criterion_main};
use cynic::MutationBuilder;
use linear_cli::client::queries::{
    CommentCreateInput, CommentCreateMutation, CommentCreateMutationVariables,
    IssueArchiveMutation, IssueArchiveMutationVariables, IssueCreateInput, IssueCreateMutation,
    IssueCreateMutationVariables, IssueRelationCreateInput, IssueRelationCreateMutation,
    IssueRelationCreateMutationVariables, IssueRelationType, IssueUnarchiveMutation,
    IssueUnarchiveMutationVariables, IssueUpdateInput, IssueUpdateMutation,
    IssueUpdateMutationVariables,
};
use linear_cli::error::CliError;
use linear_cli::issues::resolver::{
    IssueReferenceLookup, IssueReferenceResolver, ResolveIssueRefsInput,
};

fn issue_create_vars() -> IssueCreateMutationVariables {
    IssueCreateMutationVariables {
        input: IssueCreateInput {
            team_id: "team-1".to_string(),
            title: Some("Improve auth flow".to_string()),
            description: Some("Make token refresh more resilient".to_string()),
            assignee_id: Some("user-1".to_string()),
            project_id: Some("project-1".to_string()),
            state_id: Some("state-1".to_string()),
            priority: Some(2),
        },
    }
}

fn issue_update_vars() -> IssueUpdateMutationVariables {
    IssueUpdateMutationVariables {
        id: "issue-123".to_string(),
        input: IssueUpdateInput {
            title: Some("Improve auth flow (updated)".to_string()),
            description: Some("Updated description".to_string()),
            assignee_id: Some("user-2".to_string()),
            project_id: Some("project-2".to_string()),
            state_id: Some("state-2".to_string()),
            priority: Some(1),
        },
    }
}

fn comment_create_vars() -> CommentCreateMutationVariables {
    CommentCreateMutationVariables {
        input: CommentCreateInput {
            issue_id: Some("issue-123".to_string()),
            body: Some("Investigating now".to_string()),
        },
    }
}

fn issue_archive_vars() -> IssueArchiveMutationVariables {
    IssueArchiveMutationVariables {
        id: "issue-123".to_string(),
        trash: Some(false),
    }
}

fn issue_unarchive_vars() -> IssueUnarchiveMutationVariables {
    IssueUnarchiveMutationVariables {
        id: "issue-123".to_string(),
    }
}

fn issue_relation_vars() -> IssueRelationCreateMutationVariables {
    IssueRelationCreateMutationVariables {
        input: IssueRelationCreateInput {
            issue_id: "issue-123".to_string(),
            related_issue_id: "issue-456".to_string(),
            relation_type: IssueRelationType::Related,
        },
    }
}

fn bench_write_mutation_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_mutation_build");

    group.bench_function("issue_create", |b| {
        b.iter(|| black_box(IssueCreateMutation::build(issue_create_vars())))
    });

    group.bench_function("issue_update", |b| {
        b.iter(|| black_box(IssueUpdateMutation::build(issue_update_vars())))
    });

    group.bench_function("comment_create", |b| {
        b.iter(|| black_box(CommentCreateMutation::build(comment_create_vars())))
    });

    group.bench_function("issue_archive", |b| {
        b.iter(|| black_box(IssueArchiveMutation::build(issue_archive_vars())))
    });

    group.bench_function("issue_unarchive", |b| {
        b.iter(|| black_box(IssueUnarchiveMutation::build(issue_unarchive_vars())))
    });

    group.bench_function("issue_relation_create", |b| {
        b.iter(|| black_box(IssueRelationCreateMutation::build(issue_relation_vars())))
    });

    group.finish();
}

fn bench_write_mutation_serialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_mutation_build_and_serialize");

    group.bench_function("issue_create", |b| {
        b.iter(|| {
            let operation = IssueCreateMutation::build(issue_create_vars());
            black_box(serde_json::to_string(&operation).unwrap())
        })
    });

    group.bench_function("issue_update", |b| {
        b.iter(|| {
            let operation = IssueUpdateMutation::build(issue_update_vars());
            black_box(serde_json::to_string(&operation).unwrap())
        })
    });

    group.bench_function("comment_create", |b| {
        b.iter(|| {
            let operation = CommentCreateMutation::build(comment_create_vars());
            black_box(serde_json::to_string(&operation).unwrap())
        })
    });

    group.bench_function("issue_archive", |b| {
        b.iter(|| {
            let operation = IssueArchiveMutation::build(issue_archive_vars());
            black_box(serde_json::to_string(&operation).unwrap())
        })
    });

    group.bench_function("issue_unarchive", |b| {
        b.iter(|| {
            let operation = IssueUnarchiveMutation::build(issue_unarchive_vars());
            black_box(serde_json::to_string(&operation).unwrap())
        })
    });

    group.bench_function("issue_relation_create", |b| {
        b.iter(|| {
            let operation = IssueRelationCreateMutation::build(issue_relation_vars());
            black_box(serde_json::to_string(&operation).unwrap())
        })
    });

    group.finish();
}

struct BenchmarkLookup;

impl IssueReferenceLookup for BenchmarkLookup {
    fn resolve_viewer_id(&self, _token: &str) -> Result<String, CliError> {
        Ok("viewer-id".to_string())
    }

    fn resolve_user_id_by_email(
        &self,
        _token: &str,
        _email: &str,
    ) -> Result<Option<String>, CliError> {
        Ok(Some("user-from-email".to_string()))
    }

    fn resolve_team_id_by_key(&self, _token: &str, _key: &str) -> Result<Option<String>, CliError> {
        Ok(Some("team-from-key".to_string()))
    }

    fn resolve_project_id_by_slug(
        &self,
        _token: &str,
        _slug: &str,
    ) -> Result<Option<String>, CliError> {
        Ok(Some("project-from-slug".to_string()))
    }

    fn resolve_state_id_by_name(
        &self,
        _token: &str,
        _name: &str,
    ) -> Result<Option<String>, CliError> {
        Ok(Some("state-from-name".to_string()))
    }
}

fn bench_issue_reference_resolver(c: &mut Criterion) {
    let lookup = BenchmarkLookup;
    let resolver = IssueReferenceResolver::new(&lookup);

    let friendly_input = ResolveIssueRefsInput {
        team: Some("ENG".to_string()),
        assignee: Some("@me".to_string()),
        project: Some("platform".to_string()),
        state: Some("In Progress".to_string()),
    };

    let passthrough_input = ResolveIssueRefsInput {
        team: Some("11111111-1111-1111-1111-111111111111".to_string()),
        assignee: Some("22222222-2222-2222-2222-222222222222".to_string()),
        project: Some("33333333-3333-3333-3333-333333333333".to_string()),
        state: Some("44444444-4444-4444-4444-444444444444".to_string()),
    };

    let mut group = c.benchmark_group("issue_reference_resolver");

    group.bench_function("friendly_refs", |b| {
        b.iter(|| black_box(resolver.resolve("token", &friendly_input).unwrap()))
    });

    group.bench_function("uuid_passthrough", |b| {
        b.iter(|| black_box(resolver.resolve("token", &passthrough_input).unwrap()))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_write_mutation_build,
    bench_write_mutation_serialize,
    bench_issue_reference_resolver
);
criterion_main!(benches);
