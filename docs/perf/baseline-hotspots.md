# Baseline Hotspots

Generated from post-perf-pass v0.2.0 benchmarks (main `456cfeb8`, 2026-03-12).

Optimizations applied vs initial v0.2.0 baseline:
- Table preset: UTF8_FULL → ASCII_MARKDOWN (~18-23% table speedup)
- Single-issue CSV: eliminated eager allocations for None fields (~6-12%)

## Ranked table

| Rank | Benchmark | Estimate (ns) |
| --- | --- | ---: |
| 1 | table_hotspot_large_rows/project_list_table/1000 | 2126542.33 |
| 2 | table_hotspot_large_rows/team_list_table/1000 | 1459092.11 |
| 3 | table_hotspot_large_rows/project_list_table/500 | 1021537.11 |
| 4 | table_hotspot_wide_cells/team_list_table_wide | 814138.68 |
| 5 | table_hotspot_wide_cells/project_list_table_wide | 802732.43 |
| 6 | issue_list/table/500 | 766054.53 |
| 7 | table_hotspot_large_rows/team_list_table/500 | 695696.29 |
| 8 | table_hotspot_large_rows/project_list_table/250 | 487574.05 |
| 9 | table_hotspot_large_rows/team_list_table/250 | 357047.69 |
| 10 | cycle_list/table/100 | 210038.66 |
| 11 | project_list/table/100 | 196698.73 |
| 12 | issue_list/json/500 | 170340.13 |
| 13 | issue_list/table/100 | 154140.42 |
| 14 | comment_list/table/100 | 145051.11 |
| 15 | team_list/table/100 | 140782.39 |
| 16 | cycle_list/table/50 | 112804.08 |
| 17 | project_list/table/50 | 101300.05 |
| 18 | issue_list/table/50 | 78205.12 |
| 19 | team_list/table/50 | 73316.63 |
| 20 | comment_list/table/50 | 72558.94 |
| 21 | issue_list/csv/500 | 50783.81 |
| 22 | project_list/json/100 | 39244.10 |
| 23 | issue_list/markdown/500 | 38609.01 |
| 24 | issue_list/json/100 | 35793.23 |
| 25 | cli_parse_issue_paths/issue_update_rich | 32942.95 |
| 26 | cli_parse_issue_paths/issue_create_rich | 32238.97 |
| 27 | cli_parse_issue_paths/issue_relation_link | 31260.46 |
| 28 | cycle_list/json/100 | 29866.90 |
| 29 | cli_parse_issue_paths/issue_create_invalid_priority | 29468.18 |
| 30 | write_mutation_build_and_serialize/issue_relation_create | 17231.55 |
| 31 | write_mutation_build/issue_relation_create | 15956.24 |
| 32 | write_mutation_build_and_serialize/issue_update | 12525.57 |
| 33 | write_mutation_build/issue_create | 11689.37 |
