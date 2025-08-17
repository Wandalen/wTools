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
* **Part II: Internal Design (Design Recommendations)**
  * 7. Architectural Principles
  * 8. Integration Patterns
* **Part III: Development Guidelines**
  * 9. Lessons Learned Reference
  * 10. Implementation Priorities

---

## Part I: Public Contract (Mandatory Requirements)

### 1. Vision & Scope

#### 1.1. Core Vision: Practical Benchmarking Toolkit

**benchkit** is designed as a **toolkit, not a framework**. Unlike opinionated frameworks that impose specific workflows, benchkit provides flexible building blocks that developers can combine to create custom benchmarking solutions tailored to their specific needs.

**Key Philosophy:**
- **Toolkit over Framework**: Provide tools, not constraints
- **Research-Grade Statistical Rigor**: Professional statistical analysis meeting publication standards
- **Markdown-First Reporting**: Focus on readable, version-controllable reports  
- **Optimization-Focused**: Surface key metrics that guide optimization decisions
- **Integration-Friendly**: Work alongside existing tools, not replace them

#### 1.2. In Scope: The Toolkit Philosophy

**Core Capabilities:**
1. **Flexible Measurement**: Time, memory, throughput, custom metrics
2. **Data Generation**: Configurable test data generators for common patterns
3. **Report Generation**: Markdown, HTML, JSON outputs with customizable templates
4. **Analysis Tools**: Statistical analysis, comparative benchmarking, regression detection, git-style diffing, visualization
5. **Documentation Integration**: Seamlessly update markdown documentation with benchmark results

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

**FR-REPORTS-1: Markdown Integration**
- Must generate markdown tables and sections for benchmark results
- Must support updating specific sections of existing markdown files
- Must preserve non-benchmark content when updating documents

**FR-REPORTS-2: Multiple Output Formats**
- Must support markdown, HTML, and JSON output formats
- Must provide customizable templates for each format
- Must allow embedding of charts and visualizations

**FR-REPORTS-3: Documentation Focus**
- Must generate reports suitable for inclusion in documentation
- Must provide clear, actionable summaries of performance characteristics  
- Must highlight key optimization opportunities and bottlenecks

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

### 5. Non-Functional Requirements

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

### 6. Feature Flags & Modularity

| Feature | Description | Default | Dependencies |
|---------|-------------|---------|--------------|
| `enabled` | Core benchmarking functionality | ✓ | - |
| `markdown_reports` | Markdown report generation | ✓ | pulldown-cmark |
| `data_generators` | Common data generation patterns | ✓ | rand |
| `criterion_compat` | Compatibility layer with criterion | ✓ | criterion |
| `html_reports` | HTML report generation | - | tera |
| `json_reports` | JSON report output | - | serde_json |
| `statistical_analysis` | **Research-grade statistical analysis** ⭐ | - | statistical |
| `comparative_analysis` | A/B testing and comparisons | - | - |
| `diff_analysis` | Git-style benchmark result diffing | - | - |
| `visualization` | Chart generation and plotting | - | plotters |
| `optimization_hints` | Performance optimization suggestions | - | statistical_analysis |

---

## Part II: Internal Design (Design Recommendations)

### 7. Architectural Principles

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

### 8. Integration Patterns

**Pattern 1: Inline Benchmarking**
```rust
use benchkit::prelude::*;

fn benchmark_my_function()
{
  let mut suite = BenchmarkSuite::new( "my_function_performance" );
  
  suite.benchmark( "small_input", ||
  {
    let data = generate_list_data( 10 );
    bench_block( || my_function( &data ) )
  });
  
  suite.generate_markdown_report( "performance.md", "## Performance Results" );
}
```

**Pattern 2: Comparative Analysis**
```rust
use benchkit::prelude::*;

fn compare_algorithms()
{
  let comparison = ComparativeAnalysis::new()
    .algorithm( "original", || original_algorithm( &data ) )
    .algorithm( "optimized", || optimized_algorithm( &data ) )
    .with_data_sizes( &[ 10, 100, 1000, 10000 ] );
  
  let report = comparison.run_comparison();
  report.update_markdown_section( "README.md", "## Algorithm Comparison" );
}
```

**Pattern 3: Documentation Integration**
```rust
use benchkit::prelude::*;

#[ cfg( test ) ]
mod performance_tests
{
  #[ test ]
  fn update_performance_documentation()
  {
    let suite = BenchmarkSuite::from_config( "benchmarks/config.toml" );
    let results = suite.run_all();
    
    // Update multiple sections in documentation
    results.update_markdown_file( "docs/performance.md" );
    results.update_readme_section( "README.md", "## Performance" );
  }
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

**Pattern 7: Research-Grade Statistical Analysis** ⭐ **CRITICAL FEATURE**
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

### 9. Key Learnings from unilang/strs_tools Benchmarking

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

### 9. Lessons Learned Reference

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

**For complete requirements and anti-patterns, see [`recommendations.md`](recommendations.md).**

### 10. Implementation Priorities

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

**Technical Success Metrics:**
- [ ] Measurement overhead <1% for operations >1ms
- [ ] All features work independently
- [ ] Compatible with existing criterion benchmarks
- [ ] Memory usage scales linearly with data size

### Reference Documents

- **[`recommendations.md`](recommendations.md)** - Complete requirements from real-world experience
- **[`readme.md`](readme.md)** - Usage-focused documentation with examples  
- **[`examples/`](examples/)** - Comprehensive usage demonstrations