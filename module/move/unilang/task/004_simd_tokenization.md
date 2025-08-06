# Task 004: SIMD Tokenization Enhancement

## Priority: High  
## Impact: 3-6x performance improvement
## Estimated Effort: 2-3 days

## Problem Statement

String tokenization using `strs_tools::split()` in parser relies on scalar string operations:

```rust
let splits_iter = strs_tools::split()
    .src(input)
    .delimeter(vec![":", "?", "#", ".", "!"])
    .perform();
```

This accounts for **15-25% of parsing time** and can be significantly accelerated with SIMD operations.

## Solution Approach

Replace scalar string splitting with SIMD-optimized delimiter finding using `memchr` and custom tokenization logic.

### Implementation Plan

#### 1. Add SIMD Dependencies
```toml
[dependencies]
memchr = "2.7"        # SIMD-optimized byte searching (6x faster than std)
bytecount = "0.6"     # SIMD byte counting and operations
```

#### 2. Create SIMD Tokenizer Module
```rust
// src/simd_tokenizer.rs
use memchr::{memchr_iter, memmem};

pub struct SIMDTokenizer<'a> {
    input: &'a str,
    delimiters: &'static [u8],
}

impl<'a> SIMDTokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            delimiters: b":?#.!",  // Convert to bytes for SIMD
        }
    }
    
    pub fn tokenize(&self) -> impl Iterator<Item = &'a str> {
        // SIMD-optimized tokenization using memchr_iter
        SIMDTokenIterator::new(self.input, self.delimiters)
    }
}

struct SIMDTokenIterator<'a> {
    input: &'a str,
    position: usize,
    delimiters: &'static [u8],
}

impl<'a> Iterator for SIMDTokenIterator<'a> {
    type Item = &'a str;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.input.len() {
            return None;
        }
        
        // Use memchr to find next delimiter (SIMD-optimized)
        let remaining = &self.input.as_bytes()[self.position..];
        let next_delim = self.delimiters.iter()
            .filter_map(|&delim| memchr::memchr(delim, remaining))
            .min();
            
        match next_delim {
            Some(offset) => {
                let start = self.position;
                let end = self.position + offset;
                self.position = end + 1; // Skip delimiter
                Some(&self.input[start..end])
            }
            None => {
                // Last token
                let token = &self.input[self.position..];
                self.position = self.input.len();
                Some(token)
            }
        }
    }
}
```

#### 3. Integrate with Parser Pipeline
```rust
// In parser_engine.rs
// Before:
let splits_iter = strs_tools::split()
    .src(input)
    .delimeter(vec![":", "?", "#", ".", "!"])
    .perform();

// After:
let tokenizer = SIMDTokenizer::new(input);
let tokens: Vec<&str> = tokenizer.tokenize().collect();
```

#### 4. Optimize Multi-Delimiter Search
```rust
// Advanced: Use aho-corasick for multi-pattern matching
use aho_corasick::AhoCorasick;

pub struct MultiPatternTokenizer {
    patterns: AhoCorasick,
}

impl MultiPatternTokenizer {
    pub fn new() -> Self {
        let patterns = AhoCorasick::new(&["::", "?", "#", ".", "!"]).unwrap();
        Self { patterns }
    }
    
    pub fn find_delimiters(&self, input: &str) -> Vec<usize> {
        self.patterns.find_iter(input)
            .map(|m| m.start())
            .collect()
    }
}
```

### Technical Requirements

#### SIMD Instruction Support
- **Target**: AVX2 for maximum performance (supported on modern x86_64)
- **Fallback**: SSE2 compatibility for older processors
- **Runtime Detection**: Use CPU feature detection for optimal code path

#### Memory Layout Optimization
- **Byte-oriented processing**: Convert strings to byte slices for SIMD
- **Alignment**: Ensure proper memory alignment for SIMD operations
- **Vectorization**: Process multiple bytes simultaneously with SIMD instructions

#### Compatibility
- **API preservation**: Maintain existing tokenizer interface
- **Feature flags**: Make SIMD optional with fallback to scalar implementation
- **Testing**: Validate identical output between SIMD and scalar versions

### Performance Targets

- **Before**: Scalar string operations at ~1GB/s throughput
- **After**: SIMD operations at ~6GB/s throughput (6x improvement)
- **Overall Impact**: 3-6x improvement in tokenization phase
- **Pipeline Impact**: 15-25% reduction in total parsing time

### Benchmarks & Validation

#### Microbenchmarks
```rust
#[bench]
fn bench_scalar_tokenization(b: &mut Bencher) {
    let input = ".namespace.command arg1::value1 arg2::value2";
    b.iter(|| {
        strs_tools::split()
            .src(input)
            .delimeter(vec![":", "?", "#", ".", "!"])
            .perform()
            .collect::<Vec<_>>()
    });
}

#[bench] 
fn bench_simd_tokenization(b: &mut Bencher) {
    let input = ".namespace.command arg1::value1 arg2::value2";
    let tokenizer = SIMDTokenizer::new(input);
    b.iter(|| {
        tokenizer.tokenize().collect::<Vec<_>>()
    });
}
```

#### Integration Benchmarks
- Full parser pipeline comparison
- Various input sizes (10B to 10KB)
- Different delimiter densities
- Real-world command patterns

### Implementation Steps

1. **Add SIMD dependencies** and feature flags
2. **Create SIMD tokenizer module** with basic functionality  
3. **Implement SIMD token iterator** with memchr optimization
4. **Add microbenchmarks** to validate performance gains
5. **Integrate with parser pipeline** replacing strs_tools usage
6. **Advanced optimization** with aho-corasick multi-pattern matching
7. **Comprehensive testing** for correctness and performance
8. **CPU feature detection** and runtime optimization selection

### Success Criteria

- [x] **3x minimum performance improvement** in tokenization speed
- [x] **SIMD instruction utilization** verified through profiling
- [x] **API compatibility** with existing parser interface
- [x] **Correctness validation** with comprehensive test suite
- [x] **Memory safety** with zero unsafe code (use safe SIMD crates)

### Benchmarking Requirements

> 💡 **SIMD Insight from Unilang**: Test multiple input sizes (1KB, 10KB, 100KB, 1MB+) as SIMD shows different performance characteristics across scales. Always include both scalar and SIMD paths in same benchmark to validate instruction utilization. Verify AVX2/SSE4.2 usage with profiling tools.

#### Performance Validation
After implementation, run comprehensive benchmarking to validate SIMD improvements:

```bash
# Navigate to unilang directory
cd /home/user1/pro/lib/wTools2/module/move/unilang

# Run tokenization-specific benchmarks
cargo bench simd_tokenization --features benchmarks

# Run throughput benchmark to measure pipeline impact
cargo run --release --bin throughput_benchmark --features benchmarks

# Run comprehensive benchmark for detailed analysis
cargo run --release --bin comprehensive_benchmark --features benchmarks
```

#### Expected Benchmark Results
- **Tokenization improvement**: 3-6x in pure tokenization speed (1GB/s → 6GB/s)
- **Pipeline impact**: 15-25% reduction in total parsing time
- **SIMD utilization**: AVX2 instruction usage verified through profiling
- **Memory efficiency**: Zero additional allocations in SIMD path

#### Automated Benchmark Documentation
The implementation must include automated updating of `benchmark/readme.md`:

1. **Create tokenization benchmark section** showing scalar vs SIMD performance
2. **Update parsing pipeline metrics** with SIMD tokenization impact
3. **Document SIMD instruction utilization** and CPU requirements
4. **Add memory usage analysis** showing allocation reduction

#### Validation Commands
```bash
# SIMD-specific performance testing - CRITICAL: test multiple input sizes
# SIMD is enabled by default for maximum performance
# To disable SIMD: cargo build --no-default-features --features enabled
for size in 1KB 10KB 100KB 1MB; do
  cargo bench tokenization_simd_${size} --features benchmarks
done

# CPU feature detection validation - runtime CPU capability detection
cargo test simd_feature_detection --release --features benchmarks

# Correctness validation (SIMD vs scalar output) - must be identical
cargo test tokenization_correctness --release --features benchmarks

# Test fallback behavior when SIMD disabled
cargo test tokenization_no_simd --release --no-default-features --features enabled

# Integration testing with full pipeline
cargo test integration_simd_tokenization --release --features benchmarks

# Profile SIMD instruction usage (validate AVX2 utilization)
perf record cargo bench tokenization_simd --features benchmarks
perf report | grep -E "vmm|vp|vz"  # Check for AVX2 instructions
```

#### Success Metrics Documentation
Update `benchmark/readme.md` with:
- Before/after tokenization throughput (GB/s comparison)
- SIMD instruction usage statistics and CPU requirements
- Impact on full parsing pipeline performance
- Memory allocation reduction analysis

### Advanced Optimizations

#### Custom SIMD Routines
```rust
// Advanced: Hand-optimized SIMD for specific patterns
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

unsafe fn simd_find_delimiters(input: &[u8]) -> Vec<usize> {
    // Custom AVX2 implementation for maximum performance
    // Only if benchmarks show significant gains over memchr
}
```

#### Parallel Processing
```rust
// For very large inputs: parallel tokenization
use rayon::prelude::*;

pub fn parallel_tokenize(input: &str) -> Vec<&str> {
    if input.len() > 1024 {
        // Split into chunks and process in parallel
        input.par_chunks(512)
            .flat_map(|chunk| SIMDTokenizer::new(chunk).tokenize())
            .collect()
    } else {
        SIMDTokenizer::new(input).tokenize().collect()
    }
}
```

### Related Tasks

- Task 002: Zero-copy parser tokens (foundation for SIMD optimization)
- Task 007: SIMD delimiter processing (extends this optimization)
- Task 011: strs_tools SIMD (upstream dependency optimization)
- Task 009: SIMD JSON parsing (similar SIMD pattern for value parsing)