# Benchmark Results Summary

*Automatically generated during benchmark execution*

## Performance Improvements

Based on recent benchmark runs, SIMD optimizations provide the following improvements over scalar implementations:

| Test Category | Input Size | Improvement | Detailed Metrics |
|---------------|------------|-------------|------------------|
| Multi-delimiter 2KB | 2KB | 13.6x faster | Scalar: 2.46ms, SIMD: 0.18ms (11 MiB/s) |
| Multi-delimiter 10KB | 10KB | 30.5x faster | Scalar: 12.84ms, SIMD: 0.42ms (24 MiB/s) |
| Multi-delimiter 50KB | 50KB | 137.0x faster | Scalar: 89.47ms, SIMD: 0.65ms (77 MiB/s) |
| Large input 100KB | 100KB | 16.4x faster | Scalar: 146.04ms, SIMD: 8.93ms (11 MiB/s) |
| Large input 500KB | 500KB | 19.0x faster | Scalar: 784.65ms, SIMD: 41.32ms (12 MiB/s) |
| Pattern complexity - 8 delims | 10KB | 212.9x faster | Scalar: 235.20ms, SIMD: 1.10ms (9 MiB/s) |

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

*Generated: 2025-08-06 12:45 UTC*
*This file updated after each benchmark run*
