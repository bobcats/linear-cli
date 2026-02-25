use crate::error::CliError;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LatencyTargetsDocument {
    pub generated_from: String,
    pub generated_at: String,
    pub targets: Vec<LatencyTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LatencyTarget {
    pub benchmark_id: String,
    pub baseline_ns: f64,
    pub target_ns: f64,
    pub tolerance_percent: f64,
}

pub fn parse_latency_targets_json(input: &str) -> Result<LatencyTargetsDocument, CliError> {
    let doc: LatencyTargetsDocument = serde_json::from_str(input)
        .map_err(|e| CliError::General(format!("Failed to parse latency targets JSON: {e}")))?;

    validate_latency_targets(&doc)?;
    Ok(doc)
}

pub fn validate_latency_targets(doc: &LatencyTargetsDocument) -> Result<(), CliError> {
    if doc.generated_from.trim().is_empty() {
        return Err(CliError::InvalidArgs(
            "generated_from is required in latency targets document".to_string(),
        ));
    }

    if doc.generated_at.trim().is_empty() {
        return Err(CliError::InvalidArgs(
            "generated_at is required in latency targets document".to_string(),
        ));
    }

    if doc.targets.is_empty() {
        return Err(CliError::InvalidArgs(
            "latency targets document must contain at least one target".to_string(),
        ));
    }

    let mut seen = HashSet::new();

    for target in &doc.targets {
        if target.benchmark_id.trim().is_empty() {
            return Err(CliError::InvalidArgs(
                "benchmark_id is required for every latency target".to_string(),
            ));
        }

        if !seen.insert(target.benchmark_id.clone()) {
            return Err(CliError::InvalidArgs(format!(
                "duplicate benchmark_id in latency targets: {}",
                target.benchmark_id
            )));
        }

        if !target.baseline_ns.is_finite() || !target.target_ns.is_finite() {
            return Err(CliError::InvalidArgs(
                "baseline_ns and target_ns must be finite numbers".to_string(),
            ));
        }

        if target.baseline_ns <= 0.0 || target.target_ns <= 0.0 {
            return Err(CliError::InvalidArgs(
                "baseline_ns and target_ns must be positive".to_string(),
            ));
        }

        if target.target_ns > target.baseline_ns {
            return Err(CliError::InvalidArgs(format!(
                "target_ns must be less than or equal to baseline_ns for {}",
                target.benchmark_id
            )));
        }

        if !target.tolerance_percent.is_finite() || target.tolerance_percent < 0.0 {
            return Err(CliError::InvalidArgs(
                "tolerance_percent must be a finite non-negative number".to_string(),
            ));
        }
    }

    Ok(())
}
