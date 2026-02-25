use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "linear")]
#[command(about = "Linear CLI tool", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Output format flags (mutually exclusive).
///
/// JSON style can be configured via `LINEAR_CLI_JSON_STYLE=compact|pretty`.
#[derive(Args, Debug, Clone)]
#[group(multiple = false)]
pub struct FormatFlags {
    /// Output as JSON
    #[arg(long)]
    pub json: bool,

    /// Output as CSV
    #[arg(long)]
    pub csv: bool,

    /// Output as Markdown
    #[arg(long)]
    pub markdown: bool,

    /// Output as table
    #[arg(long)]
    pub table: bool,
}

impl FormatFlags {
    /// Convert format flags to OutputFormat
    /// Returns None if no explicit format flag was set
    #[must_use]
    pub fn to_format(&self) -> Option<crate::output::OutputFormat> {
        use crate::output::OutputFormat;

        if self.json {
            Some(OutputFormat::Json)
        } else if self.csv {
            Some(OutputFormat::Csv)
        } else if self.markdown {
            Some(OutputFormat::Markdown)
        } else if self.table {
            Some(OutputFormat::Table)
        } else {
            None
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Authentication commands
    Auth {
        #[command(subcommand)]
        action: AuthCommands,
    },
    /// Issue commands
    Issue {
        #[command(subcommand)]
        action: IssueCommands,
    },
    /// Team commands
    Team {
        #[command(subcommand)]
        action: TeamCommands,
    },
    /// Project commands
    Project {
        #[command(subcommand)]
        action: ProjectCommands,
    },
    /// Cycle commands
    Cycle {
        #[command(subcommand)]
        action: CycleCommands,
    },
    /// Semantic search across issues, projects, documents, initiatives
    Search {
        /// Search query
        query: String,

        /// Filter by type (issue, project, document, initiative). Comma-separated.
        #[arg(long, rename_all = "lowercase")]
        r#type: Option<String>,

        /// Maximum number of results
        #[arg(long, default_value = "50")]
        limit: i32,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// Workflow state commands
    State {
        #[command(subcommand)]
        action: StateCommands,
    },
    /// Label commands
    Label {
        #[command(subcommand)]
        action: LabelCommands,
    },
    /// User commands
    User {
        #[command(subcommand)]
        action: UserCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum AuthCommands {
    /// Authenticate with Linear
    Login {
        /// Read token from stdin instead of prompting
        #[arg(long)]
        with_token: bool,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// Show authentication status
    Status {
        #[command(flatten)]
        format: FormatFlags,
    },
    /// Remove authentication token
    Logout {
        #[command(flatten)]
        format: FormatFlags,
    },
    /// Print the authentication token (for scripting)
    Token {
        #[command(flatten)]
        format: FormatFlags,
    },
}

#[derive(Args, Debug, Clone, Default)]
pub struct IssueUpdatePatchArgs {
    /// New issue title
    #[arg(long)]
    pub title: Option<String>,

    /// New issue description
    #[arg(long)]
    pub description: Option<String>,

    /// Assignee reference (@me, email, ID, or null to clear)
    #[arg(long)]
    pub assignee: Option<String>,

    /// Project reference (slug, ID, or null to clear)
    #[arg(long)]
    pub project: Option<String>,

    /// Workflow state name or ID
    #[arg(long)]
    pub state: Option<String>,

    /// Priority: 0=None, 1=Urgent, 2=High, 3=Medium, 4=Low
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=4))]
    pub priority: Option<u8>,
}

impl IssueUpdatePatchArgs {
    #[must_use]
    pub fn has_any_field(&self) -> bool {
        self.title.is_some()
            || self.description.is_some()
            || self.assignee.is_some()
            || self.project.is_some()
            || self.state.is_some()
            || self.priority.is_some()
    }
}

#[derive(Subcommand, Debug)]
pub enum IssueLifecycleCommands {
    /// Archive an issue
    Archive {
        /// Issue identifier (e.g., ENG-123)
        identifier: String,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// Unarchive an issue
    Unarchive {
        /// Issue identifier (e.g., ENG-123)
        identifier: String,

        #[command(flatten)]
        format: FormatFlags,
    },
}

#[derive(Subcommand, Debug)]
pub enum IssueRelationCommands {
    /// Link two issues
    Link {
        /// Source issue identifier (e.g., ENG-123)
        identifier: String,

        /// Related issue identifier (e.g., ENG-456)
        related: String,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// Mark one issue as blocking another
    Block {
        /// Source issue identifier (e.g., ENG-123)
        identifier: String,

        /// Related issue identifier (e.g., ENG-456)
        related: String,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// Mark an issue as duplicate of another
    Duplicate {
        /// Source issue identifier (e.g., ENG-123)
        identifier: String,

        /// Related issue identifier (e.g., ENG-456)
        related: String,

        #[command(flatten)]
        format: FormatFlags,
    },
}

#[derive(Subcommand, Debug)]
pub enum IssueCommentCommands {
    /// Delete a comment
    Delete {
        /// Comment ID (UUID)
        id: String,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// Add a comment to an issue
    Add {
        /// Issue identifier (e.g., ENG-123)
        identifier: String,

        /// Comment body text
        #[arg(long)]
        body: String,

        #[command(flatten)]
        format: FormatFlags,
    },
}

#[derive(Subcommand, Debug)]
pub enum IssueCommands {
    /// View an issue by identifier
    View {
        /// Issue identifier (e.g., ENG-123)
        identifier: String,

        /// Include comments in the output
        #[arg(long)]
        with_comments: bool,

        /// Maximum number of comments to return (only used with --with-comments)
        #[arg(long, default_value = "50")]
        comment_limit: usize,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// List issues with optional filters
    List {
        /// Filter by assignee user ID (use @me for current user)
        #[arg(long)]
        assignee: Option<String>,
        /// Filter by project ID
        #[arg(long)]
        project: Option<String>,
        /// Maximum number of issues to return
        #[arg(long, default_value = "50")]
        limit: usize,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// Create a new issue
    Create {
        /// Team key or ID
        #[arg(long)]
        team: String,

        /// Issue title
        #[arg(long)]
        title: String,

        /// Issue description
        #[arg(long)]
        description: Option<String>,

        /// Assignee reference (@me, email, or ID)
        #[arg(long)]
        assignee: Option<String>,

        /// Project slug or ID
        #[arg(long)]
        project: Option<String>,

        /// Workflow state name or ID
        #[arg(long)]
        state: Option<String>,

        /// Priority: 0=None, 1=Urgent, 2=High, 3=Medium, 4=Low
        #[arg(long, value_parser = clap::value_parser!(u8).range(0..=4))]
        priority: Option<u8>,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// Update an existing issue
    Update {
        /// Issue identifier (e.g., ENG-123)
        identifier: String,

        #[command(flatten)]
        patch: IssueUpdatePatchArgs,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// Delete an issue
    Delete {
        /// Issue identifier (e.g., ENG-123)
        identifier: String,

        /// Permanently delete (skip 30-day grace period, admin only)
        #[arg(long)]
        permanently: bool,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// Issue lifecycle commands
    Lifecycle {
        #[command(subcommand)]
        action: IssueLifecycleCommands,
    },
    /// Issue relation commands
    Relation {
        #[command(subcommand)]
        action: IssueRelationCommands,
    },
    /// Issue comment commands
    Comment {
        #[command(subcommand)]
        action: IssueCommentCommands,
    },
    /// Search issues by text
    Search {
        /// Search term
        term: String,

        /// Boost results from a specific team (team key, e.g., ENG)
        #[arg(long)]
        team: Option<String>,

        /// Include comments in search
        #[arg(long)]
        include_comments: bool,

        /// Maximum number of results to return
        #[arg(long, default_value = "50")]
        limit: usize,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// List comments for an issue
    Comments {
        /// Issue identifier (e.g., ENG-123)
        issue_id: String,
        /// Maximum number of comments to return
        #[arg(long, default_value = "50")]
        limit: usize,

        #[command(flatten)]
        format: FormatFlags,
    },
}

#[derive(Subcommand, Debug)]
pub enum TeamCommands {
    /// View a team by ID
    View {
        /// Team ID
        id: String,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// List teams
    List {
        /// Maximum number of teams to return
        #[arg(long, default_value = "50")]
        limit: usize,

        #[command(flatten)]
        format: FormatFlags,
    },
}

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    /// View a project by ID
    View {
        /// Project ID
        id: String,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// List projects
    List {
        /// Maximum number of projects to return
        #[arg(long, default_value = "50")]
        limit: usize,

        #[command(flatten)]
        format: FormatFlags,
    },
}

#[derive(Subcommand, Debug)]
pub enum CycleCommands {
    /// View a cycle by ID
    View {
        /// Cycle ID
        id: String,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// List cycles
    List {
        /// Maximum number of cycles to return
        #[arg(long, default_value = "50")]
        limit: usize,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// Get the currently active cycle
    Current {
        #[command(flatten)]
        format: FormatFlags,
    },
}

#[derive(Subcommand, Debug)]
pub enum StateCommands {
    /// List workflow states
    List {
        /// Filter by team key (e.g., ENG)
        #[arg(long)]
        team: Option<String>,

        /// Maximum number of states to return
        #[arg(long, default_value = "50")]
        limit: usize,

        #[command(flatten)]
        format: FormatFlags,
    },
}

#[derive(Subcommand, Debug)]
pub enum LabelCommands {
    /// List issue labels
    List {
        /// Filter by team key (e.g., ENG)
        #[arg(long)]
        team: Option<String>,

        /// Maximum number of labels to return
        #[arg(long, default_value = "50")]
        limit: usize,

        #[command(flatten)]
        format: FormatFlags,
    },
}

#[derive(Subcommand, Debug)]
pub enum UserCommands {
    /// List users
    List {
        /// Maximum number of users to return
        #[arg(long, default_value = "50")]
        limit: usize,

        #[command(flatten)]
        format: FormatFlags,
    },
}
