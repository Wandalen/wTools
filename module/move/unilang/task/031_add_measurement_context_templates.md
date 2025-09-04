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

-   [ ] All benchmark functions include "What is measured:" context
-   [ ] All benchmark documentation includes "How to measure:" commands
-   [ ] Context templates follow exact format from usage.md
-   [ ] Function signatures, data types, and parameters clearly specified
-   [ ] Benchmark execution commands documented with exact feature flags
-   [ ] Algorithm comparison context includes input data specifications
-   [ ] Performance tables prefixed with visual context before data
-   [ ] Environment specifications included where relevant