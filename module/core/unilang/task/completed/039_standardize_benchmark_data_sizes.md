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

-   [x] Standard data size categories implemented: Small (10), Medium (100), Large (1000), Huge (10000)
-   [x] All benchmarks use standardized data size naming convention
-   [x] Data generation functions accept size parameters
-   [x] Benchmark naming includes size category for clarity
-   [x] Performance scaling analysis enabled across size categories
-   [x] Documentation describes what each size category represents
-   [x] Size categories appropriate for unilang use cases (commands, parsing, etc.)
-   [x] Consistent comparison enabled across different algorithm implementations

## Outcomes

**Implementation Completed:**

1. **BenchmarkDataSize Module Created** (`src/benchmark_data_sizes.rs`):
   - Standard size categories: Small (10), Medium (100), Large (1000), Huge (10000)
   - BenchmarkDataSize enum with value(), name(), description() methods
   - BenchmarkDataUtils with data generation functions
   - StandardDataGenerator trait for extensible data generation
   - Documentation generator for size category explanations

2. **All Benchmarks Updated to Use Standard Sizes**:
   - `comprehensive_framework_comparison.rs` - now has 12 standardized benchmarks (4 sizes × 3 frameworks)
   - `throughput_benchmark.rs` - uses standardized sizes with descriptive naming
   - `string_interning_benchmark.rs` - uses Huge (10,000) for statistical significance

3. **Enhanced Features**:
   - Consistent naming: unilang_small, clap_medium, pico_args_large, etc.
   - Descriptive documentation for each size category
   - Utility functions for command name and test data generation
   - JSON data generation for different payload sizes

4. **Technical Implementation**:
   - Added `benchmark_data_sizes` layer to `lib.rs`
   - All existing benchmark functions converted to use standardized sizes
   - Added large and huge command benchmarks (previously only had 10 and 100)
   - Maintained backward compatibility while implementing standardization

**Benefits Achieved**:
- Consistent performance comparison baseline across all benchmarks
- Clear scaling analysis from 10 to 10,000 commands
- Standardized naming enables easy comparison across projects
- Documentation automatically describes what each size represents

**Status:** ✅ Completed