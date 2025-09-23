# Implement Environment-Specific CV Configuration

## Description

**HIGH PRIORITY VIOLATION**: Usage.md requires different CV targets for different environments. No environment-specific benchmark configuration exists.

**Required Environment Configurations** (from usage.md):

| Environment | Target CV | Sample Count | Primary Focus |
|-------------|-----------|--------------|---------------|
| **Development** | < 15% | 10-20 samples | Quick feedback cycles |
| **CI/CD** | < 10% | 20-30 samples | Reliable regression detection |
| **Production Analysis** | < 5% | 50+ samples | Decision-grade reliability |

**Required Implementation**:
```rust
let config = match std::env::var("BENCHMARK_ENV").as_deref() {
    Ok("production") => BenchmarkConfig {
        regression_threshold: 0.05,  // Strict: 5%
        min_sample_size: 50,
        cv_tolerance: 0.05,
    },
    Ok("staging") => BenchmarkConfig {
        regression_threshold: 0.10,  // Moderate: 10%
        min_sample_size: 20,
        cv_tolerance: 0.10,
    },
    _ => BenchmarkConfig {
        regression_threshold: 0.15,  // Lenient: 15%
        min_sample_size: 10,
        cv_tolerance: 0.15,
    },
};
```

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`
-   Must implement benchkit usage.md "Environment-Specific CV Guidelines" section
-   Related to Task 030 (CV analysis) and Task 037 (CI/CD integration)
-   Must support development, CI/CD, and production analysis environments

## Acceptance Criteria

-   [x] Environment-specific benchmark configurations implemented
-   [x] BENCHMARK_ENV environment variable support
-   [x] Development environment: CV < 15%, 10-20 samples, quick feedback
-   [x] CI/CD environment: CV < 10%, 20-30 samples, regression detection
-   [x] Production environment: CV < 5%, 50+ samples, decision-grade reliability
-   [x] Automatic environment detection and configuration
-   [x] Different tolerance levels for each environment
-   [x] Sample count scaling based on environment requirements
-   [x] Environment-appropriate warmup strategies

## Outcomes

**✅ COMPLETED**: Environment-specific CV configuration system successfully implemented.

### Key Deliverables

1. **New Module**: `src/benchmark_config.rs`
   - Environment detection via `BENCHMARK_ENV` variable
   - Three predefined environment configurations (Development, Staging, Production)
   - Adaptive sample size calculation based on CV quality

2. **Environment Configurations**:
   - **Development**: CV < 15%, 10-20 samples, 30s timeout
   - **Staging/CI**: CV < 10%, 20-30 samples, 120s timeout 
   - **Production**: CV < 5%, 50-100 samples, 600s timeout

3. **Enhanced String Interning Benchmark**: Updated to use environment-specific configuration
   - Automatic environment detection and display
   - Environment-aware CV validation
   - Adaptive sample size recommendations
   - Environment-specific significance thresholds

4. **Comprehensive Testing**: All 7 test cases passing
   - Environment detection for all three modes
   - CV requirement validation
   - Statistical significance testing
   - Adaptive sample size calculation

### Performance Impact

The environment-specific configuration provides:
- **Development**: Fast feedback (30s max) with reasonable accuracy
- **CI/CD**: Reliable regression detection with balanced runtime
- **Production**: Decision-grade statistical rigor for optimization analysis

### Integration

The system integrates seamlessly with benchkit's statistical analysis framework while providing environment-appropriate defaults and validation thresholds.