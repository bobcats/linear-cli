use linear_cli::perf::hotspot_ranking::rank_hotspots_from_criterion_index;
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

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--from" {
            from = args.next();
            continue;
        }
    }

    let from = from.ok_or_else(|| "Usage: perf-hotspots --from <file-or-directory>".to_string())?;
    let input_path = resolve_input_path(Path::new(&from))?;

    let content = fs::read_to_string(&input_path)
        .map_err(|e| format!("Failed to read {}: {e}", input_path.display()))?;

    let ranked = rank_hotspots_from_criterion_index(&content)
        .map_err(|e| format!("Failed to rank hotspots: {e}"))?;

    println!("# Baseline Hotspots\n");
    println!("Source: {}\n", input_path.display());
    println!("| Rank | Benchmark | Estimate (ns) |");
    println!("| --- | --- | ---: |");

    for (index, entry) in ranked.iter().enumerate() {
        println!(
            "| {} | {} | {:.2} |",
            index + 1,
            entry.id,
            entry.estimate_ns
        );
    }

    println!("\nTip: run regression gate against targets:");
    println!(
        "perf-regression-gate --from {} --targets docs/perf/latency-targets.json",
        input_path.display()
    );

    Ok(())
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
