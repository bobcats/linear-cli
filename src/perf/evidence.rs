use crate::error::CliError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OptimizationEvidence {
    pub feature_id: String,
    pub entries: Vec<OptimizationEvidenceEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OptimizationEvidenceEntry {
    pub benchmark_id: String,
    pub before_ns: f64,
    pub after_ns: f64,
    pub notes: String,
}

pub fn validate_evidence_document(evidence: &OptimizationEvidence) -> Result<(), CliError> {
    if evidence.feature_id.trim().is_empty() {
        return Err(CliError::InvalidArgs(
            "feature_id is required in optimization evidence".to_string(),
        ));
    }

    if evidence.entries.is_empty() {
        return Err(CliError::InvalidArgs(
            "optimization evidence must include at least one entry".to_string(),
        ));
    }

    for entry in &evidence.entries {
        if entry.benchmark_id.trim().is_empty() {
            return Err(CliError::InvalidArgs(
                "benchmark_id is required for every optimization evidence entry".to_string(),
            ));
        }

        if !entry.before_ns.is_finite() || !entry.after_ns.is_finite() {
            return Err(CliError::InvalidArgs(
                "before_ns and after_ns must be finite numbers".to_string(),
            ));
        }

        if entry.before_ns <= 0.0 || entry.after_ns <= 0.0 {
            return Err(CliError::InvalidArgs(
                "before_ns and after_ns must be positive".to_string(),
            ));
        }
    }

    Ok(())
}
