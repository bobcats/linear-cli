use crate::client::LinearClient;
use crate::client::queries::{
    SemanticSearchQuery, SemanticSearchQueryVariables, SemanticSearchResultType,
};
use crate::error::CliError;
use crate::search::types::SemanticSearchResult;
use cynic::QueryBuilder;

/// Trait for semantic search operations
pub trait SemanticSearchClient: Send + Sync {
    fn semantic_search(
        &self,
        token: &str,
        query: &str,
        types: Option<Vec<SemanticSearchResultType>>,
        max_results: Option<i32>,
    ) -> Result<Vec<SemanticSearchResult>, CliError>;
}

impl SemanticSearchClient for LinearClient {
    fn semantic_search(
        &self,
        token: &str,
        query: &str,
        types: Option<Vec<SemanticSearchResultType>>,
        max_results: Option<i32>,
    ) -> Result<Vec<SemanticSearchResult>, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        let operation = SemanticSearchQuery::build(SemanticSearchQueryVariables {
            query: query.to_string(),
            max_results,
            types,
        });

        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        let payload = response
            .data
            .ok_or_else(|| CliError::General("No data returned".to_string()))?
            .semantic_search;

        Ok(payload
            .results
            .into_iter()
            .map(|node| {
                let (title, identifier, url) = match node.result_type {
                    SemanticSearchResultType::Issue => {
                        if let Some(issue) = node.issue {
                            (issue.title, Some(issue.identifier), Some(issue.url))
                        } else {
                            ("Unknown issue".to_string(), None, None)
                        }
                    }
                    SemanticSearchResultType::Project => {
                        if let Some(project) = node.project {
                            (project.name, None, Some(project.url))
                        } else {
                            ("Unknown project".to_string(), None, None)
                        }
                    }
                    SemanticSearchResultType::Document => {
                        if let Some(doc) = node.document {
                            (doc.title, None, None)
                        } else {
                            ("Unknown document".to_string(), None, None)
                        }
                    }
                    SemanticSearchResultType::Initiative => {
                        if let Some(init) = node.initiative {
                            (init.name, None, None)
                        } else {
                            ("Unknown initiative".to_string(), None, None)
                        }
                    }
                };

                let type_str = match node.result_type {
                    SemanticSearchResultType::Issue => "issue",
                    SemanticSearchResultType::Project => "project",
                    SemanticSearchResultType::Document => "document",
                    SemanticSearchResultType::Initiative => "initiative",
                };

                SemanticSearchResult {
                    id: node.id.inner().to_string(),
                    result_type: type_str.to_string(),
                    title,
                    identifier,
                    url,
                }
            })
            .collect())
    }
}

/// Mock implementation for testing
pub struct MockSemanticSearchClient {
    pub search_result: Result<Vec<SemanticSearchResult>, CliError>,
}

impl SemanticSearchClient for MockSemanticSearchClient {
    fn semantic_search(
        &self,
        _token: &str,
        _query: &str,
        _types: Option<Vec<SemanticSearchResultType>>,
        _max_results: Option<i32>,
    ) -> Result<Vec<SemanticSearchResult>, CliError> {
        self.search_result.clone()
    }
}
