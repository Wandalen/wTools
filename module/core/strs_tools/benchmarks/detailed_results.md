# Benchmark Results Summary

*Automatically generated during benchmark execution*

## Performance Improvements

Based on recent benchmark runs, SIMD optimizations provide the following improvements over scalar implementations:

| Test Category | Input Size | Improvement | Detailed Metrics |
|---------------|------------|-------------|------------------|
| Multi-delimiter 2KB | 2KB | 13.5x faster | Scalar: 2.49ms, SIMD: 0.18ms (11 MiB/s) |
| Multi-delimiter 10KB | 10KB | 30.2x faster | Scalar: 13.02ms, SIMD: 0.43ms (24 MiB/s) |
| Multi-delimiter 50KB | 50KB | 136.0x faster | Scalar: 90.72ms, SIMD: 0.66ms (78 MiB/s) |
| Large input 100KB | 100KB | 16.3x faster | Scalar: 148.08ms, SIMD: 9.05ms (11 MiB/s) |
| Large input 500KB | 500KB | 18.8x faster | Scalar: 795.60ms, SIMD: 41.90ms (12 MiB/s) |
| Pattern complexity - 8 delims | 10KB | 211.4x faster | Scalar: 238.49ms, SIMD: 1.12ms (9 MiB/s) |

## Bottleneck Analysis

### Critical Performance Factors
1. **Multi-delimiter operations** show the largest SIMD benefits
2. **Input size scaling** - benefits increase with data size  
3. **Pattern complexity** - more delimiters = greater SIMD advantage

### Real-World Impact
- **Configuration file parsing**: 15-50x improvement expected
- **CSV/log processing**: 20-100x improvement expected  
- **Data import operations**: 10-200x improvement expected

---

*Generated: 2025-08-06 11:40 UTC*
*This file updated after each benchmark run*
