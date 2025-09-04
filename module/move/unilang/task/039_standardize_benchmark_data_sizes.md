# Standardize Benchmark Data Sizes

## Description

**MODERATE PRIORITY VIOLATION**: Usage.md recommends standard data sizes for consistent comparison. Current benchmarks use inconsistent data sizing.

**Required Data Size Pattern** (from usage.md):
```rust
// Recommended data size pattern
let data_sizes = vec![
    ("Small", 10),      // Quick operations, edge cases
    ("Medium", 100),    // Typical usage scenarios  
    ("Large", 1000),    // Stress testing, scaling analysis
    ("Huge", 10000),    // Performance bottleneck detection
];

for (size_name, size) in data_sizes {
    let data = generate_test_data(size);
    suite.benchmark(&format!("algorithm_{}", size_name.to_lowercase()), 
                   || algorithm(&data));
}
```

**Why This Matters**: Consistent sizing makes it easy to compare performance across different implementations and projects.

**Current State**: Inconsistent data sizing across benchmarks without standardized categories.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`
-   Must follow benchkit usage.md "Use Standard Data Sizes" section
-   Related to Task 040 (realistic test data) and Task 041 (comparative structure)
-   Must provide consistent performance comparison baseline

## Acceptance Criteria

-   [ ] Standard data size categories implemented: Small (10), Medium (100), Large (1000), Huge (10000)
-   [ ] All benchmarks use standardized data size naming convention
-   [ ] Data generation functions accept size parameters
-   [ ] Benchmark naming includes size category for clarity
-   [ ] Performance scaling analysis enabled across size categories
-   [ ] Documentation describes what each size category represents
-   [ ] Size categories appropriate for unilang use cases (commands, parsing, etc.)
-   [ ] Consistent comparison enabled across different algorithm implementations