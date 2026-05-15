# Baseline Hotspots

Generated from post-milestone-performance-coverage benchmarks (main `a7f26bf1`, 2026-05-14).

Coverage added in this pass:
- Milestone formatter benchmarks.
- Milestone command parse/handler benchmarks.
- Milestone mutation build and build+serialize benchmarks.

## Ranked table

| Rank | Benchmark | Estimate (ns) |
| --- | --- | ---: |
| 1 | table_hotspot_large_rows/project_list_table/1000 | 2126542.33 |
| 2 | table_hotspot_large_rows/team_list_table/1000 | 1459092.11 |
| 3 | table_hotspot_large_rows/project_list_table/500 | 1021537.11 |
| 4 | milestone_list/table/500 | 930789.72 |
| 5 | table_hotspot_wide_cells/team_list_table_wide | 814138.68 |
| 6 | table_hotspot_wide_cells/project_list_table_wide | 802732.43 |
| 7 | issue_list/table/500 | 766054.53 |
| 8 | table_hotspot_large_rows/team_list_table/500 | 695696.29 |
| 9 | table_hotspot_large_rows/project_list_table/250 | 487574.05 |
| 10 | table_hotspot_large_rows/team_list_table/250 | 357047.69 |
| 11 | cycle_list/table/100 | 210038.66 |
| 12 | project_list/table/100 | 196698.73 |
| 13 | milestone_list/table/100 | 178877.73 |
| 14 | issue_list/json/500 | 170340.13 |
| 15 | issue_list/table/100 | 154140.42 |
| 16 | comment_list/table/100 | 145051.11 |
| 17 | team_list/table/100 | 140782.39 |
| 18 | table_overhead/comfy_utf8_full_100_teams | 118180.09 |
| 19 | milestone_list/json/500 | 114533.28 |
| 20 | cycle_list/table/50 | 112804.08 |
| 21 | project_list/table/50 | 101300.05 |
| 22 | table_overhead/comfy_ascii_markdown_100_teams | 92786.48 |
| 23 | milestone_list/table/50 | 89592.71 |
| 24 | issue_list/table/50 | 78205.12 |
| 25 | milestone_list/csv/500 | 76637.65 |
| 26 | team_list/table/50 | 73316.63 |
| 27 | comment_list/table/50 | 72558.94 |
| 28 | milestone_list/markdown/500 | 64920.63 |
| 29 | issue_list/csv/500 | 50783.81 |
| 30 | project_list/json/100 | 39244.10 |
| 31 | issue_list/markdown/500 | 38609.01 |
| 32 | cli_parse_milestone_paths/milestone_list | 37150.98 |
| 33 | cli_parse_milestone_paths/milestone_view | 36792.83 |
| 34 | cli_parse_milestone_paths/milestone_create | 36139.45 |
| 35 | cli_parse_milestone_paths/milestone_delete | 35833.53 |
| 36 | cli_parse_milestone_paths/milestone_update | 35806.38 |
| 37 | issue_list/json/100 | 35793.23 |
| 38 | cli_parse_issue_paths/issue_update_rich | 32942.95 |
| 39 | cli_parse_issue_paths/issue_create_rich | 32238.97 |
| 40 | cli_parse_issue_paths/issue_relation_link | 31260.46 |
| 41 | cli_parse_team_paths/team_view | 30285.86 |
| 42 | cycle_list/json/100 | 29866.90 |
| 43 | cli_parse_cycle_paths/cycle_current | 29604.99 |
| 44 | cli_parse_cycle_paths/cycle_view | 29580.77 |
| 45 | cli_parse_issue_paths/issue_create_invalid_priority | 29468.18 |
| 46 | cli_parse_project_paths/project_view | 29402.60 |
| 47 | cli_parse_cycle_paths/cycle_list | 29292.27 |
| 48 | cli_parse_team_paths/team_list | 29280.55 |
| 49 | cli_parse_project_paths/project_list | 29201.35 |
| 50 | milestone_list/json/100 | 23861.18 |
| 51 | cycle_list/table/10 | 23689.21 |
| 52 | cycle_list/csv/100 | 22957.62 |
| 53 | project_list/table/10 | 22379.79 |
| 54 | milestone_handlers_json/list | 21724.45 |
| 55 | milestone_list/table/10 | 20456.02 |
| 56 | project_list/json/50 | 19438.97 |
| 57 | issue_list/json/50 | 18808.09 |
| 58 | cycle_list/markdown/100 | 18483.00 |
| 59 | project_list/csv/100 | 18122.38 |
| 60 | issue_list/table/10 | 17907.35 |
| 61 | write_mutation_build_and_serialize/issue_relation_create | 17231.55 |
| 62 | team_list/table/10 | 17167.85 |
| 63 | comment_list/table/10 | 16806.40 |
| 64 | write_mutation_build/issue_relation_create | 15956.24 |
| 65 | project_list/markdown/100 | 15565.35 |
| 66 | comment_list/json/100 | 15073.53 |
| 67 | cycle_list/json/50 | 14883.16 |
| 68 | milestone_list/csv/100 | 14603.15 |
| 69 | project_handlers_format_compare/view_table | 14037.23 |
| 70 | project_table | 13202.33 |
| 71 | issue_hierarchy/table/25 | 13147.78 |
| 72 | team_list/json/100 | 12988.43 |
| 73 | milestone_list/markdown/100 | 12942.72 |
| 74 | write_mutation_build_and_serialize/issue_update | 12525.57 |
| 75 | milestone_list/json/50 | 12458.21 |
| 76 | write_mutation_build_and_serialize/issue_archive | 12398.19 |
| 77 | write_mutation_build_and_serialize/issue_create | 12320.33 |
| 78 | write_mutation_build_and_serialize/issue_unarchive | 12238.22 |
| 79 | issue_hierarchy/table/5 | 12028.26 |
| 80 | write_mutation_build/issue_update | 11833.44 |
