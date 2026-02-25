# Performance Tooling

This directory contains artifacts and tooling inputs for non-auth latency optimization.

## Files

- `baseline-hotspots.md` — benchmark ranking snapshot source
- `hotspot-priority.json` — selected optimization ordering
- `latency-targets.json` — per-benchmark latency targets + tolerance
- `evidence-template.md` — optimization evidence record template

## Commands

Generate hotspot ranking table from Criterion output:

```bash
cargo run --bin perf-hotspots -- --from target/criterion
```

Evaluate current benchmark results against target thresholds:

```bash
cargo run --bin perf-regression-gate -- \
  --from target/criterion \
  --targets docs/perf/latency-targets.json
```

`perf-regression-gate` exits non-zero when any target is missing or exceeds the tolerated threshold.
