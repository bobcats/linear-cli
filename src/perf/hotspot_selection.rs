use crate::error::CliError;
use crate::perf::hotspot_ranking::HotspotEntry;

/// Select top-N hotspots from a pre-ranked hotspot list.
///
/// Ordering is preserved exactly as provided in `ranked_hotspots`.
pub fn select_hotspots_for_optimization(
    ranked_hotspots: &[HotspotEntry],
    top_n: usize,
) -> Result<Vec<HotspotEntry>, CliError> {
    if top_n == 0 {
        return Err(CliError::InvalidArgs(
            "top_n must be greater than zero".to_string(),
        ));
    }

    Ok(ranked_hotspots.iter().take(top_n).cloned().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_returns_all_when_top_n_exceeds_input() {
        let ranked = vec![
            HotspotEntry {
                id: "a".to_string(),
                estimate_ns: 100.0,
            },
            HotspotEntry {
                id: "b".to_string(),
                estimate_ns: 50.0,
            },
        ];

        let selected = select_hotspots_for_optimization(&ranked, 10).unwrap();

        assert_eq!(selected, ranked);
    }
}
