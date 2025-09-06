# Scalar vs SIMD Performance Comparison

**Date**: 2025-08-06  
**Platform**: Linux ARM64  
**Test Framework**: criterion.rs with 100 samples per benchmark

## Executive Summary

Comprehensive benchmarking reveals that SIMD optimizations provide **significant performance improvements** ranging from **1.6x to 330x faster** depending on input size and operation complexity. The most dramatic improvements occur with multi-delimiter operations on larger inputs, where SIMD can be **over 300x faster** than scalar implementations.

## Detailed Performance Analysis

### 1. Single Delimiter String Splitting

| Input Size | Scalar Time | SIMD Time | SIMD Speedup | Scalar Throughput | SIMD Throughput |
|------------|-------------|-----------|--------------|-------------------|-----------------|
| **1KB** | 14.9 µs | 2.2 µs | **6.8x faster** | 64.0 MiB/s | 433.1 MiB/s |
| **10KB** | 160.8 µs | 21.4 µs | **7.5x faster** | 59.3 MiB/s | 445.6 MiB/s |
| **100KB** | 1.6 ms | 212.2 µs | **7.5x faster** | 59.6 MiB/s | 449.1 MiB/s |

**Key Insight**: Single delimiter operations show consistent **6-8x improvement** across all input sizes, with SIMD maintaining high throughput even on large inputs.

### 2. Multi-Delimiter String Splitting (2 delimiters)

| Input Size | Scalar Time | SIMD Time | SIMD Speedup | Scalar Throughput | SIMD Throughput |
|------------|-------------|-----------|--------------|-------------------|-----------------|
| **1KB** | 46.2 µs | 3.1 µs | **14.9x faster** | 20.6 MiB/s | 307.5 MiB/s |
| **10KB** | 472.5 µs | 42.0 µs | **11.3x faster** | 20.2 MiB/s | 227.0 MiB/s |
| **100KB** | 7.2 ms | 421.3 µs | **17.1x faster** | 13.2 MiB/s | 226.4 MiB/s |

**Key Insight**: Multi-delimiter operations with 2 patterns show **11-17x improvement**, with SIMD maintaining superior throughput especially on larger inputs.

### 3. Multi-Delimiter String Splitting (5 delimiters)

| Input Size | Scalar Time | SIMD Time | SIMD Speedup | Scalar Throughput | SIMD Throughput |
|------------|-------------|-----------|--------------|-------------------|-----------------|
| **1KB** | 268.3 µs | 3.7 µs | **72.5x faster** | 3.6 MiB/s | 259.1 MiB/s |
| **10KB** | 2.7 ms | 31.0 µs | **87.1x faster** | 3.5 MiB/s | 307.9 MiB/s |
| **100KB** | 26.9 ms | 293.0 µs | **91.8x faster** | 3.5 MiB/s | 325.5 MiB/s |

**Key Insight**: Medium complexity multi-delimiter operations show **70-90x improvement**, demonstrating SIMD's exceptional efficiency with multiple pattern matching.

### 4. Multi-Delimiter String Splitting (10 delimiters)

| Input Size | Scalar Time | SIMD Time | SIMD Speedup | Scalar Throughput | SIMD Throughput |
|------------|-------------|-----------|--------------|-------------------|-----------------|
| **1KB** | 695.3 µs | 4.2 µs | **165.5x faster** | 1.4 MiB/s | 224.6 MiB/s |
| **10KB** | 9.3 ms | 34.2 µs | **272.0x faster** | 1.0 MiB/s | 279.0 MiB/s |
| **100KB** | 98.4 ms | 297.9 µs | **330.4x faster** | 993 KiB/s | 320.1 MiB/s |

**Key Insight**: Complex multi-delimiter operations show the most dramatic improvements, with SIMD being **up to 330x faster** on large inputs with many delimiters.

### 5. Substring Search Operations

| Pattern | Input Size | Scalar Time | SIMD Time | SIMD Speedup | Scalar Throughput | SIMD Throughput |
|---------|------------|-------------|-----------|--------------|-------------------|-----------------|
| **"pattern"** | 1KB | 223.0 ns | 142.9 ns | **1.6x faster** | 4.2 GiB/s | 6.5 GiB/s |
| **"xyz"** | 1KB | 337.7 ns | - | - | 2.8 GiB/s | - |

**Key Insight**: Substring search shows modest but consistent improvements, with SIMD providing **1.6x speedup** and maintaining high throughput in the GiB/s range.

## Performance Scaling Analysis

### SIMD Benefits by Input Size

1. **Small Inputs (1KB)**: 
   - Single delimiter: 6.8x improvement
   - Multi-delimiter (2): 14.9x improvement  
   - Multi-delimiter (5): 72.5x improvement
   - Multi-delimiter (10): 165.5x improvement

2. **Medium Inputs (10KB)**:
   - Single delimiter: 7.5x improvement
   - Multi-delimiter (2): 11.3x improvement
   - Multi-delimiter (5): 87.1x improvement
   - Multi-delimiter (10): 272.0x improvement

3. **Large Inputs (100KB)**:
   - Single delimiter: 7.5x improvement
   - Multi-delimiter (2): 17.1x improvement
   - Multi-delimiter (5): 91.8x improvement
   - Multi-delimiter (10): 330.4x improvement

### Pattern Complexity Impact

The performance improvement scales exponentially with the number of delimiter patterns:

- **1 delimiter**: ~7x improvement
- **2 delimiters**: ~15x improvement  
- **5 delimiters**: ~85x improvement
- **10 delimiters**: ~255x improvement

This demonstrates SIMD's exceptional efficiency at multi-pattern matching compared to scalar implementations that must check each pattern sequentially.

## Throughput Analysis

### Scalar Implementation Limitations

- **Single delimiter**: Plateaus at ~60 MiB/s regardless of input size
- **Multi-delimiter**: Degrades significantly with pattern count
  - 2 patterns: ~20 MiB/s
  - 5 patterns: ~3.5 MiB/s  
  - 10 patterns: ~1 MiB/s

### SIMD Implementation Advantages

- **Consistent high throughput**: Maintains 200-450 MiB/s across all scenarios
- **Pattern count independence**: Throughput remains high regardless of delimiter count
- **Scalability**: Performance improves or remains stable with larger inputs

## Real-World Impact

### Expected Performance Gains in Typical Applications

1. **Configuration File Parsing** (multiple delimiters, medium files):
   - **Expected improvement**: 50-100x faster
   - **Use case**: Parsing complex config files with multiple separators

2. **CSV/Log Processing** (single delimiter, large files):
   - **Expected improvement**: 7-8x faster
   - **Use case**: Processing large CSV files or log files

3. **Command Line Parsing** (few delimiters, small inputs):
   - **Expected improvement**: 10-20x faster
   - **Use case**: Splitting command arguments and options

4. **Data Import/ETL** (many delimiters, large files):
   - **Expected improvement**: 200-300x faster
   - **Use case**: Processing complex data formats with many field separators

## Conclusion

The SIMD implementation delivers exceptional performance improvements:

- **Minimum improvement**: 1.6x (simple substring search)
- **Typical improvement**: 7-20x (common splitting operations)
- **Maximum improvement**: 330x (complex multi-delimiter operations)

The most significant benefits occur in scenarios that are common in real-world applications:
- Parsing structured data with multiple delimiters
- Processing large files with complex field separators
- ETL operations requiring multiple pattern matching

This validates the SIMD optimization as a highly effective enhancement to the strs_tools crate, providing substantial performance benefits across a wide range of string processing scenarios.