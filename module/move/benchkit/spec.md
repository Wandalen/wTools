# spec

- **Name:** benchkit
- **Version:** 1.0.0
- **Date:** 2025-08-08
- **Status:** DRAFT

### Table of Contents
* **Part I: Public Contract (Mandatory Requirements)**
  * 1. Vision & Scope
    * 1.1. Core Vision: Practical Benchmarking Toolkit
    * 1.2. In Scope: The Toolkit Philosophy
    * 1.3. Out of Scope
  * 2. System Actors
  * 3. Ubiquitous Language (Vocabulary) 
  * 4. Core Functional Requirements
    * 4.1. Measurement & Timing
    * 4.2. Data Generation
    * 4.3. Report Generation
    * 4.4. Analysis Tools
  * 5. Non-Functional Requirements
  * 6. Feature Flags & Modularity
  * 7. Standard Directory Requirements
* **Part II: Internal Design (Design Recommendations)**
  * 8. Architectural Principles
  * 9. Integration Patterns
* **Part III: Development Guidelines**
  * 10. Lessons Learned Reference
  * 11. Implementation Priorities

---

## Part I: Public Contract (Mandatory Requirements)

### 1. Vision & Scope

#### 1.1. Core Vision: Practical Benchmarking Toolkit

**benchkit** is designed as a **toolkit, not a framework**. Unlike opinionated frameworks that impose specific workflows, benchkit provides flexible building blocks that developers can combine to create custom benchmarking solutions tailored to their specific needs.

**Key Philosophy:**
- **Standard Directory Compliance**: ALL benchmark files must be in standard `benches/` directory
- **Automatic Documentation**: `benches/readme.md` automatically updated with comprehensive reports
- **Research-Grade Statistical Rigor**: Professional statistical analysis meeting publication standards
- **Toolkit over Framework**: Provide tools, not constraints
- **Optimization-Focused**: Surface key metrics that guide optimization decisions
- **Integration-Friendly**: Work alongside existing tools, not replace them

#### 1.2. In Scope: The Toolkit Philosophy

**Core Capabilities:**
1. **Standard Directory Integration**: ALL benchmark files organized in standard `benches/` directory following Rust conventions
2. **Automatic Report Generation**: `benches/readme.md` automatically updated with comprehensive benchmark results and analysis
3. **Flexible Measurement**: Time, memory, throughput, custom metrics with statistical rigor
4. **Data Generation**: Configurable test data generators for common patterns
5. **Analysis Tools**: Statistical analysis, comparative benchmarking, regression detection, git-style diffing, visualization
6. **Living Documentation**: Automatically maintained performance documentation that stays current with code changes

**Target Use Cases:**
- Performance analysis for optimization work
- Before/after comparisons for feature implementation
- Historical performance tracking across commits/versions
- Continuous performance monitoring in CI/CD
- Documentation generation for performance characteristics
- Research and experimentation with algorithm variants

#### 1.3. Out of Scope

**Not Provided:**
- Opinionated benchmark runner (use criterion for that)
- Automatic CI/CD integration (provide tools for manual integration)
- Real-time monitoring (focus on analysis, not monitoring)
- GUI interfaces (command-line and programmatic APIs only)

### 2. System Actors

| Actor | Description | Primary Use Cases |
|-------|-------------|-------------------|
| **Performance Engineer** | Optimizes code performance | Algorithmic comparisons, bottleneck identification |
| **Library Author** | Maintains high-performance libraries | Before/after analysis, performance documentation |
| **CI/CD System** | Automated testing and reporting | Performance regression detection, report generation |
| **Researcher** | Analyzes algorithmic performance | Experimental comparison, statistical analysis |

### 3. Ubiquitous Language (Vocabulary)

| Term | Definition |
|------|------------|
| **Benchmark Suite** | A collection of related benchmarks measuring different aspects of performance |
| **Test Case** | A single benchmark measurement with specific parameters |
| **Performance Profile** | A comprehensive view of performance across multiple dimensions |
| **Comparative Analysis** | Side-by-side comparison of two or more performance profiles |
| **Performance Regression** | A decrease in performance compared to a baseline |
| **Performance Diff** | Git-style comparison showing changes between benchmark results |
| **Optimization Insight** | Actionable recommendation derived from benchmark analysis |
| **Report Template** | A customizable format for presenting benchmark results |
| **Data Generator** | A function that creates test data for benchmarking |
| **Metric Collector** | A component that gathers specific performance measurements |

### 4. Core Functional Requirements

#### 4.1. Measurement & Timing (FR-TIMING)

**FR-TIMING-1: Flexible Timing Interface**
- Must provide simple timing functions for arbitrary code blocks
- Must support nested timing for hierarchical analysis
- Must collect statistical measures (mean, median, min, max, percentiles)

**FR-TIMING-2: Custom Metrics**
- Must support user-defined metrics beyond timing (memory, throughput, etc.)
- Must provide extensible metric collection interface
- Must allow metric aggregation and statistical analysis

**FR-TIMING-3: Baseline Comparison**
- Must support comparing current performance against saved baselines
- Must detect performance regressions automatically
- Must provide percentage improvement/degradation calculations

#### 4.2. Data Generation (FR-DATAGEN)

**FR-DATAGEN-1: Common Patterns**
- Must provide generators for common benchmark data patterns:
  - Lists of varying sizes (small: 10, medium: 100, large: 1000, huge: 10000)
  - Maps with configurable key-value distributions
  - Strings with controlled length and character sets
  - Nested data structures with configurable depth

**FR-DATAGEN-2: Parameterizable Generation**  
- Must allow easy parameterization of data size and complexity
- Must provide consistent seeding for reproducible benchmarks
- Must optimize data generation to minimize benchmark overhead

**FR-DATAGEN-3: Domain-Specific Generators**
- Must allow custom data generators for specific domains
- Must provide composition tools for combining generators
- Must support lazy generation for large datasets

#### 4.3. Report Generation (FR-REPORTS)

**FR-REPORTS-1: Standard Directory Reporting** ⭐ **CRITICAL REQUIREMENT**
- Must generate comprehensive reports in `benches/readme.md` following Rust conventions
- Must automatically update `benches/readme.md` with latest benchmark results
- Must preserve existing content while updating benchmark sections
- Must support updating specific sections of existing markdown files
- **Must use exact section matching to prevent section duplication** - Critical bug fix requirement
- Must validate section names to prevent conflicts and misuse
- Must provide conflict detection for overlapping section names

**FR-REPORTS-2: Multiple Output Formats**
- Must support markdown, HTML, and JSON output formats
- Must provide customizable templates for each format
- Must allow embedding of charts and visualizations

**FR-REPORTS-3: Living Documentation**
- Must generate reports that serve as comprehensive performance documentation
- Must provide clear, actionable summaries of performance characteristics  
- Must highlight key optimization opportunities and bottlenecks
- Must include timestamps and configuration details for reproducibility
- Must maintain historical context and trends in `benches/readme.md`

**FR-REPORTS-4: Safe API Design** ⭐ **CRITICAL REQUIREMENT**
- Must provide section name validation to prevent invalid names (empty, too long, invalid characters)
- Must offer both safe (validated) and unchecked API variants for backwards compatibility
- Must detect and warn about potential section name conflicts before they cause issues
- Must use proper error types (MarkdownError) with clear, actionable error messages
- Must prevent the critical substring matching bug through exact section matching
- Must guide users toward safe section naming practices through API design

#### 4.4. Analysis Tools (FR-ANALYSIS)

**FR-ANALYSIS-1: Research-Grade Statistical Analysis** ⭐ **CRITICAL REQUIREMENT**
- Must provide research-grade statistical rigor meeting publication standards
- Must calculate proper confidence intervals using t-distribution (not normal approximation)
- Must perform statistical significance testing (Welch's t-test for unequal variances)
- Must calculate effect sizes (Cohen's d) for practical significance assessment
- Must detect outliers using statistical methods (IQR method)
- Must assess normality of data distribution (Shapiro-Wilk test)
- Must calculate statistical power for detecting meaningful differences
- Must provide coefficient of variation for measurement reliability assessment
- Must flag unreliable results based on statistical criteria
- Must document statistical methodology in reports

**FR-ANALYSIS-2: Comparative Analysis**
- Must support before/after performance comparisons
- Must provide A/B testing capabilities for algorithm variants
- Must generate comparative reports highlighting differences

**FR-ANALYSIS-3: Git-Style Performance Diffing**
- Must compare benchmark results across different implementations or commits
- Must generate git-style diff output showing performance changes
- Must classify changes as improvements, regressions, or minor variations

**FR-ANALYSIS-4: Visualization and Charts**
- Must generate performance charts for scaling analysis and framework comparison
- Must support multiple output formats (SVG, PNG, HTML)
- Must provide high-level plotting functions for common benchmarking scenarios

**FR-ANALYSIS-5: Optimization Insights**
- Must analyze results to suggest optimization opportunities
- Must identify performance scaling characteristics
- Must provide actionable recommendations based on measurement patterns

### 5. Critical Bug Fixes and Security Requirements

**CBF-1: Markdown Section Duplication Prevention** ⭐ **CRITICAL FIX**

**Background**: A critical substring matching bug was discovered where `MarkdownUpdater.replace_section_content()` used `line.contains()` instead of exact matching for section headers. This caused severe section duplication when section names shared common substrings.

**Impact Evidence**:
- wflow project: readme.md grew from 5,865 to 7,751 lines (+1,886 lines) in one benchmark run
- 37 duplicate "Performance Benchmarks" sections created
- 201 duplicate table headers generated
- Documentation became unusable and contradictory

**Root Cause**: `src/reporting.rs:56` contained:
```rust
if line.contains(self.section_marker.trim_start_matches("## ")) {
```
This matched ANY section containing the substring, so:
- "Performance Benchmarks" ✓ (intended)
- "Language Operations Performance" ✓ (unintended - contains "Performance")
- "Realistic Scenarios Performance" ✓ (unintended - contains "Performance")

**Required Fix**: Changed to exact matching:
```rust
if line.trim() == self.section_marker.trim() {
```

**Prevention Requirements**:
- Must use exact section name matching in all markdown processing
- Must provide comprehensive regression tests for section matching edge cases
- Must validate section names to prevent conflicts
- Must detect and warn about potential substring conflicts
- Must maintain backwards compatibility through unchecked API variants

### 6. Non-Functional Requirements

**NFR-PERFORMANCE-1: Low Overhead**
- Measurement overhead must be <1% of measured operation time for operations >1ms
- Data generation must not significantly impact benchmark timing
- Report generation must complete within 10 seconds for typical benchmark suites

**NFR-USABILITY-1: Simple Integration**  
- Must integrate into existing projects with <10 lines of code
- Must provide sensible defaults for common benchmarking scenarios
- Must allow incremental adoption alongside existing benchmarking tools

**NFR-COMPATIBILITY-1: Environment Support**
- Must work in std environments (primary target)
- Should provide no_std compatibility for core timing functions
- Must support all major platforms (Linux, macOS, Windows)

**NFR-RELIABILITY-1: Reproducible Results**
- Must provide consistent results across multiple runs (±5% variance)
- Must support deterministic seeding for reproducible data generation
- Must handle system noise and provide statistical confidence measures

### 7. Feature Flags & Modularity

| Feature | Description | Default | Dependencies |
|---------|-------------|---------|--------------|
| `enabled` | Core benchmarking functionality | ✓ | - |
| `markdown_reports` | **Safe markdown report generation with exact section matching** ⭐ | ✓ | pulldown-cmark |
| `data_generators` | Common data generation patterns | ✓ | rand |
| `criterion_compat` | Compatibility layer with criterion | ✓ | criterion |
| `html_reports` | HTML report generation | - | tera |
| `json_reports` | JSON report output | - | serde_json |
| `statistical_analysis` | **Research-grade statistical analysis** ⭐ | - | statistical |
| `comparative_analysis` | A/B testing and comparisons | - | - |
| `diff_analysis` | Git-style benchmark result diffing | - | - |
| `visualization` | Chart generation and plotting | - | plotters |
| `optimization_hints` | Performance optimization suggestions | - | statistical_analysis |

**Critical Note**: The `markdown_reports` feature now includes mandatory safety features:
- Section name validation and conflict detection
- Exact section matching (prevents duplication bug)
- MarkdownError type for proper error handling
- Safe/unchecked API variants for backwards compatibility

### 8. Standard Directory Requirements

**SR-DIRECTORY-1: Standard Rust Convention Compliance** ⭐ **MANDATORY**
- ALL benchmark-related files must be located in the standard `benches/` directory
- This follows established Rust ecosystem conventions and ensures compatibility with `cargo bench`
- Benchmark binaries, data generation scripts, and analysis tools must all reside in `benches/`
- No benchmark-related files should be placed in `tests/`, `examples/`, or `src/bin/`

**SR-DIRECTORY-2: Automatic Documentation Generation** ⭐ **MANDATORY**
- `benches/readme.md` must be automatically generated and updated with benchmark results
- The file must serve as comprehensive performance documentation for the project
- Updates must preserve existing content while refreshing benchmark sections
- Reports must include timestamps, configuration details, and historical context

**SR-DIRECTORY-3: Structured Organization**
```
project/
├── benches/
│   ├── readme.md              # Automatically updated comprehensive reports
│   ├── algorithm_comparison.rs # Comparative benchmarks
│   ├── performance_suite.rs    # Main benchmark suite
│   ├── memory_benchmarks.rs    # Memory-specific benchmarks
│   └── data_generation.rs      # Custom data generators
├── src/
│   └── lib.rs                  # Main library code
└── tests/
    └── unit_tests.rs           # Unit tests (NO benchmarks)
```

**SR-DIRECTORY-4: Integration with Rust Toolchain**
- Must work seamlessly with `cargo bench` command
- Must support standard Rust benchmark discovery and execution patterns
- Must integrate with existing Rust development workflows
- Must provide compatibility with IDE tooling and cargo extensions

---

## Part II: Internal Design (Design Recommendations)

### 9. Architectural Principles

**AP-1: Toolkit over Framework**
- Provide composable functions rather than monolithic framework
- Allow users to choose which components to use
- Minimize assumptions about user workflow

**AP-2: Markdown-First Reporting**  
- Treat markdown as first-class output format
- Optimize for readability and version control
- Support inline updates of existing documentation

**AP-3: Zero-Copy Where Possible**
- Minimize allocations during measurement
- Use borrowing and references for data passing
- Optimize hot paths for measurement accuracy

**AP-4: Statistical Rigor**
- Provide proper statistical analysis of results
- Handle measurement noise and outliers appropriately  
- Offer confidence intervals and significance testing

### 10. Integration Patterns

**Pattern 1: Standard Directory Benchmarking**
```rust
// benches/performance_suite.rs
use benchkit::prelude::*;

fn main()
{
  let mut suite = BenchmarkSuite::new( "Core Function Performance" );
  
  suite.benchmark( "small_input", ||
  {
    let data = generate_list_data( 10 );
    bench_block( || my_function( &data ) )
  });
  
  let results = suite.run_all();
  
  // Automatically update benches/readme.md with safe API
  let updater = MarkdownUpdater::new( "benches/readme.md", "Performance Results" ).unwrap();
  updater.update_section( &results.generate_markdown_report() ).unwrap();
}
```

**Pattern 2: Comparative Analysis**
```rust
// benches/algorithm_comparison.rs
use benchkit::prelude::*;

fn main()
{
  let comparison = ComparativeAnalysis::new( "Algorithm Performance Comparison" )
    .algorithm( "original", || original_algorithm( &data ) )
    .algorithm( "optimized", || optimized_algorithm( &data ) )
    .with_data_sizes( &[ 10, 100, 1000, 10000 ] );
  
  let report = comparison.run_comparison();
  
  // Update benches/readme.md with comparison results using safe API
  let updater = MarkdownUpdater::new( "benches/readme.md", "Algorithm Comparison" ).unwrap();
  updater.update_section( &report.generate_markdown_report() ).unwrap();
}
```

**Pattern 3: Comprehensive Benchmark Suite**
```rust
// benches/comprehensive_suite.rs
use benchkit::prelude::*;

fn main()
{
  let mut suite = BenchmarkSuite::new( "Comprehensive Performance Suite" );
  
  // Add multiple benchmark categories
  suite.benchmark( "data_processing", || process_large_dataset() );
  suite.benchmark( "memory_operations", || memory_intensive_task() );
  suite.benchmark( "io_operations", || file_system_benchmarks() );
  
  let results = suite.run_all();
  
  // Generate comprehensive benches/readme.md report with safe API
  let comprehensive_report = results.generate_comprehensive_report();
  let updater = MarkdownUpdater::new( "benches/readme.md", "Performance Analysis" ).unwrap();
  updater.update_section( &comprehensive_report ).unwrap();
  
  println!( "Updated benches/readme.md with comprehensive performance analysis" );
}
```

**Pattern 4: Git-Style Performance Diffing**
```rust
use benchkit::prelude::*;

fn compare_implementations()
{
  // Baseline results (old implementation)
  let baseline_results = vec!
  [
    ( "string_ops".to_string(), bench_function( "old_string_ops", || old_implementation() ) ),
    ( "hash_compute".to_string(), bench_function( "old_hash", || old_hash_function() ) ),
  ];
  
  // Current results (new implementation) 
  let current_results = vec!
  [
    ( "string_ops".to_string(), bench_function( "new_string_ops", || new_implementation() ) ),
    ( "hash_compute".to_string(), bench_function( "new_hash", || new_hash_function() ) ),
  ];
  
  // Generate git-style diff
  let diff_set = diff_benchmark_sets( &baseline_results, &current_results );
  
  // Show summary and detailed analysis
  for diff in &diff_set.diffs
  {
    println!( "{}", diff.to_summary() );
  }
  
  // Check for regressions in CI/CD
  for regression in diff_set.regressions()
  {
    eprintln!( "⚠️ Performance regression detected: {}", regression.benchmark_name );
  }
}
```

**Pattern 5: Custom Metrics**
```rust
use benchkit::prelude::*;

fn memory_benchmark()
{
  let mut collector = MetricCollector::new()
    .with_timing()
    .with_memory_usage()
    .with_custom_metric( "cache_hits", || count_cache_hits() );
    
  let results = collector.measure( || expensive_operation() );
  println!( "{}", results.to_markdown_table() );
}
```

**Pattern 6: Visualization and Charts**
```rust
use benchkit::prelude::*;
use std::path::Path;

fn generate_performance_charts()
{
  // Scaling analysis chart
  let scaling_results = vec!
  [
    (10, bench_function( "test_10", || algorithm_with_n( 10 ) )),
    (100, bench_function( "test_100", || algorithm_with_n( 100 ) )),
    (1000, bench_function( "test_1000", || algorithm_with_n( 1000 ) )),
  ];
  
  plots::scaling_analysis_chart(
    &scaling_results,
    "Algorithm Scaling Performance", 
    Path::new( "docs/scaling_chart.svg" )
  );
  
  // Framework comparison chart
  let framework_results = vec!
  [
    ("Fast Framework".to_string(), bench_function( "fast", || fast_framework() )),
    ("Slow Framework".to_string(), bench_function( "slow", || slow_framework() )),
  ];
  
  plots::framework_comparison_chart(
    &framework_results,
    "Framework Performance Comparison",
    Path::new( "docs/comparison_chart.svg" )
  );
}
```

**Pattern 7: Safe Section Management with Conflict Detection** ⭐ **CRITICAL FEATURE**
```rust
// benches/safe_section_management.rs
use benchkit::prelude::*;

fn main() -> Result<(), benchkit::reporting::MarkdownError>
{
  // Safe API with validation - prevents the critical substring matching bug
  let updater = MarkdownUpdater::new("benches/readme.md", "Performance Results")?;
  
  // Check for potential conflicts before proceeding
  let conflicts = updater.check_conflicts()?;
  if !conflicts.is_empty() {
    println!("⚠️ Warning: Potential section name conflicts detected:");
    for conflict in conflicts {
      println!("  - {}", conflict);
    }
    println!("Consider using more specific section names to avoid duplication.");
  }
  
  // Safe to proceed - exact matching prevents duplication
  let suite = BenchmarkSuite::new("Core Performance");
  let results = suite.run_all();
  updater.update_section(&results.generate_markdown_report())?;
  
  // Example of problematic section names that would be caught:
  // ✅ Good: "Performance Results", "Memory Benchmarks", "API Tests"  
  // ⚠️ Risky: "Performance", "Benchmarks", "Test" (too generic, likely to conflict)
  
  // For backwards compatibility, unchecked API is still available:
  // let unchecked = MarkdownUpdater::new_unchecked("benches/readme.md", "");
  
  Ok(())
}
```

**Pattern 8: Research-Grade Statistical Analysis** ⭐ **CRITICAL FEATURE**
```rust
use benchkit::prelude::*;

fn research_grade_performance_analysis()
{
  // Collect benchmark data with proper sample size
  let algorithm_a_result = bench_function_n( "algorithm_a", 20, || algorithm_a() );
  let algorithm_b_result = bench_function_n( "algorithm_b", 20, || algorithm_b() );
  
  // Professional statistical analysis 
  let analysis_a = StatisticalAnalysis::analyze( &algorithm_a_result, SignificanceLevel::Standard ).unwrap();
  let analysis_b = StatisticalAnalysis::analyze( &algorithm_b_result, SignificanceLevel::Standard ).unwrap();
  
  // Check statistical quality before drawing conclusions
  if analysis_a.is_reliable() && analysis_b.is_reliable()
  {
    // Perform statistical comparison with proper hypothesis testing
    let comparison = StatisticalAnalysis::compare(
      &algorithm_a_result,
      &algorithm_b_result, 
      SignificanceLevel::Standard
    ).unwrap();
    
    println!( "Statistical comparison:" );
    println!( "  Effect size: {:.3} ({})", comparison.effect_size, comparison.effect_size_interpretation() );
    println!( "  P-value: {:.4}", comparison.p_value );
    println!( "  Significant: {}", comparison.is_significant );
    println!( "  Conclusion: {}", comparison.conclusion() );
    
    // Generate research-grade report with methodology
    let report = ReportGenerator::new( "Algorithm Comparison", results );
    let statistical_report = report.generate_statistical_report();
    println!( "{}", statistical_report );
  }
  else
  {
    println!( "⚠️ Results do not meet statistical reliability criteria - collect more data" );
  }
}
```

### 11. Key Learnings from unilang/strs_tools Benchmarking

**Lesson 1: Focus on Key Metrics**
- Surface 2-3 critical performance indicators  
- Hide detailed statistics behind optional analysis
- Provide clear improvement/regression percentages

**Lesson 2: Markdown Integration is Critical**
- Developers want to update documentation automatically
- Version-controlled performance results are valuable
- Manual report copying is error-prone and time-consuming

**Lesson 3: Data Generation Patterns**
- Common patterns: small (10), medium (100), large (1000), huge (10000)
- Parameterizable generators reduce boilerplate significantly
- Reproducible seeding is essential for consistent results

**Lesson 4: Statistical Rigor Matters**
- Raw numbers without confidence intervals are misleading
- Outlier detection and handling improves result quality
- Multiple sampling provides more reliable measurements

**Lesson 5: Git-Style Diffing for Performance**
- Developers are familiar with git diff workflow and expect similar experience
- Performance changes should be as easy to review as code changes
- Historical comparison across commits/implementations is essential for CI/CD

**Lesson 6: Integration Simplicity**
- Developers abandon tools that require extensive setup
- Default configurations should work for 80% of use cases
- Incremental adoption is more successful than wholesale replacement

---

---

## Part III: Development Guidelines

### 12. Lessons Learned Reference

**CRITICAL**: All development decisions for benchkit are based on real-world experience from unilang and strs_tools benchmarking work. The complete set of requirements, anti-patterns, and lessons learned is documented in [`recommendations.md`](recommendations.md).

**Key lessons that shaped benchkit design:**

#### 9.1. Toolkit vs Framework Decision
- **Problem**: Criterion's framework approach was too restrictive for our use cases
- **Solution**: benchkit provides building blocks, not rigid workflows
- **Evidence**: "I don't want to mess with all that problem I had" - User feedback on complexity

#### 9.2. Markdown-First Integration
- **Problem**: Manual copy-pasting of performance results into documentation
- **Solution**: Automated markdown section updating with version control friendly output
- **Evidence**: Frequent need to update README performance sections during optimization

#### 9.3. Standard Data Size Patterns
- **Problem**: Inconsistent data sizes across different benchmarks made comparison difficult
- **Solution**: Standardized DataSize enum with proven effective sizes
- **Evidence**: "Common patterns: small (10), medium (100), large (1000), huge (10000)"

#### 9.4. Feature Flag Philosophy
- **Problem**: Heavy dependencies slow compilation and increase complexity
- **Solution**: Granular feature flags for all non-core functionality
- **Evidence**: "put every extra feature under cargo feature" - Explicit requirement

#### 9.5. Focus on Key Metrics
- **Problem**: Statistical details overwhelm users seeking optimization guidance
- **Solution**: Surface 2-3 key indicators, hide details behind optional analysis
- **Evidence**: "expose just few critical parameters of optimization and hid the rest deeper"

#### 9.6. Critical Substring Matching Bug ⭐ **CRITICAL LESSON**
- **Problem**: Markdown section updates used substring matching causing exponential duplication
- **Impact**: Files grew from 5,865 to 7,751 lines in one run, 37 duplicate sections created
- **Root Cause**: `line.contains()` matched overlapping section names like "Performance" 
- **Solution**: Exact matching with `line.trim() == section_marker.trim()` + API validation
- **Prevention**: Safe API with conflict detection, comprehensive regression tests, backwards compatibility

**For complete requirements and anti-patterns, see [`recommendations.md`](recommendations.md).**

### 13. Implementation Priorities

Based on real-world usage patterns and critical path analysis from unilang/strs_tools work:

#### Phase 1: Core Functionality (MVP)
**Justification**: Essential for any benchmarking work
1. Basic timing and measurement (`enabled`)
2. Simple markdown report generation (`markdown_reports`)  
3. Standard data generators (`data_generators`)

#### Phase 2: Analysis Tools  
**Justification**: Essential for professional performance analysis
1. **Research-grade statistical analysis (`statistical_analysis`)** ⭐ **CRITICAL**
2. Comparative analysis (`comparative_analysis`)
3. Git-style performance diffing (`diff_analysis`)
4. Regression detection and baseline management

#### Phase 3: Advanced Features
**Justification**: Nice-to-have for comprehensive analysis
1. Chart generation and visualization (`visualization`)
2. HTML and JSON reports (`html_reports`, `json_reports`)
3. Criterion compatibility (`criterion_compat`)
4. Optimization hints and recommendations (`optimization_hints`)

#### Phase 4: Ecosystem Integration
**Justification**: Long-term adoption and CI/CD integration
1. CI/CD tooling and automation
2. IDE integration and tooling support  
3. Performance monitoring and alerting

### Success Criteria

**User Experience Success Metrics:**
- [ ] New users can run first benchmark in <5 minutes
- [ ] Integration requires <10 lines of code
- [ ] Documentation updates happen automatically
- [ ] Performance regressions detected within 1% accuracy
- [x] **Critical substring matching bug eliminated** - No more section duplication
- [x] **Safe API prevents common mistakes** - Validation guides users to best practices

**Technical Success Metrics:**
- [ ] Measurement overhead <1% for operations >1ms
- [ ] All features work independently
- [ ] Compatible with existing criterion benchmarks
- [ ] Memory usage scales linearly with data size
- [x] **Exact section matching prevents document corruption**
- [x] **Comprehensive regression tests prevent bug recurrence**
- [x] **Backwards compatibility maintained through unchecked API variants**

### Reference Documents

- **[`recommendations.md`](recommendations.md)** - Complete requirements from real-world experience
- **[`readme.md`](readme.md)** - Usage-focused documentation with examples  
- **[`examples/`](examples/)** - Comprehensive usage demonstrations