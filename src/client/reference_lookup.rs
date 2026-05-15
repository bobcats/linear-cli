use crate::client::LinearClient;
use crate::client::auth::AuthClient;
use crate::client::issues::IssueClient;
use crate::client::milestones::MilestoneClient;
use crate::client::projects::ProjectClient;
use crate::client::states::StateClient;
use crate::client::teams::TeamClient;
use crate::client::users::UserClient;
use crate::error::CliError;
use crate::issues::resolver::IssueReferenceLookup;
use crate::milestones::resolver::MilestoneReferenceLookup;

impl MilestoneReferenceLookup for LinearClient {
    fn get_milestone_by_id(
        &self,
        token: &str,
        id: &str,
    ) -> Result<Option<crate::milestones::types::Milestone>, CliError> {
        match self.get_milestone(token, id) {
            Ok(milestone) => Ok(Some(milestone)),
            Err(CliError::NotFound(_)) => Ok(None),
            Err(error) => Err(error),
        }
    }

    fn find_milestones_by_name(
        &self,
        token: &str,
        name: &str,
        project_id: Option<&str>,
    ) -> Result<Vec<crate::milestones::types::Milestone>, CliError> {
        self.list_milestones(token, project_id, Some(name), 250)
    }

    fn resolve_project_id_by_slug(
        &self,
        token: &str,
        slug: &str,
    ) -> Result<Option<String>, CliError> {
        let projects = self.list_projects(token, 250)?;
        Ok(projects
            .into_iter()
            .find(|project| project.slug_id == slug)
            .map(|project| project.id))
    }
}

impl IssueReferenceLookup for LinearClient {
    fn resolve_viewer_id(&self, token: &str) -> Result<String, CliError> {
        let user_info = self.validate_token(token)?;
        Ok(user_info.id)
    }

    fn resolve_user_id_by_email(
        &self,
        token: &str,
        email: &str,
    ) -> Result<Option<String>, CliError> {
        let users = self.list_users(token, 250)?;
        Ok(users.into_iter().find(|u| u.email == email).map(|u| u.id))
    }

    fn resolve_team_id_by_key(&self, token: &str, key: &str) -> Result<Option<String>, CliError> {
        let teams = self.list_teams(token, 250)?;
        Ok(teams.into_iter().find(|t| t.key == key).map(|t| t.id))
    }

    fn resolve_project_id_by_slug(
        &self,
        token: &str,
        slug: &str,
    ) -> Result<Option<String>, CliError> {
        let projects = self.list_projects(token, 250)?;
        Ok(projects
            .into_iter()
            .find(|p| p.slug_id == slug)
            .map(|p| p.id))
    }

    fn resolve_state_id_by_name(
        &self,
        token: &str,
        name: &str,
    ) -> Result<Option<String>, CliError> {
        let states = self.list_states(token, None, 250)?;
        Ok(states.into_iter().find(|s| s.name == name).map(|s| s.id))
    }

    fn resolve_issue_id_by_identifier(
        &self,
        token: &str,
        identifier: &str,
    ) -> Result<Option<String>, CliError> {
        match self.get_issue(token, identifier) {
            Ok(issue) => Ok(Some(issue.id)),
            Err(CliError::NotFound(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
