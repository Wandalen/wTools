# Latest Benchmark Execution Results

*Generated: 2025-08-06 13:16 UTC*

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
- Scalar: 2.690ms (742.5 MiB/s)
- SIMD: 0.198ms (12.2 MiB/s)
- **Improvement: 12.9x faster**

**Multi-delimiter 10KB** (10KB)
- Scalar: 14.054ms (711.5 MiB/s)
- SIMD: 0.461ms (26.1 MiB/s)
- **Improvement: 29.0x faster**

**Multi-delimiter 50KB** (50KB)
- Scalar: 97.942ms (510.5 MiB/s)
- SIMD: 0.714ms (84.5 MiB/s)
- **Improvement: 130.5x faster**

**Large input 100KB** (100KB)
- Scalar: 159.869ms (625.5 MiB/s)
- SIMD: 9.772ms (12.3 MiB/s)
- **Improvement: 15.6x faster**

**Large input 500KB** (500KB)
- Scalar: 858.965ms (582.1 MiB/s)
- SIMD: 45.238ms (13.3 MiB/s)
- **Improvement: 18.1x faster**

**Pattern complexity - 8 delims** (10KB)
- Scalar: 257.481ms (39.8 MiB/s)
- SIMD: 1.208ms (10.2 MiB/s)
- **Improvement: 202.8x faster**


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
