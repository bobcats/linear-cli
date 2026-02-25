use crate::error::CliError;
use crate::perf::hotspot_ranking::rank_hotspots_from_criterion_index;
use crate::perf::targets::{LatencyTargetsDocument, validate_latency_targets};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenchmarkSample {
    pub benchmark_id: String,
    pub estimate_ns: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegressionFailure {
    pub benchmark_id: String,
    pub target_ns: f64,
    pub tolerance_percent: f64,
    pub allowed_ns: f64,
    pub observed_ns: Option<f64>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegressionEvaluation {
    pub passed: bool,
    pub failures: Vec<RegressionFailure>,
}

pub fn samples_from_criterion_index(input: &str) -> Result<Vec<BenchmarkSample>, CliError> {
    let ranked = rank_hotspots_from_criterion_index(input)?;
    Ok(ranked
        .into_iter()
        .map(|entry| BenchmarkSample {
            benchmark_id: entry.id,
            estimate_ns: entry.estimate_ns,
        })
        .collect())
}

pub fn evaluate_regression_against_targets(
    targets: &LatencyTargetsDocument,
    samples: &[BenchmarkSample],
) -> Result<RegressionEvaluation, CliError> {
    validate_latency_targets(targets)?;

    let sample_by_id: HashMap<&str, f64> = samples
        .iter()
        .map(|sample| (sample.benchmark_id.as_str(), sample.estimate_ns))
        .collect();

    let mut failures = Vec::new();

    for target in &targets.targets {
        let allowed_ns = target.target_ns * (1.0 + (target.tolerance_percent / 100.0));

        match sample_by_id.get(target.benchmark_id.as_str()).copied() {
            None => failures.push(RegressionFailure {
                benchmark_id: target.benchmark_id.clone(),
                target_ns: target.target_ns,
                tolerance_percent: target.tolerance_percent,
                allowed_ns,
                observed_ns: None,
                reason: "missing benchmark sample".to_string(),
            }),
            Some(observed_ns) if observed_ns > allowed_ns => failures.push(RegressionFailure {
                benchmark_id: target.benchmark_id.clone(),
                target_ns: target.target_ns,
                tolerance_percent: target.tolerance_percent,
                allowed_ns,
                observed_ns: Some(observed_ns),
                reason: "observed latency exceeds allowed threshold".to_string(),
            }),
            Some(_) => {}
        }
    }

    Ok(RegressionEvaluation {
        passed: failures.is_empty(),
        failures,
    })
}
