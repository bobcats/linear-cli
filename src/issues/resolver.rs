use crate::error::CliError;

/// Raw user-provided references for issue create/update commands.
#[derive(Debug, Clone, Default)]
pub struct ResolveIssueRefsInput {
    pub team: Option<String>,
    pub assignee: Option<String>,
    pub project: Option<String>,
    pub state: Option<String>,
}

/// Resolved IDs ready to send in Linear GraphQL mutation inputs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResolvedIssueRefs {
    pub team_id: Option<String>,
    pub assignee_id: Option<String>,
    pub project_id: Option<String>,
    pub state_id: Option<String>,
}

/// Lookup operations required to resolve user-friendly references to IDs.
pub trait IssueReferenceLookup: Send + Sync {
    fn resolve_viewer_id(&self, token: &str) -> Result<String, CliError>;

    fn resolve_user_id_by_email(
        &self,
        token: &str,
        email: &str,
    ) -> Result<Option<String>, CliError>;

    fn resolve_team_id_by_key(&self, token: &str, key: &str) -> Result<Option<String>, CliError>;

    fn resolve_project_id_by_slug(
        &self,
        token: &str,
        slug: &str,
    ) -> Result<Option<String>, CliError>;

    fn resolve_state_id_by_name(&self, token: &str, name: &str)
    -> Result<Option<String>, CliError>;
}

/// Resolves user-facing references (e.g. `@me`, emails, team keys) into IDs.
pub struct IssueReferenceResolver<'a> {
    lookup: &'a dyn IssueReferenceLookup,
}

impl<'a> IssueReferenceResolver<'a> {
    #[must_use]
    pub fn new(lookup: &'a dyn IssueReferenceLookup) -> Self {
        Self { lookup }
    }

    pub fn resolve(
        &self,
        token: &str,
        input: &ResolveIssueRefsInput,
    ) -> Result<ResolvedIssueRefs, CliError> {
        let assignee_id = self.resolve_assignee(token, input.assignee.as_deref())?;
        let team_id = self.resolve_team(token, input.team.as_deref())?;
        let project_id = self.resolve_project(token, input.project.as_deref())?;
        let state_id = self.resolve_state(token, input.state.as_deref())?;

        Ok(ResolvedIssueRefs {
            team_id,
            assignee_id,
            project_id,
            state_id,
        })
    }

    fn resolve_assignee(
        &self,
        token: &str,
        value: Option<&str>,
    ) -> Result<Option<String>, CliError> {
        let Some(value) = value else {
            return Ok(None);
        };

        if value == "@me" {
            return self.lookup.resolve_viewer_id(token).map(Some);
        }

        if value.contains('@') {
            let user_id = self
                .lookup
                .resolve_user_id_by_email(token, value)?
                .ok_or_else(|| {
                    CliError::NotFound(format!("assignee not found for email: {value}"))
                })?;
            return Ok(Some(user_id));
        }

        Ok(Some(value.to_string()))
    }

    fn resolve_team(&self, token: &str, value: Option<&str>) -> Result<Option<String>, CliError> {
        let Some(value) = value else {
            return Ok(None);
        };

        if is_uuid_like(value) {
            return Ok(Some(value.to_string()));
        }

        let team_id = self
            .lookup
            .resolve_team_id_by_key(token, value)?
            .ok_or_else(|| CliError::NotFound(format!("team not found for key: {value}")))?;

        Ok(Some(team_id))
    }

    fn resolve_project(
        &self,
        token: &str,
        value: Option<&str>,
    ) -> Result<Option<String>, CliError> {
        let Some(value) = value else {
            return Ok(None);
        };

        if is_uuid_like(value) {
            return Ok(Some(value.to_string()));
        }

        let project_id = self
            .lookup
            .resolve_project_id_by_slug(token, value)?
            .ok_or_else(|| CliError::NotFound(format!("project not found for slug: {value}")))?;

        Ok(Some(project_id))
    }

    fn resolve_state(&self, token: &str, value: Option<&str>) -> Result<Option<String>, CliError> {
        let Some(value) = value else {
            return Ok(None);
        };

        if is_uuid_like(value) {
            return Ok(Some(value.to_string()));
        }

        let state_id = self
            .lookup
            .resolve_state_id_by_name(token, value)?
            .ok_or_else(|| CliError::NotFound(format!("state not found for name: {value}")))?;

        Ok(Some(state_id))
    }
}

fn is_uuid_like(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 36 {
        return false;
    }

    // xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    for (idx, b) in bytes.iter().enumerate() {
        let is_dash = matches!(idx, 8 | 13 | 18 | 23);
        if is_dash {
            if *b != b'-' {
                return false;
            }
            continue;
        }

        if !b.is_ascii_hexdigit() {
            return false;
        }
    }

    true
}
