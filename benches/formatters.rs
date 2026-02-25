use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use linear_cli::comments::types::{Comment, CommentList};
use linear_cli::cycles::types::{Cycle, CycleList};
use linear_cli::issues::types::{Issue, IssueList, IssueState, Priority, User};
use linear_cli::output::Formattable;
use linear_cli::projects::types::{Project, ProjectList};
use linear_cli::teams::types::{Team, TeamList};

/// Helper to create a test issue
fn create_test_issue(id: usize) -> Issue {
    Issue {
        id: format!("issue-{}", id),
        identifier: format!("ENG-{}", id),
        title: format!("Issue {} - Fix critical bug in authentication flow", id),
        description: Some(format!(
            "This is a detailed description for issue {}. Users are experiencing problems with the SSO login flow. We need to investigate the token validation logic and ensure it handles edge cases correctly.",
            id
        )),
        state: IssueState {
            id: format!("state-{}", id % 4),
            name: match id % 4 {
                0 => "Todo",
                1 => "In Progress",
                2 => "In Review",
                _ => "Done",
            }
            .to_string(),
        },
        priority: match id % 5 {
            0 => Priority::None,
            1 => Priority::Urgent,
            2 => Priority::High,
            3 => Priority::Medium,
            _ => Priority::Low,
        },
        assignee: if id.is_multiple_of(3) {
            None
        } else {
            Some(User {
                id: format!("user-{}", id % 10),
                name: format!("Engineer {}", id % 10),
                email: format!("engineer{}@example.com", id % 10),
            })
        },
        creator: User {
            id: format!("creator-{}", id % 5),
            name: format!("Creator {}", id % 5),
            email: format!("creator{}@example.com", id % 5),
        },
        project: None,
        created_at: "2025-11-01T10:00:00Z".to_string(),
        updated_at: "2025-11-13T14:30:00Z".to_string(),
        url: format!("https://linear.app/team/issue/ENG-{}", id),
        comments: None,
    }
}

/// Create a list of test issues
fn create_test_issues(count: usize) -> Vec<Issue> {
    (0..count).map(create_test_issue).collect()
}

/// Benchmark single issue formatters
fn bench_single_issue(c: &mut Criterion) {
    let issue = create_test_issue(123);

    c.bench_function("issue_json", |b| {
        b.iter(|| black_box(issue.to_json().unwrap()))
    });

    c.bench_function("issue_csv", |b| {
        b.iter(|| black_box(issue.to_csv().unwrap()))
    });

    c.bench_function("issue_markdown", |b| {
        b.iter(|| black_box(issue.to_markdown().unwrap()))
    });

    c.bench_function("issue_table", |b| {
        b.iter(|| black_box(issue.to_table().unwrap()))
    });
}

/// Benchmark issue list formatters with varying sizes
fn bench_issue_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("issue_list");

    for size in [10, 50, 100, 500].iter() {
        let issues = create_test_issues(*size);
        let list = IssueList(issues);

        group.bench_with_input(BenchmarkId::new("json", size), &list, |b, list| {
            b.iter(|| black_box(list.to_json().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("csv", size), &list, |b, list| {
            b.iter(|| black_box(list.to_csv().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("markdown", size), &list, |b, list| {
            b.iter(|| black_box(list.to_markdown().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("table", size), &list, |b, list| {
            b.iter(|| black_box(list.to_table().unwrap()))
        });
    }

    group.finish();
}

/// Helper to create a test team
fn create_test_team(id: usize) -> Team {
    Team {
        id: format!("team-{id}"),
        key: format!("TEAM{id}"),
        name: format!("Team {} - Engineering", id),
        description: Some(format!(
            "This is team {} responsible for backend services",
            id
        )),
        color: Some("#FF6900".to_string()),
        icon: Some("🔧".to_string()),
        private: id.is_multiple_of(2),
        created_at: "2025-01-15T10:30:00Z".to_string(),
    }
}

/// Benchmark single team formatters
fn bench_single_team(c: &mut Criterion) {
    let team = create_test_team(42);

    c.bench_function("team_json", |b| {
        b.iter(|| black_box(team.to_json().unwrap()))
    });

    c.bench_function("team_csv", |b| b.iter(|| black_box(team.to_csv().unwrap())));

    c.bench_function("team_markdown", |b| {
        b.iter(|| black_box(team.to_markdown().unwrap()))
    });

    c.bench_function("team_table", |b| {
        b.iter(|| black_box(team.to_table().unwrap()))
    });
}

/// Benchmark team list formatters
fn bench_team_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("team_list");

    for size in [10, 50, 100].iter() {
        let teams: Vec<Team> = (0..*size).map(create_test_team).collect();
        let list = TeamList(teams);

        group.bench_with_input(BenchmarkId::new("json", size), &list, |b, list| {
            b.iter(|| black_box(list.to_json().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("csv", size), &list, |b, list| {
            b.iter(|| black_box(list.to_csv().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("markdown", size), &list, |b, list| {
            b.iter(|| black_box(list.to_markdown().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("table", size), &list, |b, list| {
            b.iter(|| black_box(list.to_table().unwrap()))
        });
    }

    group.finish();
}

/// Helper to create a test project
fn create_test_project(id: usize) -> Project {
    Project {
        id: format!("project-{id}"),
        name: format!("Project {} - Backend Redesign", id),
        description: format!(
            "Comprehensive redesign of the backend infrastructure for project {}. This includes database optimization, API refactoring, and performance improvements.",
            id
        ),
        content: None,
        slug_id: format!("backend-redesign-{id}"),
        url: format!("https://linear.app/team/project/backend-redesign-{id}"),
        color: "#FF6900".to_string(),
        icon: Some("🔧".to_string()),
        status_name: match id % 4 {
            0 => "Backlog",
            1 => "Planned",
            2 => "In Progress",
            _ => "Completed",
        }
        .to_string(),
        status_type: match id % 4 {
            0 => "backlog",
            1 => "planned",
            2 => "started",
            _ => "completed",
        }
        .to_string(),
        status_color: "#5E6AD2".to_string(),
        progress: (id % 100) as f64 / 100.0,
        priority: (id % 5) as i32,
        priority_label: match id % 5 {
            0 => "None",
            1 => "Low",
            2 => "Medium",
            3 => "High",
            _ => "Urgent",
        }
        .to_string(),
        start_date: if id.is_multiple_of(3) {
            None
        } else {
            Some("2024-01-15".to_string())
        },
        target_date: Some("2024-06-30".to_string()),
        lead_name: if id.is_multiple_of(2) {
            Some(format!("Lead Engineer {}", id % 10))
        } else {
            None
        },
        created_at: "2024-01-10T10:00:00Z".to_string(),
        updated_at: "2024-02-15T14:30:00Z".to_string(),
    }
}

/// Benchmark single project formatters
fn bench_single_project(c: &mut Criterion) {
    let project = create_test_project(42);

    c.bench_function("project_json", |b| {
        b.iter(|| black_box(project.to_json().unwrap()))
    });

    c.bench_function("project_csv", |b| {
        b.iter(|| black_box(project.to_csv().unwrap()))
    });

    c.bench_function("project_markdown", |b| {
        b.iter(|| black_box(project.to_markdown().unwrap()))
    });

    c.bench_function("project_table", |b| {
        b.iter(|| black_box(project.to_table().unwrap()))
    });
}

/// Benchmark project list formatters
fn bench_project_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("project_list");

    for size in [10, 50, 100].iter() {
        let projects: Vec<Project> = (0..*size).map(create_test_project).collect();
        let list = ProjectList(projects);

        group.bench_with_input(BenchmarkId::new("json", size), &list, |b, list| {
            b.iter(|| black_box(list.to_json().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("csv", size), &list, |b, list| {
            b.iter(|| black_box(list.to_csv().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("markdown", size), &list, |b, list| {
            b.iter(|| black_box(list.to_markdown().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("table", size), &list, |b, list| {
            b.iter(|| black_box(list.to_table().unwrap()))
        });
    }

    group.finish();
}

/// Helper to create a test cycle
fn create_test_cycle(id: usize) -> Cycle {
    Cycle {
        id: format!("cycle-{id}"),
        name: format!("Sprint {} - Q1 2024", id),
        number: id as f64,
        description: Some(format!(
            "Development cycle {} focusing on performance improvements and feature development. Key objectives include API optimization and user experience enhancements.",
            id
        )),
        starts_at: "2024-01-15T00:00:00Z".to_string(),
        ends_at: "2024-01-29T23:59:59Z".to_string(),
        created_at: "2024-01-10T10:00:00Z".to_string(),
        completed_at: if id.is_multiple_of(3) {
            Some("2024-01-29T18:00:00Z".to_string())
        } else {
            None
        },
        progress: (id % 100) as f64 / 100.0,
        is_active: id.is_multiple_of(5),
        is_future: id.is_multiple_of(7),
        is_next: id.is_multiple_of(11),
        is_past: id.is_multiple_of(13),
        is_previous: id.is_multiple_of(17),
        team_name: format!("Team {}", id % 10),
        team_key: format!("TEAM{}", id % 10),
    }
}

/// Benchmark single cycle formatters
fn bench_single_cycle(c: &mut Criterion) {
    let cycle = create_test_cycle(42);

    c.bench_function("cycle_json", |b| {
        b.iter(|| black_box(cycle.to_json().unwrap()))
    });

    c.bench_function("cycle_csv", |b| {
        b.iter(|| black_box(cycle.to_csv().unwrap()))
    });

    c.bench_function("cycle_markdown", |b| {
        b.iter(|| black_box(cycle.to_markdown().unwrap()))
    });

    c.bench_function("cycle_table", |b| {
        b.iter(|| black_box(cycle.to_table().unwrap()))
    });
}

/// Benchmark cycle list formatters
fn bench_cycle_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("cycle_list");

    for size in [10, 50, 100].iter() {
        let cycles: Vec<Cycle> = (0..*size).map(create_test_cycle).collect();
        let list = CycleList(cycles);

        group.bench_with_input(BenchmarkId::new("json", size), &list, |b, list| {
            b.iter(|| black_box(list.to_json().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("csv", size), &list, |b, list| {
            b.iter(|| black_box(list.to_csv().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("markdown", size), &list, |b, list| {
            b.iter(|| black_box(list.to_markdown().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("table", size), &list, |b, list| {
            b.iter(|| black_box(list.to_table().unwrap()))
        });
    }

    group.finish();
}

/// Benchmark finding the active cycle - comparing different implementations
fn bench_cycle_current(c: &mut Criterion) {
    let mut group = c.benchmark_group("cycle_current");

    for size in [10, 25, 50].iter() {
        // Create cycles with one active cycle in the middle
        let cycles: Vec<Cycle> = (0..*size)
            .map(|i| {
                let mut cycle = create_test_cycle(i);
                // Make the middle one active
                cycle.is_active = i == size / 2;
                cycle.is_past = i < size / 2;
                cycle.is_future = i > size / 2;
                cycle
            })
            .collect();

        // Strategy 1: iter().find() + clone (returns reference, then clone)
        group.bench_with_input(
            BenchmarkId::new("iter_find_clone", size),
            &cycles,
            |b, cycles| {
                b.iter(|| {
                    black_box(
                        cycles
                            .iter()
                            .find(|c| c.is_active)
                            .expect("Should find active cycle")
                            .clone(),
                    )
                })
            },
        );

        // Strategy 2: into_iter().find() (consumes vector, returns owned)
        group.bench_with_input(
            BenchmarkId::new("into_iter_find", size),
            &cycles,
            |b, cycles| {
                b.iter(|| {
                    black_box(
                        cycles
                            .clone()
                            .into_iter()
                            .find(|c| c.is_active)
                            .expect("Should find active cycle"),
                    )
                })
            },
        );

        // Strategy 3: Manual for loop with early return
        group.bench_with_input(BenchmarkId::new("for_loop", size), &cycles, |b, cycles| {
            b.iter(|| {
                black_box({
                    let mut result = None;
                    for cycle in cycles {
                        if cycle.is_active {
                            result = Some(cycle.clone());
                            break;
                        }
                    }
                    result.expect("Should find active cycle")
                })
            })
        });

        // Strategy 4: position() + indexing (find index, then clone at index)
        group.bench_with_input(
            BenchmarkId::new("position_index", size),
            &cycles,
            |b, cycles| {
                b.iter(|| {
                    black_box({
                        let pos = cycles
                            .iter()
                            .position(|c| c.is_active)
                            .expect("Should find position");
                        cycles[pos].clone()
                    })
                })
            },
        );

        // Strategy 5: filter().next() (alternative to find)
        group.bench_with_input(
            BenchmarkId::new("filter_next", size),
            &cycles,
            |b, cycles| {
                b.iter(|| {
                    black_box(
                        cycles
                            .iter()
                            .find(|c| c.is_active)
                            .expect("Should find active cycle")
                            .clone(),
                    )
                })
            },
        );

        // Strategy 6: iter().cloned().find() (clone during iteration)
        group.bench_with_input(
            BenchmarkId::new("iter_cloned_find", size),
            &cycles,
            |b, cycles| {
                b.iter(|| {
                    black_box(
                        cycles
                            .iter()
                            .find(|c| c.is_active)
                            .cloned()
                            .expect("Should find active cycle"),
                    )
                })
            },
        );
    }

    group.finish();
}

/// Helper to create a test comment
fn create_test_comment(id: usize) -> Comment {
    Comment {
        id: format!("comment-{id}"),
        body: format!(
            "This is comment {} with **markdown** formatting and some detailed content about the issue discussion.",
            id
        ),
        user_name: format!("User {}", id % 10),
        user_email: format!("user{}@example.com", id % 10),
        created_at: "2024-01-15T10:30:00Z".to_string(),
        updated_at: "2024-01-15T10:30:00Z".to_string(),
        edited_at: if id.is_multiple_of(3) {
            Some("2024-01-15T14:00:00Z".to_string())
        } else {
            None
        },
        issue_identifier: Some(format!("ENG-{}", id + 100)),
    }
}

/// Benchmark single comment formatters
fn bench_single_comment(c: &mut Criterion) {
    let comment = create_test_comment(42);

    c.bench_function("comment_json", |b| {
        b.iter(|| black_box(comment.to_json().unwrap()))
    });

    c.bench_function("comment_csv", |b| {
        b.iter(|| black_box(comment.to_csv().unwrap()))
    });

    c.bench_function("comment_markdown", |b| {
        b.iter(|| black_box(comment.to_markdown().unwrap()))
    });

    c.bench_function("comment_table", |b| {
        b.iter(|| black_box(comment.to_table().unwrap()))
    });
}

/// Benchmark comment list formatters
fn bench_comment_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("comment_list");

    for size in [10, 50, 100].iter() {
        let comments: Vec<Comment> = (0..*size).map(create_test_comment).collect();
        let list = CommentList(comments);

        group.bench_with_input(BenchmarkId::new("json", size), &list, |b, list| {
            b.iter(|| black_box(list.to_json().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("csv", size), &list, |b, list| {
            b.iter(|| black_box(list.to_csv().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("markdown", size), &list, |b, list| {
            b.iter(|| black_box(list.to_markdown().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("table", size), &list, |b, list| {
            b.iter(|| black_box(list.to_table().unwrap()))
        });
    }

    group.finish();
}

/// Benchmark issue with comments - varying comment counts
fn bench_issue_with_comments(c: &mut Criterion) {
    let mut group = c.benchmark_group("issue_with_comments");

    // Benchmark with 0, 10, and 50 comments to show performance impact
    for comment_count in [0, 10, 50].iter() {
        let mut issue = create_test_issue(123);

        // Add comments to the issue
        if *comment_count > 0 {
            let comments: Vec<Comment> = (0..*comment_count)
                .map(|i| Comment {
                    id: format!("comment-{}", i),
                    body: format!("This is comment {} with some content about the issue", i),
                    user_name: format!("User {}", i % 5),
                    user_email: format!("user{}@example.com", i % 5),
                    created_at: "2024-01-15T10:00:00Z".to_string(),
                    updated_at: "2024-01-15T10:00:00Z".to_string(),
                    edited_at: None,
                    issue_identifier: Some("ENG-123".to_string()),
                })
                .collect();
            issue.comments = Some(comments);
        }

        group.bench_with_input(
            BenchmarkId::new("json", comment_count),
            &issue,
            |b, issue| b.iter(|| black_box(issue.to_json().unwrap())),
        );

        group.bench_with_input(
            BenchmarkId::new("csv", comment_count),
            &issue,
            |b, issue| b.iter(|| black_box(issue.to_csv().unwrap())),
        );

        group.bench_with_input(
            BenchmarkId::new("markdown", comment_count),
            &issue,
            |b, issue| b.iter(|| black_box(issue.to_markdown().unwrap())),
        );

        group.bench_with_input(
            BenchmarkId::new("table", comment_count),
            &issue,
            |b, issue| b.iter(|| black_box(issue.to_table().unwrap())),
        );
    }

    group.finish();
}

fn bench_table_hotspots(c: &mut Criterion) {
    let mut large_rows = c.benchmark_group("table_hotspot_large_rows");

    for size in [250, 500, 1000] {
        let teams: Vec<Team> = (0..size).map(create_test_team).collect();
        let team_list = TeamList(teams);

        large_rows.bench_with_input(
            BenchmarkId::new("team_list_table", size),
            &team_list,
            |b, list| b.iter(|| black_box(list.to_table().unwrap())),
        );

        let projects: Vec<Project> = (0..size).map(create_test_project).collect();
        let project_list = ProjectList(projects);

        large_rows.bench_with_input(
            BenchmarkId::new("project_list_table", size),
            &project_list,
            |b, list| b.iter(|| black_box(list.to_table().unwrap())),
        );
    }

    large_rows.finish();

    let mut wide_cells = c.benchmark_group("table_hotspot_wide_cells");

    let wide_teams: Vec<Team> = (0..300)
        .map(|id| Team {
            description: Some("x".repeat(512)),
            ..create_test_team(id)
        })
        .collect();
    let wide_team_list = TeamList(wide_teams);
    wide_cells.bench_function("team_list_table_wide", |b| {
        b.iter(|| black_box(wide_team_list.to_table().unwrap()))
    });

    let wide_projects: Vec<Project> = (0..300)
        .map(|id| Project {
            name: format!("{}{}", create_test_project(id).name, "y".repeat(256)),
            description: "z".repeat(1024),
            ..create_test_project(id)
        })
        .collect();
    let wide_project_list = ProjectList(wide_projects);
    wide_cells.bench_function("project_list_table_wide", |b| {
        b.iter(|| black_box(wide_project_list.to_table().unwrap()))
    });

    wide_cells.finish();
}

criterion_group!(
    benches,
    bench_single_issue,
    bench_issue_list,
    bench_issue_with_comments,
    bench_single_team,
    bench_team_list,
    bench_single_project,
    bench_project_list,
    bench_single_cycle,
    bench_cycle_list,
    bench_cycle_current,
    bench_single_comment,
    bench_comment_list,
    bench_table_hotspots
);
criterion_main!(benches);
