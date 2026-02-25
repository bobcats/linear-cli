use crate::error::CliError;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq)]
pub struct HotspotEntry {
    pub id: String,
    pub estimate_ns: f64,
}

#[derive(Debug, Deserialize)]
struct CriterionIndex {
    benchmarks: Vec<CriterionBenchmark>,
}

#[derive(Debug, Deserialize)]
struct CriterionBenchmark {
    id: String,
    typical: CriterionEstimate,
}

#[derive(Debug, Deserialize)]
struct CriterionEstimate {
    estimate: f64,
    unit: String,
}

/// Parse Criterion-style benchmark index JSON and return hotspots sorted by
/// descending estimated latency in nanoseconds.
pub fn rank_hotspots_from_criterion_index(input: &str) -> Result<Vec<HotspotEntry>, CliError> {
    let index: CriterionIndex = serde_json::from_str(input)
        .map_err(|e| CliError::General(format!("Failed to parse benchmark index JSON: {e}")))?;

    let mut entries: Vec<HotspotEntry> = index
        .benchmarks
        .into_iter()
        .map(|benchmark| {
            let estimate_ns = to_nanoseconds(benchmark.typical.estimate, &benchmark.typical.unit)?;
            Ok(HotspotEntry {
                id: benchmark.id,
                estimate_ns,
            })
        })
        .collect::<Result<_, CliError>>()?;

    entries.sort_by(|a, b| {
        b.estimate_ns
            .total_cmp(&a.estimate_ns)
            .then_with(|| a.id.cmp(&b.id))
    });

    Ok(entries)
}

fn to_nanoseconds(estimate: f64, unit: &str) -> Result<f64, CliError> {
    match unit {
        "ns" => Ok(estimate),
        "us" | "µs" => Ok(estimate * 1_000.0),
        "ms" => Ok(estimate * 1_000_000.0),
        "s" => Ok(estimate * 1_000_000_000.0),
        other => Err(CliError::General(format!(
            "Unsupported benchmark unit: {other}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_conversion() {
        assert_eq!(to_nanoseconds(5.0, "ns").unwrap(), 5.0);
        assert_eq!(to_nanoseconds(2.0, "us").unwrap(), 2_000.0);
        assert_eq!(to_nanoseconds(2.0, "µs").unwrap(), 2_000.0);
        assert_eq!(to_nanoseconds(1.0, "ms").unwrap(), 1_000_000.0);
        assert_eq!(to_nanoseconds(1.0, "s").unwrap(), 1_000_000_000.0);
    }
}
