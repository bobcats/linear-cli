use crate::client::queries::{ProjectMilestoneCreateInput as QueryCreateInput, ProjectMilestoneCreateMutation, ProjectMilestoneCreateMutationVariables, ProjectMilestoneDeleteMutation, ProjectMilestoneDeleteMutationVariables, ProjectMilestoneQuery, ProjectMilestoneQueryVariables, ProjectMilestoneUpdateInput as QueryUpdateInput, ProjectMilestoneUpdateMutation, ProjectMilestoneUpdateMutationVariables, ProjectMilestonesForProjectQuery, ProjectMilestonesForProjectQueryVariables, ProjectMilestonesQuery, ProjectMilestonesQueryVariables, TimelessDate};
use crate::client::LinearClient;
use crate::error::CliError;
use crate::milestones::types::Milestone;
use cynic::{MutationBuilder, QueryBuilder};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CreateMilestoneInput { pub project_id: String, pub name: String, pub description: Option<String>, pub target_date: Option<String> }

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UpdateMilestoneInput { pub name: Option<String>, pub description: Option<String>, pub project_id: Option<String>, pub target_date: Option<String> }

pub trait MilestoneClient: Send + Sync {
    fn get_milestone(&self, token: &str, id: &str) -> Result<Milestone, CliError>;
    fn list_milestones(&self, token: &str, project_id: Option<&str>, name: Option<&str>, limit: usize) -> Result<Vec<Milestone>, CliError>;
    fn create_milestone(&self, token: &str, input: CreateMilestoneInput) -> Result<Milestone, CliError>;
    fn update_milestone(&self, token: &str, id: &str, input: UpdateMilestoneInput) -> Result<Milestone, CliError>;
    fn delete_milestone(&self, token: &str, id: &str) -> Result<(), CliError>;
}

impl MilestoneClient for LinearClient {
    fn get_milestone(&self, token: &str, id: &str) -> Result<Milestone, CliError> {
        if token.is_empty() { return Err(CliError::auth_error("Token cannot be empty")); }
        let response = self.execute_query(token, ProjectMilestoneQuery::build(ProjectMilestoneQueryVariables { id: id.to_string() }), crate::client::GraphQlErrorType::General)?;
        Ok(response.data.ok_or_else(|| CliError::NotFound(format!("Milestone {id} not found")))?.project_milestone.into())
    }

    fn list_milestones(&self, token: &str, project_id: Option<&str>, name: Option<&str>, limit: usize) -> Result<Vec<Milestone>, CliError> {
        if token.is_empty() { return Err(CliError::auth_error("Token cannot be empty")); }
        let first = Some(limit as i32);
        let name = name.map(str::to_string);
        let nodes = if let Some(project_id) = project_id {
            self.execute_query(token, ProjectMilestonesForProjectQuery::build(ProjectMilestonesForProjectQueryVariables { id: project_id.to_string(), first, name }), crate::client::GraphQlErrorType::General)?.data.ok_or_else(|| CliError::General("No data returned".to_string()))?.project.project_milestones.nodes
        } else {
            self.execute_query(token, ProjectMilestonesQuery::build(ProjectMilestonesQueryVariables { first, name }), crate::client::GraphQlErrorType::General)?.data.ok_or_else(|| CliError::General("No data returned".to_string()))?.project_milestones.nodes
        };
        Ok(nodes.into_iter().map(Into::into).collect())
    }

    fn create_milestone(&self, token: &str, input: CreateMilestoneInput) -> Result<Milestone, CliError> {
        if token.is_empty() { return Err(CliError::auth_error("Token cannot be empty")); }
        let response = self.execute_query(token, ProjectMilestoneCreateMutation::build(ProjectMilestoneCreateMutationVariables { input: QueryCreateInput { project_id: input.project_id, name: input.name, description: input.description, target_date: input.target_date.map(TimelessDate) } }), crate::client::GraphQlErrorType::General)?;
        Ok(response.data.ok_or_else(|| CliError::General("No data returned".to_string()))?.project_milestone_create.project_milestone.into())
    }

    fn update_milestone(&self, token: &str, id: &str, input: UpdateMilestoneInput) -> Result<Milestone, CliError> {
        if token.is_empty() { return Err(CliError::auth_error("Token cannot be empty")); }
        let response = self.execute_query(token, ProjectMilestoneUpdateMutation::build(ProjectMilestoneUpdateMutationVariables { id: id.to_string(), input: QueryUpdateInput { name: input.name, description: input.description, project_id: input.project_id, target_date: input.target_date.map(TimelessDate) } }), crate::client::GraphQlErrorType::General)?;
        Ok(response.data.ok_or_else(|| CliError::General("No data returned".to_string()))?.project_milestone_update.project_milestone.into())
    }

    fn delete_milestone(&self, token: &str, id: &str) -> Result<(), CliError> {
        if token.is_empty() { return Err(CliError::auth_error("Token cannot be empty")); }
        let response = self.execute_query(token, ProjectMilestoneDeleteMutation::build(ProjectMilestoneDeleteMutationVariables { id: id.to_string() }), crate::client::GraphQlErrorType::General)?;
        let payload = response.data.ok_or_else(|| CliError::General("No data returned".to_string()))?.project_milestone_delete;
        if payload.success { Ok(()) } else { Err(CliError::General(format!("Failed to delete milestone {id}"))) }
    }
}
