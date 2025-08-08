# benchkit Development Recommendations

**Source**: Lessons learned during unilang and strs_tools benchmarking development  
**Date**: 2025-08-08  
**Context**: Real-world performance analysis challenges and solutions

---

## Table of Contents

1. [Core Philosophy Recommendations](#core-philosophy-recommendations)
2. [Technical Architecture Requirements](#technical-architecture-requirements)
3. [User Experience Guidelines](#user-experience-guidelines)
4. [Performance Analysis Best Practices](#performance-analysis-best-practices)
5. [Documentation Integration Requirements](#documentation-integration-requirements)
6. [Data Generation Standards](#data-generation-standards)
7. [Statistical Analysis Requirements](#statistical-analysis-requirements)
8. [Feature Organization Principles](#feature-organization-principles)

---

## Core Philosophy Recommendations

### REQ-PHIL-001: Toolkit over Framework Philosophy
**Source**: "I don't want to mess with all that problem I had" - User feedback on criterion complexity

**Requirements:**
- **MUST** provide building blocks, not rigid workflows
- **MUST** allow integration into existing test files without structural changes
- **MUST** avoid forcing specific directory organization (like criterion's `benches/` requirement)
- **SHOULD** work in any context: tests, examples, binaries, documentation generation

**Anti-patterns to avoid:**
- Requiring separate benchmark directory structure
- Forcing specific CLI interfaces or runner programs
- Imposing opinionated report formats that can't be customized
- Making assumptions about user's project organization

### REQ-PHIL-002: Non-restrictive User Interface
**Source**: "toolkit non overly restricting its user and easy to use"

**Requirements:**
- **MUST** provide multiple ways to achieve the same goal
- **MUST** allow partial adoption (use only needed components)
- **SHOULD** provide sensible defaults but allow full customization
- **SHOULD** compose well with existing benchmarking tools (criterion compatibility layer)

### REQ-PHIL-003: Focus on Big Picture Optimization
**Source**: "encourage its user to expose just few critical parameters of optimization and hid the rest deeper, focusing end user on big picture"

**Requirements:**
- **MUST** surface 2-3 key performance indicators prominently
- **MUST** hide detailed statistics behind optional analysis functions
- **SHOULD** provide clear improvement/regression percentages
- **SHOULD** offer actionable optimization recommendations
- **MUST** avoid overwhelming users with statistical details by default

---

## Technical Architecture Requirements

### REQ-ARCH-001: Minimal Overhead Design
**Source**: Benchmarking accuracy concerns and timing precision requirements

**Requirements:**
- **MUST** have <1% measurement overhead for operations >1ms
- **MUST** use efficient timing mechanisms (avoid allocations in hot paths)
- **MUST** provide zero-copy where possible during measurement
- **SHOULD** allow custom metric collection without performance penalty

### REQ-ARCH-002: Feature Flag Organization
**Source**: "put every extra feature under cargo feature" - Explicit requirement

**Requirements:**
- **MUST** make all non-core functionality optional via feature flags
- **MUST** have granular control over dependencies (avoid pulling in unnecessary crates)
- **MUST** provide sensible feature combinations (full, default, minimal)
- **SHOULD** document feature flag impact on binary size and dependencies

**Specific feature requirements:**
```toml
[features]
default = ["enabled", "markdown_reports", "data_generators"]  # Essential features only
full = ["default", "html_reports", "statistical_analysis"]    # Everything
minimal = ["enabled"]                                          # Core timing only
```

### REQ-ARCH-003: Dependency Management
**Source**: Issues with heavy dependencies in benchmarking tools

**Requirements:**
- **MUST** keep core functionality dependency-free where possible
- **MUST** use workspace dependencies consistently
- **SHOULD** prefer lightweight alternatives for optional features
- **MUST** avoid dependency version conflicts with criterion (for compatibility)

---

## User Experience Guidelines

### REQ-UX-001: Simple Integration Pattern
**Source**: Frustration with complex setup requirements

**Requirements:**
- **MUST** work with <10 lines of code for basic usage
- **MUST** provide working examples in multiple contexts:
  - Unit tests with `#[test]` functions
  - Integration tests 
  - Standalone binaries
  - Documentation generation scripts

**Example integration requirement:**
```rust
// This must work in any test file
use benchkit::prelude::*;

#[test]  
fn my_performance_test() {
    let result = bench_function("my_operation", || my_function());
    assert!(result.mean_time() < Duration::from_millis(100));
}
```

### REQ-UX-002: Incremental Adoption Support
**Source**: Need to work alongside existing tools

**Requirements:**
- **MUST** provide criterion compatibility layer
- **SHOULD** allow migration from criterion without rewriting existing benchmarks
- **SHOULD** work alongside other benchmarking tools without conflicts
- **MUST** not interfere with existing project benchmarking setup

### REQ-UX-003: Clear Error Messages and Debugging
**Source**: Time spent debugging benchmarking issues

**Requirements:**
- **MUST** provide clear error messages for common mistakes
- **SHOULD** suggest fixes for configuration problems
- **SHOULD** validate benchmark setup and warn about potential issues
- **MUST** provide debugging tools for measurement accuracy verification

---

## Performance Analysis Best Practices

### REQ-PERF-001: Standard Data Size Patterns
**Source**: "Common patterns: small (10), medium (100), large (1000), huge (10000)" - From unilang/strs_tools analysis

**Requirements:**
- **MUST** provide `DataSize` enum with standardized sizes
- **MUST** use these specific values by default:
  - Small: 10 items
  - Medium: 100 items  
  - Large: 1000 items
  - Huge: 10000 items
- **SHOULD** allow custom sizes but encourage standard patterns
- **MUST** provide generators for these patterns

### REQ-PERF-002: Comparative Analysis Requirements
**Source**: Before/after comparison needs from optimization work

**Requirements:**
- **MUST** provide easy before/after comparison tools
- **MUST** calculate improvement/regression percentages
- **MUST** detect significant changes (>5% threshold by default)
- **SHOULD** provide multiple algorithm comparison (A/B/C testing)
- **MUST** highlight best performing variant clearly

### REQ-PERF-003: Real-World Measurement Patterns
**Source**: Actual measurement scenarios from unilang/strs_tools work

**Requirements:**
- **MUST** support these measurement patterns:
  - Single operation timing (`bench_once`)
  - Multi-iteration timing (`bench_function`)
  - Throughput measurement (operations per second)
  - Custom metric collection (memory, cache hits, etc.)
- **SHOULD** provide statistical confidence measures
- **MUST** handle noisy measurements gracefully

---

## Documentation Integration Requirements

### REQ-DOC-001: Markdown File Section Updates
**Source**: "function and structures which often required, for example for finding and patching corresponding section of md file"

**Requirements:**
- **MUST** provide tools for updating specific markdown file sections
- **MUST** preserve non-benchmark content when updating
- **MUST** support standard markdown section patterns (## Performance)
- **SHOULD** handle nested sections and complex document structures

**Technical requirements:**
```rust
// This functionality must be provided
let results = suite.run_all();
results.update_markdown_section("README.md", "## Performance")?;
results.update_markdown_section("docs/performance.md", "## Latest Results")?;
```

### REQ-DOC-002: Version-Controlled Performance Results
**Source**: Need for performance tracking over time

**Requirements:**
- **MUST** generate markdown suitable for version control
- **SHOULD** provide consistent formatting across runs
- **SHOULD** include timestamps and context information
- **MUST** be human-readable and reviewable in PRs

### REQ-DOC-003: Report Template System
**Source**: Different documentation needs for different projects

**Requirements:**
- **MUST** provide customizable report templates
- **SHOULD** support multiple output formats (markdown, HTML, JSON)
- **SHOULD** allow embedding of charts and visualizations
- **MUST** focus on actionable insights rather than raw data

---

## Data Generation Standards

### REQ-DATA-001: Realistic Test Data Patterns
**Source**: Need for representative benchmark data from unilang/strs_tools experience

**Requirements:**
- **MUST** provide generators for common parsing scenarios:
  - Comma-separated lists with configurable sizes
  - Key-value maps with various delimiters
  - Nested data structures (JSON-like)
  - File paths and URLs
  - Command-line argument patterns

**Specific generator requirements:**
```rust
// These generators must be provided
generate_list_data(DataSize::Medium)           // "item1,item2,...,item100"
generate_map_data(DataSize::Small)             // "key1=value1,key2=value2,..."  
generate_enum_data(DataSize::Large)            // "choice1,choice2,...,choice1000"
generate_nested_data(depth: 3, width: 4)      // JSON-like nested structures
```

### REQ-DATA-002: Reproducible Data Generation
**Source**: Need for consistent benchmark results

**Requirements:**
- **MUST** support seeded random generation
- **MUST** produce identical data across runs with same seed
- **SHOULD** optimize generation to minimize benchmark overhead
- **SHOULD** provide lazy generation for large datasets

### REQ-DATA-003: Domain-Specific Patterns
**Source**: Different projects need different data patterns

**Requirements:**
- **MUST** allow custom data generator composition
- **SHOULD** provide domain-specific generators:
  - Parsing test data (CSV, JSON, command args)
  - String processing data (various lengths, character sets)
  - Algorithmic test data (sorted/unsorted arrays, graphs)
- **SHOULD** support parameterized generation functions

---

## Statistical Analysis Requirements

### REQ-STAT-001: Proper Statistical Measures
**Source**: Need for reliable performance measurements

**Requirements:**
- **MUST** provide these statistical measures:
  - Mean, median, min, max execution times
  - Standard deviation and confidence intervals
  - Percentiles (especially p95, p99)
  - Operations per second calculations
- **SHOULD** detect and handle outliers appropriately
- **MUST** provide sample size recommendations

### REQ-STAT-002: Regression Detection
**Source**: Need for performance monitoring in CI/CD

**Requirements:**
- **MUST** support baseline comparison and regression detection
- **MUST** provide configurable regression thresholds (default: 5%)
- **SHOULD** generate CI-friendly reports (pass/fail, exit codes)
- **SHOULD** support performance history tracking

### REQ-STAT-003: Confidence and Reliability
**Source**: Dealing with measurement noise and variability

**Requirements:**
- **MUST** provide confidence intervals for measurements
- **SHOULD** recommend minimum sample sizes for reliability
- **SHOULD** detect when measurements are too noisy for conclusions
- **MUST** handle system noise gracefully (warm-up iterations, etc.)

---

## Feature Organization Principles

### REQ-ORG-001: Modular Feature Design
**Source**: "avoid large overheads, put every extra feature under cargo feature"

**Requirements:**
- **MUST** organize features by functionality and dependencies:
  - Core: `enabled` (no dependencies)
  - Reporting: `markdown_reports`, `html_reports`, `json_reports` 
  - Analysis: `statistical_analysis`, `comparative_analysis`
  - Utilities: `data_generators`, `criterion_compat`
- **MUST** allow independent feature selection
- **SHOULD** provide feature combination presets (default, full, minimal)

### REQ-ORG-002: Backward Compatibility
**Source**: Need to work with existing benchmarking ecosystems

**Requirements:**
- **MUST** provide criterion compatibility layer under feature flag
- **SHOULD** support migration from criterion with minimal code changes
- **SHOULD** work alongside existing criterion benchmarks
- **MUST** not conflict with other benchmarking tools

### REQ-ORG-003: Documentation and Examples
**Source**: Need for clear usage patterns and integration guides

**Requirements:**
- **MUST** provide comprehensive examples for each major feature
- **MUST** document all feature flag combinations and their implications
- **SHOULD** provide integration guides for common scenarios:
  - Unit test integration
  - CI/CD pipeline setup  
  - Documentation automation
  - Multi-algorithm comparison
- **MUST** include troubleshooting guide for common issues

---

## Implementation Priorities

### Phase 1: Core Functionality (MVP)
1. Basic timing and measurement (`enabled`)
2. Simple markdown report generation (`markdown_reports`)
3. Standard data generators (`data_generators`)

### Phase 2: Analysis Tools
1. Comparative analysis (`comparative_analysis`)
2. Statistical analysis (`statistical_analysis`)
3. Regression detection and baseline management

### Phase 3: Advanced Features
1. HTML and JSON reports (`html_reports`, `json_reports`)
2. Criterion compatibility (`criterion_compat`)
3. Optimization hints and recommendations (`optimization_hints`)

### Phase 4: Ecosystem Integration
1. CI/CD tooling and automation
2. IDE integration and tooling support
3. Performance monitoring and alerting

---

## Success Criteria

### User Experience Success Metrics
- [ ] New users can run first benchmark in <5 minutes
- [ ] Integration into existing project requires <10 lines of code
- [ ] Documentation updates happen automatically without manual intervention
- [ ] Performance regressions detected within 1% accuracy

### Technical Success Metrics  
- [ ] Measurement overhead <1% for operations >1ms
- [ ] All features work independently (no hidden dependencies)
- [ ] Compatible with existing criterion benchmarks
- [ ] Memory usage scales linearly with data size

### Ecosystem Success Metrics
- [ ] Used alongside criterion without conflicts
- [ ] Adopted for documentation generation in multiple projects
- [ ] Provides actionable optimization recommendations
- [ ] Reduces benchmarking setup time by >50% compared to manual approaches

---

*This document captures the essential requirements and recommendations derived from real-world benchmarking challenges encountered during unilang and strs_tools performance optimization work. It serves as the definitive guide for benchkit development priorities and design decisions.*