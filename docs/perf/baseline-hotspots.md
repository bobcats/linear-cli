# Baseline Hotspots

Generated from v0.2.0 benchmark run (`a15e6398`, main, 2026-03-12).

Run:

```bash
python3 scripts/build-criterion-index.py  # synthesize index from per-benchmark estimates.json
cargo run --bin perf-hotspots -- --from target_latest/criterion
```

## Ranked table

| Rank | Benchmark | Estimate (ns) |
| --- | --- | ---: |
| 1 | table_hotspot_large_rows/project_list_table/1000 | 2365662.78 |
| 2 | table_hotspot_large_rows/team_list_table/1000 | 1795889.31 |
| 3 | table_hotspot_large_rows/project_list_table/500 | 1185316.73 |
| 4 | table_hotspot_wide_cells/team_list_table_wide | 1002174.80 |
| 5 | issue_list/table/500 | 980766.07 |
| 6 | table_hotspot_wide_cells/project_list_table_wide | 980354.82 |
| 7 | table_hotspot_large_rows/team_list_table/500 | 895820.97 |
| 8 | table_hotspot_large_rows/project_list_table/250 | 586936.53 |
| 9 | table_hotspot_large_rows/team_list_table/250 | 456165.52 |
| 10 | cycle_list/table/100 | 261937.58 |
| 11 | project_list/table/100 | 237333.74 |
| 12 | issue_list/table/100 | 193051.17 |
| 13 | comment_list/table/100 | 187195.11 |
| 14 | team_list/table/100 | 181646.63 |
| 15 | issue_list/json/500 | 168265.43 |
| 16 | cycle_list/table/50 | 134609.86 |
| 17 | project_list/table/50 | 121924.07 |
| 18 | issue_list/table/50 | 96910.44 |
| 19 | comment_list/table/50 | 95535.73 |
| 20 | team_list/table/50 | 91382.79 |
| 21 | issue_list/csv/500 | 54097.85 |
| 22 | project_list/json/100 | 40788.14 |
| 23 | issue_list/markdown/500 | 39372.70 |
| 24 | issue_list/json/100 | 34799.65 |
| 25 | cli_parse_issue_paths/issue_update_rich | 32453.20 |
| 26 | cli_parse_issue_paths/issue_create_rich | 32009.65 |
| 27 | cli_parse_issue_paths/issue_relation_link | 30322.10 |
| 28 | cycle_list/json/100 | 29654.50 |
| 29 | cli_parse_issue_paths/issue_create_invalid_priority | 29522.64 |
| 30 | cli_parse_cycle_paths/cycle_list | 29215.42 |
| 31 | cli_parse_project_paths/project_view | 29037.61 |
| 32 | cycle_list/table/10 | 29012.51 |
| 33 | cli_parse_cycle_paths/cycle_current | 28881.60 |
| 34 | cli_parse_project_paths/project_list | 28880.93 |
| 35 | cli_parse_team_paths/team_view | 28842.45 |
| 36 | cli_parse_cycle_paths/cycle_view | 28767.74 |
| 37 | cli_parse_team_paths/team_list | 28330.84 |
| 38 | project_list/table/10 | 27405.47 |
| 39 | cycle_list/csv/100 | 23696.32 |
| 40 | issue_list/table/10 | 21952.38 |
| 41 | comment_list/table/10 | 21676.02 |
| 42 | team_list/table/10 | 20632.61 |
| 43 | project_handlers_format_compare/view_table | 20139.19 |
| 44 | project_list/json/50 | 20078.51 |
| 45 | project_list/csv/100 | 19488.00 |
| 46 | project_table | 18727.70 |
| 47 | issue_list/json/50 | 17999.37 |
| 48 | issue_hierarchy/table/25 | 17987.49 |
| 49 | cycle_list/markdown/100 | 17927.71 |
| 50 | write_mutation_build_and_serialize/issue_relation_create | 16754.48 |
| 51 | issue_hierarchy/table/5 | 16401.16 |
| 52 | write_mutation_build/issue_relation_create | 16083.19 |
| 53 | comment_list/json/100 | 15400.86 |
| 54 | project_list/markdown/100 | 15304.88 |
| 55 | cycle_list/json/50 | 14956.41 |
| 56 | cycle_table | 14812.98 |
| 57 | issue_with_comments/table/50 | 14689.93 |
| 58 | issue_with_comments/table/10 | 14680.30 |
| 59 | issue_table | 14030.98 |
| 60 | issue_hierarchy/table/0 | 13963.41 |
| 61 | issue_with_comments/table/0 | 13602.76 |
| 62 | issue_handlers_format_compare/create_table | 13225.99 |
| 63 | write_mutation_build_and_serialize/issue_update | 12900.11 |
| 64 | team_list/json/100 | 12876.96 |
| 65 | write_mutation_build_and_serialize/issue_create | 12607.85 |
| 66 | cycle_handlers_format_compare/view_table | 12364.27 |
| 67 | write_mutation_build_and_serialize/issue_archive | 12360.81 |
| 68 | write_mutation_build_and_serialize/issue_unarchive | 12302.36 |
| 69 | write_mutation_build/issue_update | 11832.87 |
| 70 | write_mutation_build/issue_create | 11811.21 |
| 71 | write_mutation_build/issue_archive | 11779.67 |
| 72 | write_mutation_build/issue_unarchive | 11504.18 |
| 73 | cycle_list/csv/50 | 11450.99 |
| 74 | issue_list/csv/100 | 10991.03 |
| 75 | comment_list/csv/100 | 10928.57 |
