use linear_cli::perf::evidence::{
    OptimizationEvidence, OptimizationEvidenceEntry, validate_evidence_document,
};

#[test]
fn test_validate_evidence_document_requires_before_and_after_metrics() {
    let evidence = OptimizationEvidence {
        feature_id: "bd-yo7".to_string(),
        entries: vec![OptimizationEvidenceEntry {
            benchmark_id: "issue_handlers_json/create".to_string(),
            before_ns: 850.0,
            after_ns: 790.0,
            notes: "Reduced allocations in command path".to_string(),
        }],
    };

    let result = validate_evidence_document(&evidence);

    assert!(result.is_ok());
}

#[test]
fn test_validate_evidence_document_rejects_missing_benchmark_id() {
    let evidence = OptimizationEvidence {
        feature_id: "bd-yo7".to_string(),
        entries: vec![OptimizationEvidenceEntry {
            benchmark_id: String::new(),
            before_ns: 850.0,
            after_ns: 790.0,
            notes: "missing benchmark id".to_string(),
        }],
    };

    let result = validate_evidence_document(&evidence);

    assert!(result.is_err());
}
