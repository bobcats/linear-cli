use linear_cli::perf::regression::{
    evaluate_regression_against_targets, samples_from_criterion_index,
};
use linear_cli::perf::targets::parse_latency_targets_json;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut from: Option<String> = None;
    let mut targets: Option<String> = None;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--from" => from = args.next(),
            "--targets" => targets = args.next(),
            _ => {}
        }
    }

    let from = from.ok_or_else(|| {
        "Usage: perf-regression-gate --from <criterion-index-or-dir> --targets <targets-json>"
            .to_string()
    })?;
    let targets = targets.ok_or_else(|| {
        "Usage: perf-regression-gate --from <criterion-index-or-dir> --targets <targets-json>"
            .to_string()
    })?;

    let input_path = resolve_input_path(Path::new(&from))?;

    let benchmark_content = fs::read_to_string(&input_path)
        .map_err(|e| format!("Failed to read {}: {e}", input_path.display()))?;
    let target_content =
        fs::read_to_string(&targets).map_err(|e| format!("Failed to read {targets}: {e}"))?;

    let target_doc = parse_latency_targets_json(&target_content)
        .map_err(|e| format!("Failed to parse targets: {e}"))?;
    let samples = samples_from_criterion_index(&benchmark_content)
        .map_err(|e| format!("Failed to parse benchmark samples: {e}"))?;

    let evaluation = evaluate_regression_against_targets(&target_doc, &samples)
        .map_err(|e| format!("Failed to evaluate regression gate: {e}"))?;

    if evaluation.passed {
        println!("✅ Regression gate PASSED");
        println!("Checked {} latency targets.", target_doc.targets.len());
        return Ok(());
    }

    println!("❌ Regression gate FAILED");
    println!(
        "{} target(s) exceeded threshold or were missing.",
        evaluation.failures.len()
    );
    println!("\n| Benchmark | Target (ns) | Allowed (ns) | Observed (ns) | Reason |");
    println!("| --- | ---: | ---: | ---: | --- |");

    for failure in &evaluation.failures {
        let observed = failure
            .observed_ns
            .map(|n| format!("{n:.2}"))
            .unwrap_or_else(|| "MISSING".to_string());

        println!(
            "| {} | {:.2} | {:.2} | {} | {} |",
            failure.benchmark_id, failure.target_ns, failure.allowed_ns, observed, failure.reason
        );
    }

    Err("regression gate failed".to_string())
}

fn resolve_input_path(path: &Path) -> Result<PathBuf, String> {
    if path.is_file() {
        return Ok(path.to_path_buf());
    }

    if !path.is_dir() {
        return Err(format!("Path does not exist: {}", path.display()));
    }

    let candidates = [path.join("report/index.json"), path.join("index.json")];

    candidates
        .into_iter()
        .find(|candidate| candidate.is_file())
        .ok_or_else(|| {
            format!(
                "No benchmark index found under {} (expected report/index.json or index.json)",
                path.display()
            )
        })
}
