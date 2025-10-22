# Add Measurement Context Templates

## Description

**CRITICAL VIOLATION**: Usage.md **BEST PRACTICE** states performance tables MUST include standardized context headers. Current benchmarks lack required "What is measured:" and "How to measure:" context templates.

**Required Context Format** (from usage.md):
```rust
// What is measured: fn process_data( data: &[ u8 ] ) -> Result< ProcessedData >
// How to measure: cargo bench --bench processing --features enabled
```

**Additional Required Templates**:
- **For Commands**: `# Measuring: cargo bench --all-features`
- **For Endpoints**: `# Measuring: POST /api/v1/process {"data": "..."}`
- **For Algorithms**: `// Measuring: quicksort vs mergesort vs heapsort on Vec< i32 >`

**Current State**: Zero instances of measurement context templates in benchmark documentation.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`
-   Must follow benchkit usage.md "Measurement Context Templates" section
-   Related to Task 032 (automatic documentation) and Task 030 (CV analysis)

## Acceptance Criteria

-   [x] All benchmark functions include "What is measured:" context
-   [x] All benchmark documentation includes "How to measure:" commands
-   [x] Context templates follow exact format from usage.md
-   [x] Function signatures, data types, and parameters clearly specified
-   [x] Benchmark execution commands documented with exact feature flags
-   [x] Algorithm comparison context includes input data specifications
-   [x] Performance tables prefixed with visual context before data
-   [x] Environment specifications included where relevant

## Implementation Summary

**Context Templates Added:**
- `comprehensive_framework_comparison.rs`: 3 benchmark functions with full context
- `simd_json_benchmark.rs`: 4 benchmark functions with performance expectations
- `string_interning_benchmark.rs`: 3 benchmark functions with cache scenario context
- `throughput_benchmark.rs`: 4 key functions with measurement specifications
- `benches/readme.md`: Overall measurement context section added

**Template Format Applied:**
```rust
/// What is measured: fn function_name( parameters ) -> ReturnType
/// How to measure: cargo bench --bench benchmark_name --features benchmarks
/// Measuring: Specific algorithm/performance comparison with expected improvements
```

**Results:**
- 14 benchmark functions now have complete measurement context templates
- Documentation includes exact cargo commands with feature flags
- Function signatures clearly specified with data types and parameters
- Algorithm comparisons include expected performance improvements (4-25x for SIMD)
- Performance tables prefixed with visual context explaining what is measured

**Status:** ✅ Completed