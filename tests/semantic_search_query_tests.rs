use cynic::QueryBuilder;
use linear_cli::client::queries::{
    SemanticSearchQuery, SemanticSearchQueryVariables, SemanticSearchResultType,
};

#[test]
fn test_semantic_search_query_serializes_with_query() {
    let operation = SemanticSearchQuery::build(SemanticSearchQueryVariables {
        query: "authentication flow".to_string(),
        max_results: Some(10),
        types: None,
    });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert_eq!(json["variables"]["query"], "authentication flow");
    assert_eq!(json["variables"]["maxResults"], 10);
}

#[test]
fn test_semantic_search_query_with_type_filter() {
    let operation = SemanticSearchQuery::build(SemanticSearchQueryVariables {
        query: "bidding".to_string(),
        max_results: Some(25),
        types: Some(vec![SemanticSearchResultType::Issue]),
    });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert_eq!(json["variables"]["query"], "bidding");
    let types = json["variables"]["types"]
        .as_array()
        .expect("types should be array");
    assert_eq!(types.len(), 1);
    assert_eq!(types[0], "issue");
}

#[test]
fn test_semantic_search_query_omits_optional_fields() {
    let operation = SemanticSearchQuery::build(SemanticSearchQueryVariables {
        query: "test".to_string(),
        max_results: None,
        types: None,
    });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert!(
        json["variables"].get("maxResults").is_none() || json["variables"]["maxResults"].is_null()
    );
}
