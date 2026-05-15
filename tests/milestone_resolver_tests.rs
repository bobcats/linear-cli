use linear_cli::error::CliError;
use linear_cli::milestones::resolver::{
    MilestoneReferenceLookup, MilestoneReferenceResolver, ResolveMilestoneInput,
    ResolvedMilestonePatch,
};
use linear_cli::milestones::types::{Milestone, MilestoneProject};
use std::collections::HashMap;

#[derive(Default)]
struct MockLookup {
    by_id: HashMap<String, Milestone>,
    by_name: HashMap<(Option<String>, String), Vec<Milestone>>,
    projects: HashMap<String, String>,
}

impl MilestoneReferenceLookup for MockLookup {
    fn get_milestone_by_id(&self, _token: &str, id: &str) -> Result<Option<Milestone>, CliError> {
        Ok(self.by_id.get(id).cloned())
    }

    fn find_milestones_by_name(
        &self,
        _token: &str,
        name: &str,
        project_id: Option<&str>,
    ) -> Result<Vec<Milestone>, CliError> {
        Ok(self
            .by_name
            .get(&(project_id.map(str::to_string), name.to_string()))
            .cloned()
            .unwrap_or_default())
    }

    fn resolve_project_id_by_slug(
        &self,
        _token: &str,
        slug: &str,
    ) -> Result<Option<String>, CliError> {
        Ok(self.projects.get(slug).cloned())
    }
}

fn milestone(id: &str, name: &str, project_name: &str, project_id: &str) -> Milestone {
    Milestone {
        id: id.to_string(),
        name: name.to_string(),
        description: None,
        status: "next".to_string(),
        progress: 0.0,
        sort_order: 0.0,
        target_date: None,
        project: MilestoneProject {
            id: project_id.to_string(),
            name: project_name.to_string(),
            slug_id: project_name.to_lowercase(),
        },
        created_at: "2026-05-01T00:00:00Z".to_string(),
        updated_at: "2026-05-01T00:00:00Z".to_string(),
        archived_at: None,
    }
}

#[test]
fn uuid_resolves_via_direct_lookup() {
    let id = "123e4567-e89b-12d3-a456-426614174000";
    let mut lookup = MockLookup::default();
    lookup
        .by_id
        .insert(id.to_string(), milestone(id, "Beta", "App", "project-1"));
    let resolver = MilestoneReferenceResolver::new(&lookup);

    let resolved = resolver.resolve_required_id("token", id, None).unwrap();

    assert_eq!(resolved, id);
}

#[test]
fn linear_url_parses_final_segment_and_resolves_direct_lookup() {
    let id = "123e4567-e89b-12d3-a456-426614174000";
    let mut lookup = MockLookup::default();
    lookup
        .by_id
        .insert(id.to_string(), milestone(id, "Beta", "App", "project-1"));
    let resolver = MilestoneReferenceResolver::new(&lookup);

    let resolved = resolver
        .resolve_required_id(
            "token",
            &format!("https://linear.app/acme/project/milestone/{id}"),
            None,
        )
        .unwrap();

    assert_eq!(resolved, id);
}

#[test]
fn global_unique_name_resolves() {
    let mut lookup = MockLookup::default();
    lookup.by_name.insert(
        (None, "Beta".to_string()),
        vec![milestone("milestone-1", "Beta", "App", "project-1")],
    );
    let resolver = MilestoneReferenceResolver::new(&lookup);

    let resolved = resolver.resolve_required_id("token", "Beta", None).unwrap();

    assert_eq!(resolved, "milestone-1");
}

#[test]
fn scoped_name_uses_project_id_and_resolves() {
    let mut lookup = MockLookup::default();
    lookup
        .projects
        .insert("APP".to_string(), "project-1".to_string());
    lookup.by_name.insert(
        (Some("project-1".to_string()), "Beta".to_string()),
        vec![milestone("milestone-1", "Beta", "App", "project-1")],
    );
    let resolver = MilestoneReferenceResolver::new(&lookup);

    let resolved = resolver
        .resolve_required_id("token", "Beta", Some("APP".to_string()))
        .unwrap();

    assert_eq!(resolved, "milestone-1");
}

#[test]
fn missing_name_returns_not_found() {
    let lookup = MockLookup::default();
    let resolver = MilestoneReferenceResolver::new(&lookup);

    let result = resolver.resolve_required_id("token", "Missing", None);

    assert!(matches!(result, Err(CliError::NotFound(message)) if message.contains("Missing")));
}

#[test]
fn ambiguous_global_name_mentions_project_names_and_project_flag() {
    let mut lookup = MockLookup::default();
    lookup.by_name.insert(
        (None, "Beta".to_string()),
        vec![
            milestone("milestone-1", "Beta", "App", "project-1"),
            milestone("milestone-2", "Beta", "Web", "project-2"),
        ],
    );
    let resolver = MilestoneReferenceResolver::new(&lookup);

    let result = resolver.resolve_required_id("token", "Beta", None);

    assert!(
        matches!(result, Err(CliError::InvalidArgs(message)) if message.contains("--project") && message.contains("App") && message.contains("Web"))
    );
}

#[test]
fn null_returns_clear_patch_when_allowed() {
    let lookup = MockLookup::default();
    let resolver = MilestoneReferenceResolver::new(&lookup);

    let resolved = resolver
        .resolve_patch(
            "token",
            ResolveMilestoneInput {
                reference: Some("null".to_string()),
                project: None,
                allow_null_clear: true,
            },
        )
        .unwrap();

    assert_eq!(resolved, ResolvedMilestonePatch::Clear);
}

#[test]
fn null_is_invalid_for_crud_target_resolution() {
    let lookup = MockLookup::default();
    let resolver = MilestoneReferenceResolver::new(&lookup);

    let result = resolver.resolve_required_id("token", "null", None);

    assert!(matches!(result, Err(CliError::InvalidArgs(message)) if message.contains("null")));
}
