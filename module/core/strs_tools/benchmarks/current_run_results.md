# Latest Benchmark Execution Results

*Generated: 2025-08-06 10:29 UTC*

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
