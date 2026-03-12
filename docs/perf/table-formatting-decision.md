# Table Formatting Optimization Decision

**Date:** 2026-03-12

## Measurements (100 teams, --quick)

| Approach | Time | vs UTF8_FULL |
|---|---:|---:|
| comfy-table `UTF8_FULL` | 120 µs | baseline |
| comfy-table `ASCII_MARKDOWN` | 90 µs | **−25%** |
| Raw string (no library) | 0.8 µs | **−150×** |

Overhead ratio comfy-table vs raw: **150×**.

## Decision: Option B — `ASCII_MARKDOWN` preset swap

Rationale:
- 25% win is real and free (single preset constant change per call site)
- `ASCII_MARKDOWN` outputs pipe-separated markdown-compatible tables — readable in terminal and parseable by humans; still aligned via padding
- Option C (custom renderer) gives ~150× but requires a full UTF-8 column-width implementation and a non-trivial amount of new code. At 2.4ms for 1000 rows and network RTTs of 200ms+, the user will not notice
- If list sizes grow significantly or a non-interactive use case emerges that needs tighter formatting budgets, revisit Option C

## Visual difference

`UTF8_FULL`:
```
╭──────────┬───────────────╮
│ Key      │ Name          │
╞══════════╪═══════════════╡
│ ENG      │ Engineering   │
╰──────────┴───────────────╯
```

`ASCII_MARKDOWN`:
```
| Key | Name        |
| --- | ----------- |
| ENG | Engineering |
```

The ASCII_MARKDOWN output is valid GitHub Flavored Markdown and more copy-paste friendly. Downside: loses UTF-8 box drawing aesthetics in terminal use. Accepted.
