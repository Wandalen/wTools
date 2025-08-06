# Latest Benchmark Execution Results

*Generated: 2025-08-06 12:37 UTC*

## Benchmark Execution Summary

The benchmark system tests three critical bottlenecks:

### 1. Multi-Delimiter Bottleneck
**Purpose**: Tests splitting performance with 3-8 delimiters on realistic data sizes
**Test cases**:
- Medium (2KB): Uses "quick" complexity data with 3 delimiters
- Large (10KB): Uses "quick" complexity data with 5 delimiters  
- Extra Large (50KB): Uses "full" complexity data with 8 delimiters

### 2. Large Input Scalability
**Purpose**: Tests performance scaling from 10KB to 500KB inputs
**Focus**: Memory and throughput bottlenecks for file processing

### 3. Pattern Complexity Impact  
**Purpose**: Compares 1, 3, and 8 delimiter performance
**Focus**: Algorithmic efficiency and SIMD pattern matching benefits

## Current Run Results

### Detailed Timing Data
**Multi-delimiter 2KB** (2KB)
- Scalar: 2.550ms (783.2 MiB/s)
- SIMD: 0.187ms (11.5 MiB/s)
- **Improvement: 13.3x faster**

**Multi-delimiter 10KB** (10KB)
- Scalar: 13.325ms (750.4 MiB/s)
- SIMD: 0.437ms (24.8 MiB/s)
- **Improvement: 29.9x faster**

**Multi-delimiter 50KB** (50KB)
- Scalar: 92.857ms (538.4 MiB/s)
- SIMD: 0.677ms (80.1 MiB/s)
- **Improvement: 134.4x faster**

**Large input 100KB** (100KB)
- Scalar: 151.570ms (659.8 MiB/s)
- SIMD: 9.265ms (11.7 MiB/s)
- **Improvement: 16.1x faster**

**Large input 500KB** (500KB)
- Scalar: 814.374ms (613.9 MiB/s)
- SIMD: 42.889ms (12.6 MiB/s)
- **Improvement: 18.6x faster**

**Pattern complexity - 8 delims** (10KB)
- Scalar: 244.114ms (42.0 MiB/s)
- SIMD: 1.145ms (9.7 MiB/s)
- **Improvement: 208.8x faster**


## Performance Characteristics

### SIMD Advantages
- **Multi-pattern matching**: aho-corasick provides dramatic speedup
- **Large input processing**: memchr optimizations scale well
- **Complex delimiter sets**: More patterns = greater SIMD benefit

### Scalar Fallbacks
- **Small inputs**: SIMD overhead may reduce benefits
- **Simple patterns**: Single delimiter operations show modest improvement
- **No SIMD support**: Graceful fallback to standard implementations

## Benchmark Configuration

- **Framework**: criterion.rs with statistical validation
- **Sample size**: 100 samples per test for accuracy
- **Complexity levels**: "quick" (simple patterns), "full" (complex patterns)
- **Platform**: ARM64 with SIMD instruction support

---

*This file provides technical details for the most recent benchmark execution*
*Updated automatically each time benchmarks are run*
