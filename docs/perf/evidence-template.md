# Optimization Evidence Template

Feature: `<feature-id>`

| Benchmark ID | Before (ns) | After (ns) | Delta (%) | Notes |
| --- | ---: | ---: | ---: | --- |
| issue_handlers_json/create | 850.00 | 790.00 | -7.06 | Reduced allocations in command path |

## Rules
- Include at least one entry.
- Benchmark ID is required for each entry.
- Before/after metrics must be positive finite values.
- Notes should summarize what changed and why.
