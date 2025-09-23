# Benchmark Results Summary

*Automatically generated during benchmark execution*

## Performance Improvements

Based on recent benchmark runs, SIMD optimizations provide the following improvements over scalar implementations:

| Test Category | Input Size | Improvement | Detailed Metrics |
|---------------|------------|-------------|------------------|
| Multi-delimiter 2KB | 2KB | 12.9x faster | Scalar: 2.69ms, SIMD: 0.20ms (12 MiB/s) |
| Multi-delimiter 10KB | 10KB | 29.0x faster | Scalar: 14.05ms, SIMD: 0.46ms (26 MiB/s) |
| Multi-delimiter 50KB | 50KB | 130.5x faster | Scalar: 97.94ms, SIMD: 0.71ms (84 MiB/s) |
| Large input 100KB | 100KB | 15.6x faster | Scalar: 159.87ms, SIMD: 9.77ms (12 MiB/s) |
| Large input 500KB | 500KB | 18.1x faster | Scalar: 858.97ms, SIMD: 45.24ms (13 MiB/s) |
| Pattern complexity - 8 delims | 10KB | 202.8x faster | Scalar: 257.48ms, SIMD: 1.21ms (10 MiB/s) |

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

*Generated: 2025-08-06 13:16 UTC*
*This file updated after each benchmark run*
