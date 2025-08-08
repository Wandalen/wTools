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

---

## Part I: Public Contract (Mandatory Requirements)

### 1. Vision & Scope

#### 1.1. Core Vision: Practical Benchmarking Toolkit

**benchkit** is designed as a **toolkit, not a framework**. Unlike opinionated frameworks that impose specific workflows, benchkit provides flexible building blocks that developers can combine to create custom benchmarking solutions tailored to their specific needs.

**Key Philosophy:**
- **Toolkit over Framework**: Provide tools, not constraints
- **Markdown-First Reporting**: Focus on readable, version-controllable reports
- **Optimization-Focused**: Surface key metrics that guide optimization decisions
- **Integration-Friendly**: Work alongside existing tools, not replace them

#### 1.2. In Scope: The Toolkit Philosophy

**Core Capabilities:**
1. **Flexible Measurement**: Time, memory, throughput, custom metrics
2. **Data Generation**: Configurable test data generators for common patterns
3. **Report Generation**: Markdown, HTML, JSON outputs with customizable templates
4. **Analysis Tools**: Statistical analysis, comparative benchmarking, regression detection
5. **Documentation Integration**: Seamlessly update markdown documentation with benchmark results

**Target Use Cases:**
- Performance analysis for optimization work
- Before/after comparisons for feature implementation
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

**FR-ANALYSIS-1: Statistical Analysis**
- Must provide standard statistical measures for benchmark results
- Must detect outliers and provide confidence intervals
- Must support multiple sampling strategies

**FR-ANALYSIS-2: Comparative Analysis**
- Must support before/after performance comparisons
- Must provide A/B testing capabilities for algorithm variants
- Must generate comparative reports highlighting differences

**FR-ANALYSIS-3: Optimization Insights**
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
| `statistical_analysis` | Advanced statistical analysis | - | statistical |
| `comparative_analysis` | A/B testing and comparisons | - | - |
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

fn benchmark_my_function() {
    let mut suite = BenchmarkSuite::new("my_function_performance");
    
    suite.benchmark("small_input", || {
        let data = generate_list_data(10);
        bench_block(|| my_function(&data))
    });
    
    suite.generate_markdown_report("performance.md", "## Performance Results");
}
```

**Pattern 2: Comparative Analysis**
```rust
use benchkit::prelude::*;

fn compare_algorithms() {
    let comparison = ComparativeAnalysis::new()
        .algorithm("original", || original_algorithm(&data))
        .algorithm("optimized", || optimized_algorithm(&data))
        .with_data_sizes(&[10, 100, 1000, 10000]);
    
    let report = comparison.run_comparison();
    report.update_markdown_section("README.md", "## Algorithm Comparison");
}
```

**Pattern 3: Documentation Integration**
```rust
use benchkit::prelude::*;

#[cfg(test)]
mod performance_tests {
    #[test]
    fn update_performance_documentation() {
        let suite = BenchmarkSuite::from_config("benchmarks/config.toml");
        let results = suite.run_all();
        
        // Update multiple sections in documentation
        results.update_markdown_file("docs/performance.md");
        results.update_readme_section("README.md", "## Performance");
    }
}
```

**Pattern 4: Custom Metrics**
```rust
use benchkit::prelude::*;

fn memory_benchmark() {
    let mut collector = MetricCollector::new()
        .with_timing()
        .with_memory_usage()
        .with_custom_metric("cache_hits", || count_cache_hits());
        
    let results = collector.measure(|| expensive_operation());
    println!("{}", results.to_markdown_table());
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

**Lesson 5: Integration Simplicity**
- Developers abandon tools that require extensive setup
- Default configurations should work for 80% of use cases
- Incremental adoption is more successful than wholesale replacement

---

### Implementation Priority

1. **Phase 1**: Core timing and measurement (`enabled`)
2. **Phase 2**: Basic markdown report generation (`markdown_reports`)  
3. **Phase 3**: Data generators and common patterns (`data_generators`)
4. **Phase 4**: Comparative analysis capabilities (`comparative_analysis`)
5. **Phase 5**: Advanced features (HTML, statistical analysis, optimization hints)