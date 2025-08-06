# Benchmark Results Summary

*Automatically generated during benchmark execution*

## Performance Improvements

Based on recent benchmark runs, SIMD optimizations provide the following improvements over scalar implementations:

| Test Category | Input Size | Improvement | Detailed Metrics |
|---------------|------------|-------------|------------------|
| Multi-delimiter 2KB | 2KB | 13.3x faster | Scalar: 2.55ms, SIMD: 0.19ms (12 MiB/s) |
| Multi-delimiter 10KB | 10KB | 29.9x faster | Scalar: 13.32ms, SIMD: 0.44ms (25 MiB/s) |
| Multi-delimiter 50KB | 50KB | 134.4x faster | Scalar: 92.86ms, SIMD: 0.68ms (80 MiB/s) |
| Large input 100KB | 100KB | 16.1x faster | Scalar: 151.57ms, SIMD: 9.26ms (12 MiB/s) |
| Large input 500KB | 500KB | 18.6x faster | Scalar: 814.37ms, SIMD: 42.89ms (13 MiB/s) |
| Pattern complexity - 8 delims | 10KB | 208.8x faster | Scalar: 244.11ms, SIMD: 1.15ms (10 MiB/s) |

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

*Generated: 2025-08-06 12:37 UTC*
*This file updated after each benchmark run*
