use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

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
    /// Project milestone commands
    Milestone {
        #[command(subcommand)]
        action: MilestoneCommands,
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

    /// Read issue description from a file
    #[arg(long, value_name = "PATH")]
    pub description_file: Option<PathBuf>,

    /// Assignee reference (@me, email, ID, or null to clear)
    #[arg(long)]
    pub assignee: Option<String>,

    /// Project reference (slug, ID, or null to clear)
    #[arg(long)]
    pub project: Option<String>,

    /// Workflow state name or ID
    #[arg(long)]
    pub state: Option<String>,

    /// Parent issue identifier (e.g., ENG-123), ID, or null to clear
    #[arg(long)]
    pub parent: Option<String>,

    /// Priority: 0=None, 1=Urgent, 2=High, 3=Medium, 4=Low
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=4))]
    pub priority: Option<u8>,

    /// Project milestone reference, or null to clear
    #[arg(long)]
    pub milestone: Option<String>,
}

impl IssueUpdatePatchArgs {
    #[must_use]
    pub fn has_any_field(&self) -> bool {
        self.title.is_some()
            || self.description.is_some()
            || self.description_file.is_some()
            || self.assignee.is_some()
            || self.project.is_some()
            || self.state.is_some()
            || self.parent.is_some()
            || self.priority.is_some()
            || self.milestone.is_some()
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
        body: Option<String>,

        /// Read comment body text from a file
        #[arg(long, value_name = "PATH")]
        body_file: Option<PathBuf>,

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

        /// Read issue description from a file
        #[arg(long, value_name = "PATH")]
        description_file: Option<PathBuf>,

        /// Assignee reference (@me, email, or ID)
        #[arg(long)]
        assignee: Option<String>,

        /// Project slug or ID
        #[arg(long)]
        project: Option<String>,

        /// Workflow state name or ID
        #[arg(long)]
        state: Option<String>,

        /// Parent issue identifier (e.g., ENG-123) or ID
        #[arg(long)]
        parent: Option<String>,

        /// Priority: 0=None, 1=Urgent, 2=High, 3=Medium, 4=Low
        #[arg(long, value_parser = clap::value_parser!(u8).range(0..=4))]
        priority: Option<u8>,

        /// Project milestone reference
        #[arg(long)]
        milestone: Option<String>,

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
pub enum MilestoneCommands {
    /// View a milestone by ID, URL, or unique name
    View {
        /// Milestone reference
        reference: String,

        /// Project reference to scope name resolution
        #[arg(long)]
        project: Option<String>,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// List milestones
    List {
        /// Project reference to scope milestone listing
        #[arg(long)]
        project: Option<String>,

        /// Maximum number of milestones to return
        #[arg(long, default_value = "50")]
        limit: usize,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// Create a milestone
    Create {
        /// Project reference
        #[arg(long)]
        project: String,

        /// Milestone name
        #[arg(long)]
        name: String,

        /// Milestone description
        #[arg(long)]
        description: Option<String>,

        /// Target date in YYYY-MM-DD format
        #[arg(long, value_parser = parse_timeless_date)]
        target_date: Option<String>,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// Update a milestone
    Update {
        /// Milestone reference
        reference: String,

        /// Project reference to scope name resolution or move milestone
        #[arg(long)]
        project: Option<String>,

        /// New milestone name
        #[arg(long)]
        name: Option<String>,

        /// New milestone description
        #[arg(long)]
        description: Option<String>,

        /// New target date in YYYY-MM-DD format
        #[arg(long, value_parser = parse_timeless_date)]
        target_date: Option<String>,

        #[command(flatten)]
        format: FormatFlags,
    },
    /// Delete a milestone
    Delete {
        /// Milestone reference
        reference: String,

        /// Project reference to scope name resolution
        #[arg(long)]
        project: Option<String>,

        #[command(flatten)]
        format: FormatFlags,
    },
}

fn parse_timeless_date(value: &str) -> Result<String, String> {
    if value.len() != 10
        || value.as_bytes().get(4) != Some(&b'-')
        || value.as_bytes().get(7) != Some(&b'-')
        || !value
            .chars()
            .enumerate()
            .all(|(index, ch)| matches!(index, 4 | 7) || ch.is_ascii_digit())
    {
        return Err("date must use YYYY-MM-DD format".to_string());
    }

    let year: i32 = value[0..4]
        .parse()
        .map_err(|_| "date must use YYYY-MM-DD format".to_string())?;
    let month: u32 = value[5..7]
        .parse()
        .map_err(|_| "date must use YYYY-MM-DD format".to_string())?;
    let day: u32 = value[8..10]
        .parse()
        .map_err(|_| "date must use YYYY-MM-DD format".to_string())?;

    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => return Err("date must be a valid calendar date".to_string()),
    };

    if day == 0 || day > max_day {
        return Err("date must be a valid calendar date".to_string());
    }

    Ok(value.to_string())
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
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
