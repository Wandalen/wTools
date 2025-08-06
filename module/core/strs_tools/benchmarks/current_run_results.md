# Latest Benchmark Execution Results

*Generated: 2025-08-06 11:40 UTC*

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
- Scalar: 2.492ms (801.7 MiB/s)
- SIMD: 0.183ms (11.3 MiB/s)
- **Improvement: 13.5x faster**

**Multi-delimiter 10KB** (10KB)
- Scalar: 13.018ms (768.1 MiB/s)
- SIMD: 0.427ms (24.2 MiB/s)
- **Improvement: 30.2x faster**

**Multi-delimiter 50KB** (50KB)
- Scalar: 90.716ms (551.1 MiB/s)
- SIMD: 0.661ms (78.2 MiB/s)
- **Improvement: 136.0x faster**

**Large input 100KB** (100KB)
- Scalar: 148.075ms (675.3 MiB/s)
- SIMD: 9.051ms (11.4 MiB/s)
- **Improvement: 16.3x faster**

**Large input 500KB** (500KB)
- Scalar: 795.599ms (628.4 MiB/s)
- SIMD: 41.900ms (12.3 MiB/s)
- **Improvement: 18.8x faster**

**Pattern complexity - 8 delims** (10KB)
- Scalar: 238.486ms (43.0 MiB/s)
- SIMD: 1.119ms (9.5 MiB/s)
- **Improvement: 211.4x faster**


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
