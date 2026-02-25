use clap::Parser;
use linear_cli::auth::commands::{handle_login, handle_logout, handle_status, handle_token};
use linear_cli::auth::config::EnvConfigProvider;
use linear_cli::auth::storage::KeyringStorage;
use linear_cli::cli::{
    AuthCommands, Cli, Commands, CycleCommands, IssueCommands, IssueCommentCommands,
    IssueLifecycleCommands, IssueRelationCommands, LabelCommands, ProjectCommands, StateCommands,
    TeamCommands, UserCommands,
};
use linear_cli::client::LinearClient;
use linear_cli::client::auth::AuthClient;
use linear_cli::client::comments::CommentClient;
use linear_cli::client::cycles::CycleClient;
use linear_cli::client::issues::IssueClient;
use linear_cli::client::labels::LabelClient;
use linear_cli::client::projects::ProjectClient;
use linear_cli::client::search::SearchClient;
use linear_cli::client::semantic_search::SemanticSearchClient;
use linear_cli::client::states::StateClient;
use linear_cli::client::teams::TeamClient;
use linear_cli::client::users::UserClient;
use linear_cli::comments::commands::handle_list as handle_comment_list;
use linear_cli::cycles::commands::{
    handle_current as handle_cycle_current, handle_list as handle_cycle_list,
    handle_view as handle_cycle_view,
};
use linear_cli::io::RealIo;
use linear_cli::issues::commands::{
    comment_delete::handle_comment_delete,
    delete::handle_delete as handle_issue_delete,
    handle_archive as handle_issue_archive, handle_block as handle_issue_relation_block,
    handle_comment_add as handle_issue_comment_add, handle_create as handle_issue_create,
    handle_duplicate as handle_issue_relation_duplicate, handle_link as handle_issue_relation_link,
    handle_list as handle_issue_list, handle_unarchive as handle_issue_unarchive,
    handle_update as handle_issue_update,
    search::handle_search as handle_issue_search,
    view::{ViewDeps, handle_view as handle_issue_view},
};
use linear_cli::labels::commands::list::handle_list as handle_label_list;
use linear_cli::projects::commands::{
    handle_list as handle_project_list, handle_view as handle_project_view,
};
use linear_cli::search::commands::search::handle_semantic_search;
use linear_cli::states::commands::list::handle_list as handle_state_list;
use linear_cli::teams::commands::{
    handle_list as handle_team_list, handle_view as handle_team_view,
};
use linear_cli::users::commands::list::handle_list as handle_user_list;
use secrecy::SecretString;
use std::io::{self, Read};

/// Read token from stdin if --with-token flag is set
/// Returns SecretString to minimize exposure window in memory
fn read_token_from_stdin_if_needed(with_token: bool) -> Option<SecretString> {
    if !with_token {
        return None;
    }

    let mut token = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut token) {
        eprintln!("Error reading token from stdin: {}", e);
        std::process::exit(1);
    }
    Some(SecretString::from(token.trim().to_string()))
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Auth { action } => {
            let storage = match KeyringStorage::new() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(e.exit_code());
                }
            };
            let config = EnvConfigProvider;
            let io = RealIo;

            let client = LinearClient::new();

            match action {
                AuthCommands::Login {
                    with_token,
                    format: _,
                } => {
                    // Auth login doesn't use formatters (just returns user info)
                    let token_input = read_token_from_stdin_if_needed(with_token);
                    handle_login(token_input, &client as &dyn AuthClient, &storage, &io).map(|_| ())
                }
                AuthCommands::Status { format } => handle_status(
                    &config,
                    &storage,
                    &client as &dyn AuthClient,
                    &io,
                    format.to_format(),
                ),
                AuthCommands::Logout { format } => {
                    handle_logout(&storage, &config, &io, format.to_format())
                }
                AuthCommands::Token { format: _ } => {
                    // Auth token outputs raw token (no formatting)
                    handle_token(&config, &storage, &io)
                }
            }
        }
        Commands::Issue { action } => {
            let storage = match KeyringStorage::new() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(e.exit_code());
                }
            };
            let config = EnvConfigProvider;
            let io = RealIo;
            let client = LinearClient::new();

            match action {
                IssueCommands::View {
                    identifier,
                    with_comments,
                    comment_limit,
                    format,
                } => {
                    let deps = ViewDeps {
                        issue_client: &client as &dyn IssueClient,
                        comment_client: &client as &dyn CommentClient,
                        config: &config,
                        storage: &storage,
                        io: &io,
                    };
                    handle_issue_view(
                        &identifier,
                        with_comments,
                        comment_limit,
                        &deps,
                        format.to_format(),
                    )
                }
                IssueCommands::List {
                    assignee,
                    project,
                    limit,
                    format,
                } => handle_issue_list(
                    assignee,
                    project,
                    limit,
                    &client as &dyn IssueClient,
                    &config,
                    &storage,
                    &io,
                    format.to_format(),
                ),
                IssueCommands::Create {
                    team,
                    title,
                    description,
                    assignee,
                    project,
                    state,
                    priority,
                    format,
                } => handle_issue_create(
                    &team,
                    &title,
                    description,
                    assignee,
                    project,
                    state,
                    priority.map(i32::from),
                    &client as &dyn IssueClient,
                    &config,
                    &storage,
                    &io,
                    format.to_format(),
                ),
                IssueCommands::Update {
                    identifier,
                    patch,
                    format,
                } => handle_issue_update(
                    &identifier,
                    patch.title,
                    patch.description,
                    patch.assignee,
                    patch.project,
                    patch.state,
                    patch.priority.map(i32::from),
                    &client as &dyn IssueClient,
                    &config,
                    &storage,
                    &io,
                    format.to_format(),
                ),
                IssueCommands::Search {
                    term,
                    team,
                    include_comments,
                    limit,
                    format,
                } => handle_issue_search(
                    &term,
                    team.as_deref(),
                    include_comments,
                    limit,
                    &client as &dyn SearchClient,
                    &config,
                    &storage,
                    &io,
                    format.to_format(),
                ),
                IssueCommands::Delete {
                    identifier,
                    permanently,
                    format,
                } => handle_issue_delete(
                    &identifier,
                    permanently,
                    &client as &dyn IssueClient,
                    &config,
                    &storage,
                    &io,
                    format.to_format(),
                ),
                IssueCommands::Lifecycle { action } => match action {
                    IssueLifecycleCommands::Archive { identifier, format } => handle_issue_archive(
                        &identifier,
                        false,
                        &client as &dyn IssueClient,
                        &config,
                        &storage,
                        &io,
                        format.to_format(),
                    ),
                    IssueLifecycleCommands::Unarchive { identifier, format } => {
                        handle_issue_unarchive(
                            &identifier,
                            &client as &dyn IssueClient,
                            &config,
                            &storage,
                            &io,
                            format.to_format(),
                        )
                    }
                },
                IssueCommands::Relation { action } => match action {
                    IssueRelationCommands::Link {
                        identifier,
                        related,
                        format,
                    } => handle_issue_relation_link(
                        &identifier,
                        &related,
                        &client as &dyn IssueClient,
                        &config,
                        &storage,
                        &io,
                        format.to_format(),
                    ),
                    IssueRelationCommands::Block {
                        identifier,
                        related,
                        format,
                    } => handle_issue_relation_block(
                        &identifier,
                        &related,
                        &client as &dyn IssueClient,
                        &config,
                        &storage,
                        &io,
                        format.to_format(),
                    ),
                    IssueRelationCommands::Duplicate {
                        identifier,
                        related,
                        format,
                    } => handle_issue_relation_duplicate(
                        &identifier,
                        &related,
                        &client as &dyn IssueClient,
                        &config,
                        &storage,
                        &io,
                        format.to_format(),
                    ),
                },
                IssueCommands::Comment { action } => match action {
                    IssueCommentCommands::Delete { id, format } => handle_comment_delete(
                        &id,
                        &client as &dyn CommentClient,
                        &config,
                        &storage,
                        &io,
                        format.to_format(),
                    ),
                    IssueCommentCommands::Add {
                        identifier,
                        body,
                        format,
                    } => handle_issue_comment_add(
                        &identifier,
                        &body,
                        &client as &dyn CommentClient,
                        &config,
                        &storage,
                        &io,
                        format.to_format(),
                    ),
                },
                IssueCommands::Comments {
                    issue_id,
                    limit,
                    format,
                } => handle_comment_list(
                    &client as &dyn CommentClient,
                    &config,
                    &storage,
                    &io,
                    &issue_id,
                    limit,
                    format.to_format(),
                ),
            }
        }
        Commands::Team { action } => {
            let storage = match KeyringStorage::new() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(e.exit_code());
                }
            };
            let config = EnvConfigProvider;
            let io = RealIo;
            let client = LinearClient::new();

            match action {
                TeamCommands::View { id, format } => handle_team_view(
                    &id,
                    &client as &dyn TeamClient,
                    &config,
                    &storage,
                    &io,
                    format.to_format(),
                ),
                TeamCommands::List { limit, format } => handle_team_list(
                    limit,
                    &client as &dyn TeamClient,
                    &config,
                    &storage,
                    &io,
                    format.to_format(),
                ),
            }
        }
        Commands::Project { action } => {
            let storage = match KeyringStorage::new() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(e.exit_code());
                }
            };
            let config = EnvConfigProvider;
            let io = RealIo;
            let client = LinearClient::new();

            match action {
                ProjectCommands::View { id, format } => handle_project_view(
                    &id,
                    &client as &dyn ProjectClient,
                    &config,
                    &storage,
                    &io,
                    format.to_format(),
                ),
                ProjectCommands::List { limit, format } => handle_project_list(
                    limit,
                    &client as &dyn ProjectClient,
                    &config,
                    &storage,
                    &io,
                    format.to_format(),
                ),
            }
        }
        Commands::Cycle { action } => {
            let storage = match KeyringStorage::new() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(e.exit_code());
                }
            };
            let config = EnvConfigProvider;
            let io = RealIo;
            let client = LinearClient::new();

            match action {
                CycleCommands::View { id, format } => handle_cycle_view(
                    &id,
                    &client as &dyn CycleClient,
                    &config,
                    &storage,
                    &io,
                    format.to_format(),
                ),
                CycleCommands::List { limit, format } => handle_cycle_list(
                    limit,
                    &client as &dyn CycleClient,
                    &config,
                    &storage,
                    &io,
                    format.to_format(),
                ),
                CycleCommands::Current { format } => handle_cycle_current(
                    &client as &dyn CycleClient,
                    &config,
                    &storage,
                    &io,
                    format.to_format(),
                ),
            }
        }
        Commands::Search {
            query,
            r#type,
            limit,
            format,
        } => {
            let storage = match KeyringStorage::new() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(e.exit_code());
                }
            };
            let config = EnvConfigProvider;
            let io = RealIo;
            let client = LinearClient::new();

            handle_semantic_search(
                &query,
                r#type.as_deref(),
                Some(limit),
                &client as &dyn SemanticSearchClient,
                &config,
                &storage,
                &io,
                format.to_format(),
            )
        }
        Commands::State { action } => {
            let storage = match KeyringStorage::new() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(e.exit_code());
                }
            };
            let config = EnvConfigProvider;
            let io = RealIo;
            let client = LinearClient::new();

            match action {
                StateCommands::List {
                    team,
                    limit,
                    format,
                } => handle_state_list(
                    limit,
                    team.as_deref(),
                    &client as &dyn StateClient,
                    &config,
                    &storage,
                    &io,
                    format.to_format(),
                ),
            }
        }
        Commands::Label { action } => {
            let storage = match KeyringStorage::new() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(e.exit_code());
                }
            };
            let config = EnvConfigProvider;
            let io = RealIo;
            let client = LinearClient::new();

            match action {
                LabelCommands::List {
                    team,
                    limit,
                    format,
                } => handle_label_list(
                    limit,
                    team.as_deref(),
                    &client as &dyn LabelClient,
                    &config,
                    &storage,
                    &io,
                    format.to_format(),
                ),
            }
        }
        Commands::User { action } => {
            let storage = match KeyringStorage::new() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(e.exit_code());
                }
            };
            let config = EnvConfigProvider;
            let io = RealIo;
            let client = LinearClient::new();

            match action {
                UserCommands::List { limit, format } => handle_user_list(
                    limit,
                    &client as &dyn UserClient,
                    &config,
                    &storage,
                    &io,
                    format.to_format(),
                ),
            }
        }
    };

    match result {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(e.exit_code());
        }
    }
}
