use linear_cli::error::CliError;
use linear_cli::issues::resolver::{
    IssueReferenceLookup, IssueReferenceResolver, ResolveIssueRefsInput,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct MockLookup {
    viewer_id: Option<String>,
    user_by_email: HashMap<String, String>,
    team_by_key: HashMap<String, String>,
    project_by_slug: HashMap<String, String>,
    state_by_name: HashMap<String, String>,
    call_log: Arc<Mutex<Vec<String>>>,
    viewer_error: Option<CliError>,
}

impl MockLookup {
    fn with_defaults() -> Self {
        let mut lookup = Self {
            viewer_id: Some("viewer-123".to_string()),
            ..Default::default()
        };
        lookup
            .user_by_email
            .insert("alice@example.com".to_string(), "user-42".to_string());
        lookup
            .team_by_key
            .insert("ENG".to_string(), "team-1".to_string());
        lookup
            .project_by_slug
            .insert("api-platform".to_string(), "project-1".to_string());
        lookup
            .state_by_name
            .insert("In Progress".to_string(), "state-2".to_string());
        lookup
    }

    fn call_log(&self) -> Vec<String> {
        self.call_log.lock().unwrap().clone()
    }
}

impl IssueReferenceLookup for MockLookup {
    fn resolve_viewer_id(&self, _token: &str) -> Result<String, CliError> {
        self.call_log
            .lock()
            .unwrap()
            .push("resolve_viewer_id".to_string());

        if let Some(err) = &self.viewer_error {
            return Err(err.clone());
        }

        self.viewer_id
            .clone()
            .ok_or_else(|| CliError::NotFound("Viewer not found".to_string()))
    }

    fn resolve_user_id_by_email(
        &self,
        _token: &str,
        email: &str,
    ) -> Result<Option<String>, CliError> {
        self.call_log
            .lock()
            .unwrap()
            .push(format!("resolve_user_id_by_email:{email}"));
        Ok(self.user_by_email.get(email).cloned())
    }

    fn resolve_team_id_by_key(&self, _token: &str, key: &str) -> Result<Option<String>, CliError> {
        self.call_log
            .lock()
            .unwrap()
            .push(format!("resolve_team_id_by_key:{key}"));
        Ok(self.team_by_key.get(key).cloned())
    }

    fn resolve_project_id_by_slug(
        &self,
        _token: &str,
        slug: &str,
    ) -> Result<Option<String>, CliError> {
        self.call_log
            .lock()
            .unwrap()
            .push(format!("resolve_project_id_by_slug:{slug}"));
        Ok(self.project_by_slug.get(slug).cloned())
    }

    fn resolve_state_id_by_name(
        &self,
        _token: &str,
        name: &str,
    ) -> Result<Option<String>, CliError> {
        self.call_log
            .lock()
            .unwrap()
            .push(format!("resolve_state_id_by_name:{name}"));
        Ok(self.state_by_name.get(name).cloned())
    }
}

#[test]
fn resolves_assignee_me_to_viewer_id() {
    let lookup = MockLookup::with_defaults();
    let resolver = IssueReferenceResolver::new(&lookup);

    let input = ResolveIssueRefsInput {
        team: None,
        assignee: Some("@me".to_string()),
        project: None,
        state: None,
    };

    let resolved = resolver.resolve("test-token", &input).unwrap();

    assert_eq!(resolved.assignee_id.as_deref(), Some("viewer-123"));
    assert!(lookup.call_log().contains(&"resolve_viewer_id".to_string()));
}

#[test]
fn resolves_assignee_email_to_user_id() {
    let lookup = MockLookup::with_defaults();
    let resolver = IssueReferenceResolver::new(&lookup);

    let input = ResolveIssueRefsInput {
        team: None,
        assignee: Some("alice@example.com".to_string()),
        project: None,
        state: None,
    };

    let resolved = resolver.resolve("test-token", &input).unwrap();

    assert_eq!(resolved.assignee_id.as_deref(), Some("user-42"));
    assert!(
        lookup
            .call_log()
            .contains(&"resolve_user_id_by_email:alice@example.com".to_string())
    );
}

#[test]
fn passes_through_uuid_like_assignee_id_without_lookup() {
    let lookup = MockLookup::with_defaults();
    let resolver = IssueReferenceResolver::new(&lookup);

    let assignee_id = "123e4567-e89b-12d3-a456-426614174000";
    let input = ResolveIssueRefsInput {
        team: None,
        assignee: Some(assignee_id.to_string()),
        project: None,
        state: None,
    };

    let resolved = resolver.resolve("test-token", &input).unwrap();

    assert_eq!(resolved.assignee_id.as_deref(), Some(assignee_id));
    assert!(
        !lookup
            .call_log()
            .iter()
            .any(|call| call.starts_with("resolve_user_id_by_email"))
    );
}

#[test]
fn resolves_team_key_project_slug_and_state_name() {
    let lookup = MockLookup::with_defaults();
    let resolver = IssueReferenceResolver::new(&lookup);

    let input = ResolveIssueRefsInput {
        team: Some("ENG".to_string()),
        assignee: None,
        project: Some("api-platform".to_string()),
        state: Some("In Progress".to_string()),
    };

    let resolved = resolver.resolve("test-token", &input).unwrap();

    assert_eq!(resolved.team_id.as_deref(), Some("team-1"));
    assert_eq!(resolved.project_id.as_deref(), Some("project-1"));
    assert_eq!(resolved.state_id.as_deref(), Some("state-2"));
}

#[test]
fn unresolved_project_slug_fails_fast_with_not_found() {
    let lookup = MockLookup::with_defaults();
    let resolver = IssueReferenceResolver::new(&lookup);

    let input = ResolveIssueRefsInput {
        team: None,
        assignee: None,
        project: Some("does-not-exist".to_string()),
        state: None,
    };

    let error = resolver.resolve("test-token", &input).unwrap_err();

    match error {
        CliError::NotFound(message) => assert!(message.contains("project")),
        _ => panic!("expected NotFound"),
    }
}

#[test]
fn unresolved_state_name_fails_fast_with_not_found() {
    let lookup = MockLookup::with_defaults();
    let resolver = IssueReferenceResolver::new(&lookup);

    let input = ResolveIssueRefsInput {
        team: None,
        assignee: None,
        project: None,
        state: Some("Not a Real State".to_string()),
    };

    let error = resolver.resolve("test-token", &input).unwrap_err();

    match error {
        CliError::NotFound(message) => assert!(message.contains("state")),
        _ => panic!("expected NotFound"),
    }
}

#[test]
fn viewer_lookup_auth_error_is_propagated_for_me_resolution() {
    let lookup = MockLookup {
        viewer_error: Some(CliError::AuthError("token invalid".to_string())),
        ..MockLookup::with_defaults()
    };
    let resolver = IssueReferenceResolver::new(&lookup);

    let input = ResolveIssueRefsInput {
        team: None,
        assignee: Some("@me".to_string()),
        project: None,
        state: None,
    };

    let error = resolver.resolve("test-token", &input).unwrap_err();

    match error {
        CliError::AuthError(message) => assert!(message.contains("token invalid")),
        _ => panic!("expected AuthError"),
    }
}
