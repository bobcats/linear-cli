use crate::error::CliError;
use crate::milestones::types::Milestone;

pub trait MilestoneReferenceLookup: Send + Sync {
    fn get_milestone_by_id(&self, token: &str, id: &str) -> Result<Option<Milestone>, CliError>;

    fn find_milestones_by_name(
        &self,
        token: &str,
        name: &str,
        project_id: Option<&str>,
    ) -> Result<Vec<Milestone>, CliError>;

    fn resolve_project_id_by_slug(
        &self,
        token: &str,
        slug: &str,
    ) -> Result<Option<String>, CliError>;
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResolveMilestoneInput {
    pub reference: Option<String>,
    pub project: Option<String>,
    pub allow_null_clear: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedMilestonePatch {
    Unchanged,
    Set(String),
    Clear,
}

pub struct MilestoneReferenceResolver<'a> {
    lookup: &'a dyn MilestoneReferenceLookup,
}

impl<'a> MilestoneReferenceResolver<'a> {
    #[must_use]
    pub fn new(lookup: &'a dyn MilestoneReferenceLookup) -> Self {
        Self { lookup }
    }

    pub fn resolve_patch(
        &self,
        token: &str,
        input: ResolveMilestoneInput,
    ) -> Result<ResolvedMilestonePatch, CliError> {
        let Some(reference) = input.reference else {
            return Ok(ResolvedMilestonePatch::Unchanged);
        };

        if reference == "null" {
            if input.allow_null_clear {
                return Ok(ResolvedMilestonePatch::Clear);
            }
            return Err(CliError::InvalidArgs(
                "milestone reference 'null' is only valid when clearing issue milestone".to_string(),
            ));
        }

        let id = self.resolve_required_id(token, &reference, input.project)?;
        Ok(ResolvedMilestonePatch::Set(id))
    }

    pub fn resolve_required_id(
        &self,
        token: &str,
        reference: &str,
        project: Option<String>,
    ) -> Result<String, CliError> {
        if reference == "null" {
            return Err(CliError::InvalidArgs(
                "milestone target cannot be null".to_string(),
            ));
        }

        let parsed_reference = parse_direct_reference(reference);
        if is_uuid_like(parsed_reference) {
            return self
                .lookup
                .get_milestone_by_id(token, parsed_reference)?
                .map(|milestone| milestone.id)
                .ok_or_else(|| CliError::NotFound(format!("milestone not found: {reference}")));
        }

        let project_id = match project {
            Some(project) if is_uuid_like(&project) => Some(project),
            Some(project) => Some(
                self.lookup
                    .resolve_project_id_by_slug(token, &project)?
                    .unwrap_or(project),
            ),
            None => None,
        };

        let matches = self
            .lookup
            .find_milestones_by_name(token, parsed_reference, project_id.as_deref())?;

        match matches.len() {
            0 => Err(CliError::NotFound(format!(
                "milestone not found: {parsed_reference}"
            ))),
            1 => Ok(matches[0].id.clone()),
            _ => {
                let projects = matches
                    .iter()
                    .map(|milestone| milestone.project.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                Err(CliError::InvalidArgs(format!(
                    "milestone name '{parsed_reference}' is ambiguous across projects: {projects}. Pass --project to scope resolution"
                )))
            }
        }
    }
}

fn parse_direct_reference(reference: &str) -> &str {
    reference.trim_end_matches('/').rsplit('/').next().unwrap_or(reference)
}

fn is_uuid_like(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 36 {
        return false;
    }

    for (idx, byte) in bytes.iter().enumerate() {
        let is_dash = matches!(idx, 8 | 13 | 18 | 23);
        if is_dash {
            if *byte != b'-' {
                return false;
            }
            continue;
        }
        if !byte.is_ascii_hexdigit() {
            return false;
        }
    }
    true
}
