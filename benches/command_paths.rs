#![allow(clippy::unit_arg)]

use clap::Parser;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use linear_cli::auth::UserInfo;
use linear_cli::auth::config::TestConfigProvider;
use linear_cli::auth::storage::MockTokenStorage;
use linear_cli::cli::Cli;
use linear_cli::client::comments::{CommentClient, CreateCommentInput};
use linear_cli::client::cycles::CycleClient;
use linear_cli::client::issues::{
    CreateIssueInput, CreateIssueRelationInput, IssueClient, UpdateIssueInput,
};
use linear_cli::client::projects::ProjectClient;
use linear_cli::client::teams::TeamClient;
use linear_cli::comments::types::Comment;
use linear_cli::cycles::commands::{
    handle_current as handle_cycle_current, handle_list as handle_cycle_list,
    handle_view as handle_cycle_view,
};
use linear_cli::cycles::types::Cycle;
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::issues::commands::{
    handle_archive, handle_block, handle_comment_add, handle_create, handle_duplicate, handle_link,
    handle_unarchive, handle_update,
};
use linear_cli::issues::types::{Issue, IssueState, Priority, User};
use linear_cli::output::OutputFormat;
use linear_cli::projects::commands::{
    handle_list as handle_project_list, handle_view as handle_project_view,
};
use linear_cli::projects::types::Project;
use linear_cli::teams::commands::{
    handle_list as handle_team_list, handle_view as handle_team_view,
};
use linear_cli::teams::types::Team;

struct NoopIo;

impl Io for NoopIo {
    fn read_secret(&self, _prompt: &str) -> Result<String, CliError> {
        Ok(String::new())
    }

    fn print(&self, _message: &str) {}

    fn print_error(&self, _message: &str) {}
}

fn benchmark_storage() -> MockTokenStorage {
    MockTokenStorage {
        token: Some("token".to_string()),
        user_info: Some(UserInfo {
            id: "user-1".to_string(),
            name: "Engineer".to_string(),
            email: "engineer@example.com".to_string(),
        }),
    }
}

fn benchmark_config() -> TestConfigProvider {
    TestConfigProvider {
        values: std::collections::HashMap::new(),
    }
}

struct BenchIssueClient {
    issue: Issue,
}

impl IssueClient for BenchIssueClient {
    fn get_issue(&self, _token: &str, _identifier: &str) -> Result<Issue, CliError> {
        Ok(self.issue.clone())
    }

    fn list_issues(
        &self,
        _token: &str,
        _assignee: Option<String>,
        _project: Option<String>,
        _limit: usize,
    ) -> Result<Vec<Issue>, CliError> {
        Ok(vec![self.issue.clone()])
    }

    fn create_issue(&self, _token: &str, _input: CreateIssueInput) -> Result<Issue, CliError> {
        Ok(self.issue.clone())
    }

    fn update_issue(
        &self,
        _token: &str,
        _id: &str,
        _input: UpdateIssueInput,
    ) -> Result<Issue, CliError> {
        Ok(self.issue.clone())
    }

    fn archive_issue(&self, _token: &str, _id: &str, _trash: bool) -> Result<Issue, CliError> {
        Ok(self.issue.clone())
    }

    fn unarchive_issue(&self, _token: &str, _id: &str) -> Result<Issue, CliError> {
        Ok(self.issue.clone())
    }

    fn create_issue_relation(
        &self,
        _token: &str,
        _input: CreateIssueRelationInput,
    ) -> Result<Issue, CliError> {
        Ok(self.issue.clone())
    }
}

struct BenchCommentClient {
    comment: Comment,
}

impl CommentClient for BenchCommentClient {
    fn list_comments(
        &self,
        _token: &str,
        _issue_id: &str,
        _limit: usize,
    ) -> Result<Vec<Comment>, CliError> {
        Ok(vec![self.comment.clone()])
    }

    fn create_comment(
        &self,
        _token: &str,
        _input: CreateCommentInput,
    ) -> Result<Comment, CliError> {
        Ok(self.comment.clone())
    }
}

struct BenchProjectClient {
    project: Project,
}

impl ProjectClient for BenchProjectClient {
    fn get_project(&self, _token: &str, _id: &str) -> Result<Project, CliError> {
        Ok(self.project.clone())
    }

    fn list_projects(&self, _token: &str, _limit: usize) -> Result<Vec<Project>, CliError> {
        Ok(vec![self.project.clone()])
    }
}

struct BenchTeamClient {
    team: Team,
}

impl TeamClient for BenchTeamClient {
    fn get_team(&self, _token: &str, _id: &str) -> Result<Team, CliError> {
        Ok(self.team.clone())
    }

    fn list_teams(&self, _token: &str, _limit: usize) -> Result<Vec<Team>, CliError> {
        Ok(vec![self.team.clone()])
    }
}

struct BenchCycleClient {
    cycle: Cycle,
}

impl CycleClient for BenchCycleClient {
    fn get_cycle(&self, _token: &str, _id: &str) -> Result<Cycle, CliError> {
        Ok(self.cycle.clone())
    }

    fn list_cycles(&self, _token: &str, _limit: usize) -> Result<Vec<Cycle>, CliError> {
        Ok(vec![self.cycle.clone()])
    }
}

fn sample_issue() -> Issue {
    Issue {
        id: "issue-123".to_string(),
        identifier: "ENG-123".to_string(),
        title: "Benchmark issue".to_string(),
        description: Some("Issue used for benchmarking command handlers".to_string()),
        state: IssueState {
            id: "state-1".to_string(),
            name: "In Progress".to_string(),
        },
        priority: Priority::High,
        assignee: Some(User {
            id: "user-1".to_string(),
            name: "Engineer".to_string(),
            email: "engineer@example.com".to_string(),
        }),
        creator: User {
            id: "user-2".to_string(),
            name: "Creator".to_string(),
            email: "creator@example.com".to_string(),
        },
        project: None,
        created_at: "2026-02-24T00:00:00Z".to_string(),
        updated_at: "2026-02-24T00:00:00Z".to_string(),
        url: "https://linear.app/company/issue/ENG-123".to_string(),
        comments: None,
    }
}

fn sample_comment() -> Comment {
    Comment {
        id: "comment-123".to_string(),
        body: "Benchmark comment body".to_string(),
        user_name: "Engineer".to_string(),
        user_email: "engineer@example.com".to_string(),
        created_at: "2026-02-24T00:00:00Z".to_string(),
        updated_at: "2026-02-24T00:00:00Z".to_string(),
        edited_at: None,
        issue_identifier: Some("ENG-123".to_string()),
    }
}

fn sample_project() -> Project {
    Project {
        id: "project-1".to_string(),
        name: "Benchmark Project".to_string(),
        description: "Project used for benchmark command handlers".to_string(),
        content: None,
        slug_id: "benchmark-project".to_string(),
        url: "https://linear.app/company/project/benchmark-project".to_string(),
        color: "#5E6AD2".to_string(),
        icon: Some("📦".to_string()),
        status_name: "In Progress".to_string(),
        status_type: "started".to_string(),
        status_color: "#5E6AD2".to_string(),
        progress: 0.5,
        priority: 2,
        priority_label: "High".to_string(),
        start_date: Some("2026-01-01".to_string()),
        target_date: Some("2026-03-01".to_string()),
        lead_name: Some("Engineer".to_string()),
        created_at: "2026-02-24T00:00:00Z".to_string(),
        updated_at: "2026-02-24T00:00:00Z".to_string(),
    }
}

fn sample_team() -> Team {
    Team {
        id: "team-1".to_string(),
        key: "ENG".to_string(),
        name: "Engineering".to_string(),
        description: Some("Engineering team".to_string()),
        color: Some("#FF6900".to_string()),
        icon: Some("🔧".to_string()),
        private: false,
        created_at: "2026-02-24T00:00:00Z".to_string(),
    }
}

fn sample_cycle() -> Cycle {
    Cycle {
        id: "cycle-1".to_string(),
        name: "Sprint 1".to_string(),
        number: 1.0,
        description: Some("Cycle for benchmarking".to_string()),
        starts_at: "2026-02-01T00:00:00Z".to_string(),
        ends_at: "2026-02-14T00:00:00Z".to_string(),
        created_at: "2026-01-25T00:00:00Z".to_string(),
        completed_at: None,
        progress: 0.75,
        is_active: true,
        is_future: false,
        is_next: false,
        is_past: false,
        is_previous: false,
        team_name: "Engineering".to_string(),
        team_key: "ENG".to_string(),
    }
}

fn bench_issue_handler_paths(c: &mut Criterion) {
    let issue_client = BenchIssueClient {
        issue: sample_issue(),
    };
    let comment_client = BenchCommentClient {
        comment: sample_comment(),
    };

    let storage = benchmark_storage();
    let config = benchmark_config();
    let io = NoopIo;

    let mut group = c.benchmark_group("issue_handlers_json");

    group.bench_function("create", |b| {
        b.iter(|| {
            black_box(
                handle_create(
                    "ENG",
                    "Benchmark create",
                    Some("Description".to_string()),
                    Some("@me".to_string()),
                    Some("project-1".to_string()),
                    Some("state-1".to_string()),
                    Some(2),
                    &issue_client,
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
                handle_update(
                    "ENG-123",
                    Some("Benchmark update".to_string()),
                    Some("New description".to_string()),
                    Some("user-1".to_string()),
                    Some("project-1".to_string()),
                    Some("state-1".to_string()),
                    Some(1),
                    &issue_client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    group.bench_function("comment_add", |b| {
        b.iter(|| {
            black_box(
                handle_comment_add(
                    "ENG-123",
                    "Benchmark comment",
                    &comment_client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    group.bench_function("lifecycle_archive", |b| {
        b.iter(|| {
            black_box(
                handle_archive(
                    "ENG-123",
                    false,
                    &issue_client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    group.bench_function("lifecycle_unarchive", |b| {
        b.iter(|| {
            black_box(
                handle_unarchive(
                    "ENG-123",
                    &issue_client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    group.bench_function("relation_link", |b| {
        b.iter(|| {
            black_box(
                handle_link(
                    "ENG-123",
                    "ENG-456",
                    &issue_client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    group.bench_function("relation_block", |b| {
        b.iter(|| {
            black_box(
                handle_block(
                    "ENG-123",
                    "ENG-456",
                    &issue_client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    group.bench_function("relation_duplicate", |b| {
        b.iter(|| {
            black_box(
                handle_duplicate(
                    "ENG-123",
                    "ENG-456",
                    &issue_client,
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

    let mut format_group = c.benchmark_group("issue_handlers_format_compare");
    format_group.bench_function("create_json", |b| {
        b.iter(|| {
            black_box(
                handle_create(
                    "ENG",
                    "Benchmark create",
                    Some("Description".to_string()),
                    None,
                    None,
                    None,
                    Some(2),
                    &issue_client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    format_group.bench_function("create_table", |b| {
        b.iter(|| {
            black_box(
                handle_create(
                    "ENG",
                    "Benchmark create",
                    Some("Description".to_string()),
                    None,
                    None,
                    None,
                    Some(2),
                    &issue_client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Table),
                )
                .unwrap(),
            )
        })
    });

    format_group.finish();
}

fn bench_project_handler_paths(c: &mut Criterion) {
    let client = BenchProjectClient {
        project: sample_project(),
    };
    let storage = benchmark_storage();
    let config = benchmark_config();
    let io = NoopIo;

    let mut group = c.benchmark_group("project_handlers_json");

    group.bench_function("list", |b| {
        b.iter(|| {
            black_box(
                handle_project_list(
                    50,
                    &client,
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
                handle_project_view(
                    "project-1",
                    &client,
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

    let mut format_group = c.benchmark_group("project_handlers_format_compare");

    format_group.bench_function("list_json", |b| {
        b.iter(|| {
            black_box(
                handle_project_list(
                    50,
                    &client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    format_group.bench_function("list_table", |b| {
        b.iter(|| {
            black_box(
                handle_project_list(
                    50,
                    &client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Table),
                )
                .unwrap(),
            )
        })
    });

    format_group.bench_function("view_json", |b| {
        b.iter(|| {
            black_box(
                handle_project_view(
                    "project-1",
                    &client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    format_group.bench_function("view_table", |b| {
        b.iter(|| {
            black_box(
                handle_project_view(
                    "project-1",
                    &client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Table),
                )
                .unwrap(),
            )
        })
    });

    format_group.finish();
}

fn bench_team_handler_paths(c: &mut Criterion) {
    let client = BenchTeamClient {
        team: sample_team(),
    };
    let storage = benchmark_storage();
    let config = benchmark_config();
    let io = NoopIo;

    let mut group = c.benchmark_group("team_handlers_json");

    group.bench_function("list", |b| {
        b.iter(|| {
            black_box(
                handle_team_list(
                    50,
                    &client,
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
                handle_team_view(
                    "team-1",
                    &client,
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

    let mut format_group = c.benchmark_group("team_handlers_format_compare");

    format_group.bench_function("list_json", |b| {
        b.iter(|| {
            black_box(
                handle_team_list(
                    50,
                    &client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    format_group.bench_function("list_table", |b| {
        b.iter(|| {
            black_box(
                handle_team_list(
                    50,
                    &client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Table),
                )
                .unwrap(),
            )
        })
    });

    format_group.bench_function("view_json", |b| {
        b.iter(|| {
            black_box(
                handle_team_view(
                    "team-1",
                    &client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    format_group.bench_function("view_table", |b| {
        b.iter(|| {
            black_box(
                handle_team_view(
                    "team-1",
                    &client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Table),
                )
                .unwrap(),
            )
        })
    });

    format_group.finish();
}

fn bench_cycle_handler_paths(c: &mut Criterion) {
    let client = BenchCycleClient {
        cycle: sample_cycle(),
    };
    let storage = benchmark_storage();
    let config = benchmark_config();
    let io = NoopIo;

    let mut group = c.benchmark_group("cycle_handlers_json");

    group.bench_function("list", |b| {
        b.iter(|| {
            black_box(
                handle_cycle_list(
                    50,
                    &client,
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
                handle_cycle_view(
                    "cycle-1",
                    &client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    group.bench_function("current", |b| {
        b.iter(|| {
            black_box(
                handle_cycle_current(&client, &config, &storage, &io, Some(OutputFormat::Json))
                    .unwrap(),
            )
        })
    });

    group.finish();

    let mut format_group = c.benchmark_group("cycle_handlers_format_compare");

    format_group.bench_function("list_json", |b| {
        b.iter(|| {
            black_box(
                handle_cycle_list(
                    50,
                    &client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    format_group.bench_function("list_table", |b| {
        b.iter(|| {
            black_box(
                handle_cycle_list(
                    50,
                    &client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Table),
                )
                .unwrap(),
            )
        })
    });

    format_group.bench_function("view_json", |b| {
        b.iter(|| {
            black_box(
                handle_cycle_view(
                    "cycle-1",
                    &client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Json),
                )
                .unwrap(),
            )
        })
    });

    format_group.bench_function("view_table", |b| {
        b.iter(|| {
            black_box(
                handle_cycle_view(
                    "cycle-1",
                    &client,
                    &config,
                    &storage,
                    &io,
                    Some(OutputFormat::Table),
                )
                .unwrap(),
            )
        })
    });

    format_group.finish();
}

fn bench_cli_parse_paths(c: &mut Criterion) {
    let mut issue_group = c.benchmark_group("cli_parse_issue_paths");

    issue_group.bench_function("issue_create_rich", |b| {
        b.iter(|| {
            black_box(
                Cli::try_parse_from([
                    "linear",
                    "issue",
                    "create",
                    "--team",
                    "ENG",
                    "--title",
                    "Benchmark title",
                    "--description",
                    "Benchmark description",
                    "--assignee",
                    "@me",
                    "--project",
                    "buildr-agent-phase-2",
                    "--state",
                    "In Progress",
                    "--priority",
                    "2",
                ])
                .unwrap(),
            )
        })
    });

    issue_group.bench_function("issue_update_rich", |b| {
        b.iter(|| {
            black_box(
                Cli::try_parse_from([
                    "linear",
                    "issue",
                    "update",
                    "ENG-123",
                    "--title",
                    "Updated benchmark",
                    "--description",
                    "Updated description",
                    "--assignee",
                    "engineer@example.com",
                    "--project",
                    "Buildr Agent Phase 2",
                    "--state",
                    "Done",
                    "--priority",
                    "1",
                ])
                .unwrap(),
            )
        })
    });

    issue_group.bench_function("issue_relation_link", |b| {
        b.iter(|| {
            black_box(
                Cli::try_parse_from([
                    "linear", "issue", "relation", "link", "ENG-123", "ENG-456", "--json",
                ])
                .unwrap(),
            )
        })
    });

    issue_group.bench_function("issue_create_invalid_priority", |b| {
        b.iter(|| {
            black_box(
                Cli::try_parse_from([
                    "linear",
                    "issue",
                    "create",
                    "--team",
                    "ENG",
                    "--title",
                    "Bad priority",
                    "--priority",
                    "9",
                ])
                .is_err(),
            )
        })
    });

    issue_group.finish();

    let mut project_group = c.benchmark_group("cli_parse_project_paths");

    project_group.bench_function("project_list", |b| {
        b.iter(|| {
            black_box(Cli::try_parse_from(["linear", "project", "list", "--limit", "10"]).unwrap())
        })
    });

    project_group.bench_function("project_view", |b| {
        b.iter(|| {
            black_box(
                Cli::try_parse_from(["linear", "project", "view", "project-1", "--json"]).unwrap(),
            )
        })
    });

    project_group.finish();

    let mut team_group = c.benchmark_group("cli_parse_team_paths");

    team_group.bench_function("team_list", |b| {
        b.iter(|| {
            black_box(Cli::try_parse_from(["linear", "team", "list", "--limit", "10"]).unwrap())
        })
    });

    team_group.bench_function("team_view", |b| {
        b.iter(|| {
            black_box(Cli::try_parse_from(["linear", "team", "view", "team-1", "--json"]).unwrap())
        })
    });

    team_group.finish();

    let mut cycle_group = c.benchmark_group("cli_parse_cycle_paths");

    cycle_group.bench_function("cycle_list", |b| {
        b.iter(|| {
            black_box(Cli::try_parse_from(["linear", "cycle", "list", "--limit", "10"]).unwrap())
        })
    });

    cycle_group.bench_function("cycle_view", |b| {
        b.iter(|| {
            black_box(
                Cli::try_parse_from(["linear", "cycle", "view", "cycle-1", "--json"]).unwrap(),
            )
        })
    });

    cycle_group.bench_function("cycle_current", |b| {
        b.iter(|| black_box(Cli::try_parse_from(["linear", "cycle", "current", "--json"]).unwrap()))
    });

    cycle_group.finish();
}

criterion_group!(
    benches,
    bench_issue_handler_paths,
    bench_project_handler_paths,
    bench_team_handler_paths,
    bench_cycle_handler_paths,
    bench_cli_parse_paths
);
criterion_main!(benches);
