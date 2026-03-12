use crate::client::LinearClient;
use crate::client::queries::{
    ProjectQuery, ProjectQueryVariables, ProjectsQuery, ProjectsQueryVariables,
};
use crate::error::CliError;
use crate::projects::types::Project;
use cynic::QueryBuilder;

/// Trait for project operations with Linear API
pub trait ProjectClient: Send + Sync {
    /// Get a project by ID
    fn get_project(&self, token: &str, id: &str) -> Result<Project, CliError>;

    /// List projects with optional filters
    fn list_projects(&self, token: &str, limit: usize) -> Result<Vec<Project>, CliError>;
}

/// Production implementation using Linear GraphQL API
impl ProjectClient for LinearClient {
    fn get_project(&self, token: &str, id: &str) -> Result<Project, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        // Build the project query using Cynic
        let operation = ProjectQuery::build(ProjectQueryVariables { id: id.to_string() });

        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        // Extract project data
        let project_node = response
            .data
            .ok_or_else(|| CliError::NotFound(format!("Project {id} not found")))?
            .project;

        // Convert from Cynic types to our public Project type using From
        Ok(project_node.into())
    }

    fn list_projects(&self, token: &str, limit: usize) -> Result<Vec<Project>, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        // Build the projects query using Cynic
        let operation = ProjectsQuery::build(ProjectsQueryVariables {
            first: Some(limit as i32),
        });

        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        // Extract projects from response
        let projects_connection = response
            .data
            .ok_or_else(|| CliError::General("No data returned".to_string()))?
            .projects;

        // Convert from Cynic types to our public Project type using From
        Ok(projects_connection
            .nodes
            .into_iter()
            .map(Into::into)
            .collect())
    }
}

/// Mock implementation for testing
pub struct MockProjectClient {
    pub result: Result<Project, CliError>,
    pub list_result: Result<Vec<Project>, CliError>,
}

impl ProjectClient for MockProjectClient {
    fn get_project(&self, _token: &str, _id: &str) -> Result<Project, CliError> {
        self.result.clone()
    }

    fn list_projects(&self, _token: &str, _limit: usize) -> Result<Vec<Project>, CliError> {
        self.list_result.clone()
    }
}
