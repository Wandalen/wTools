<!-- Last updated: 2025-08-06 10:32:09 UTC -->
# Unilang Performance Benchmarks

Performance benchmarking suite focused on bottleneck analysis and optimization tracking. SIMD optimizations enabled by default for maximum performance.

## 🎯 Key Performance Results

| Framework | Commands/sec | vs Unilang (SIMD) | Primary Strength |
|-----------|--------------|------------|------------------|
| **Pico-Args** | ~6.2M | **116x faster** | Zero-copy parsing |
| **Clap** | ~87K | **1.6x faster** | Rich feature set |
| **Unilang (SIMD)** | ~53K | *baseline* | Multi-modal interface |
| **Unilang (No SIMD)** | ~45K | **1.2x slower** | Multi-modal, no SIMD |

**Performance Gap**: Unilang (SIMD) is currently **116x slower** than Pico-Args, representing the primary optimization target.
**SIMD Benefit**: SIMD optimizations provide a **1.2x performance improvement** over standard string operations.

## 🔬 Available Benchmarks

### Throughput Benchmark (Daily Use)
**Command**: `cargo run --release --bin throughput_benchmark --features benchmarks`
- **Duration**: 30-60 seconds
- **Focus**: Runtime performance across command counts (10¹ to 10⁵)
- **Output**: Commands per second comparison

### Comprehensive Framework Comparison
**Command**: `cargo run --release --bin comprehensive_benchmark --features benchmarks`
- **Duration**: 8-10 minutes  
- **Focus**: Complete analysis including build time, binary size, runtime
- **Output**: Full performance matrix with bottleneck identification

## 📊 Bottleneck Analysis

### Primary Bottlenecks (by impact)
1. **String Allocation** (40-60% of hot path) - Token creation overhead
2. **Command Name Construction** (10-15% of hot path) - Repeated format! calls  
3. **JSON Parsing** (varies by workload) - Standard serde_json overhead
4. **String Tokenization** (15-25% of parsing) - Scalar string operations

### SIMD Optimization Status
- ✅ **Enabled by default** for maximum performance
- ✅ **JSON parsing**: 4-25x faster (simd-json)
- ✅ **String operations**: 6x faster (memchr, aho-corasick)
- ✅ **Tokenization**: 3-6x faster (AVX2/SSE4.2/NEON)

## 🚀 Running Benchmarks

**Quick validation**:
```bash
cargo run --release --bin throughput_benchmark --features benchmarks
```

**Complete analysis**:
```bash
./benchmarks/run_comprehensive_benchmark.sh
```

**All benchmarks with documentation updates**:
```bash
./benchmarks/run_all_benchmarks.sh
```

## 📈 Performance Tracking

- **Results automatically update** this file and detailed analysis files
- **No manual editing required** - all metrics generated during benchmark execution
- **Focus on closing the 167x performance gap** with Pico-Args through systematic optimization

---
*Last updated: [Auto-generated during benchmark execution on 2025-08-06 10:32:09 UTC]*