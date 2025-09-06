# Implement Comparative Benchmark Structure

## Description

**MODERATE PRIORITY VIOLATION**: Usage.md requires side-by-side algorithm comparisons. Some frameworks comparison exists but missing systematic comparative structure.

**Required Comparative Pattern** (from usage.md):
```rust
// Better: Structured comparison
let algorithms = vec![
  ( "quicksort", quicksort as fn( &[ i32 ] ) -> Vec< i32 > ),
  ( "mergesort", mergesort ),
  ( "heapsort", heapsort ),
];

for ( name, algorithm ) in algorithms
{
  suite.benchmark( &format!( "{}_large_dataset", name ), 
                 || algorithm( &large_dataset ) );
}
```

**Required Output Format**:
```rust
// What is measured: Sorting algorithms on Vec< i32 > with 10,000 elements
// How to measure: cargo bench --bench sorting_algorithms --features enabled

| Algorithm | Average Time | Std Dev | Relative Performance |
|-----------|--------------|---------|---------------------|
| quicksort_large_dataset | 2.1ms | ±0.15ms | 1.00x (baseline) |
| mergesort_large_dataset | 2.8ms | ±0.12ms | 1.33x slower |
| heapsort_large_dataset | 3.2ms | ±0.18ms | 1.52x slower |
```

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`
-   Must follow benchkit usage.md "Write Comparative Benchmarks" section
-   Related to Task 040 (realistic data) and Task 042 (documentation context)
-   Must make performance differences immediately clear

## Acceptance Criteria

-   [ ] Side-by-side algorithm comparisons implemented
-   [ ] Structured comparison pattern for similar algorithms
-   [ ] Baseline establishment (1.00x reference point)
-   [ ] Relative performance calculations and reporting
-   [ ] Clear performance comparison tables generated
-   [ ] Multiple algorithms tested with same input data
-   [ ] Statistical significance comparison between algorithms
-   [ ] Winner identification in comparison tables
-   [ ] Comprehensive framework comparisons (unilang vs alternatives)