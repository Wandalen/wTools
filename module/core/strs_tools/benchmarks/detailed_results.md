# Benchmark Results Summary

*Automatically generated during benchmark execution*

## Performance Improvements

Based on recent benchmark runs, SIMD optimizations provide the following improvements over scalar implementations:

| Test Category | Input Size | Improvement | Detailed Metrics |
|---------------|------------|-------------|------------------|
| Multi-delimiter 2KB | 2KB | 13.0x faster | Scalar: 2.67ms, SIMD: 0.20ms (12 MiB/s) |
| Multi-delimiter 10KB | 10KB | 29.1x faster | Scalar: 13.96ms, SIMD: 0.46ms (26 MiB/s) |
| Multi-delimiter 50KB | 50KB | 131.0x faster | Scalar: 97.32ms, SIMD: 0.71ms (84 MiB/s) |
| Large input 100KB | 100KB | 15.7x faster | Scalar: 158.85ms, SIMD: 9.71ms (12 MiB/s) |
| Large input 500KB | 500KB | 18.1x faster | Scalar: 853.49ms, SIMD: 44.95ms (13 MiB/s) |
| Pattern complexity - 8 delims | 10KB | 203.5x faster | Scalar: 255.84ms, SIMD: 1.20ms (10 MiB/s) |

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

*Generated: 2025-08-06 12:56 UTC*
*This file updated after each benchmark run*
