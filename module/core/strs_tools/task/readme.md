<!-- task_system_metadata
type: local
root: ../../../../task/readme.md
crate: strs_tools
last_sync: 2026-06-23
-->

# Task Management

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | State | Executor | Dir | Task | Purpose |
|-------|-----|--------------|-------|----------|--------|----------|-------|----------|-----|------|---------|
| 1 | [012](completed/012_create_test_surface_and_fill_coverage_gaps.md) | 0 | 8 | 6 | 9 | 0 | ✅ (Completed) | any | module/core/strs_tools | Create test surface and fill coverage gaps | Create tests/docs/ specs and fill 3 remaining coverage gaps (SIMD, indentation expansion, number parsing expansion) |
| 2 | [011](completed/011_add_visual_width_display_columns.md) | 0 | 6 | 9 | 9 | 0 | ✅ (Completed) | any | module/core/strs_tools | Add visual_width display columns | Add `visual_width()` display-column measurement API + migrate unicode-width to workspace ^0.2 |
| 3 | [010](completed/010_extract_formatting_utilities_from_wplan.md) | 0 | 9 | 6 | 8 | 0 | ✅ (Completed) | any | . | Extract Formatting Utilities from wplan | Extract ANSI handling utilities from wplan |
| 4 | [008](completed/008_parser_integration.md) | 0 | 10 | 5 | 5 | 0 | ✅ (Completed) | any | . | Parser Integration | Parser integration optimization for parsing pipelines |
| 5 | [007](completed/007_specialized_algorithms.md) | 0 | 8 | 5 | 5 | 0 | ✅ (Completed) | any | . | Specialized Algorithms | SingleCharSplitIterator, Boyer-Moore, smart_split |
| 6 | [003](completed/003_compile_time_pattern_optimization.md) | 0 | 10 | 5 | 5 | 0 | ✅ (Completed) | any | . | Compile Time Pattern Optimization | Compile-time pattern optimization with procedural macros |
| 7 | [002](completed/002_zero_copy_optimization.md) | 0 | 10 | 5 | 5 | 0 | ✅ (Completed) | any | . | Zero Copy Optimization | Zero-copy string operations with copy-on-write semantics |
| 8 | [001](completed/001_simd_optimization.md) | 0 | 10 | 5 | 5 | 0 | ✅ (Completed) | any | . | SIMD Optimization | SIMD-optimized string operations with automatic fallback |

## Bug Index

| ID | Title | Related Task | Status |
|----|-------|--------------|--------|
| BUG-001 | [Escape Sequence Handling Bug](bug/closed/001_escaped_quotes_in_quoted_strings.md) | 008 | ✅ Closed |
