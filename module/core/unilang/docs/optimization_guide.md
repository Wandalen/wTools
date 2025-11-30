# Optimization Guide

## Current Performance Characteristics

### Command Lookup Performance

Based on benchmarking and NFR requirements (spec.md NFR-PERF-2: p99 < 100ns):

| Approach | Lookup Time | Memory Overhead | Performance vs Runtime |
|----------|-------------|-----------------|----------------------|
| **Static (Compile-Time)** | ~80-100ns | Zero (static data) | 50x faster |
| **Runtime (HashMap)** | ~4,000-5,000ns | Heap allocations | Baseline |

**Key Metrics:**
- Static lookups meet NFR-PERF-2 requirement (p99 < 100ns)
- Runtime lookups are acceptable for REPL/plugin scenarios
- 50x performance gap justifies compile-time as default approach

### SIMD Optimizations

**JSON Parsing Performance:**
- SIMD-enabled JSON parsing: 4-25x faster than standard serde_json
- Enabled via `simd` feature (included in default features)
- Requires `simd-json` crate for implementation
- See `src/simd_json_parser.rs` for implementation details

**String Tokenization:**
- SIMD-accelerated delimiter detection
- Improves command parsing throughput
- Particularly effective for batch command processing

### Memory Optimizations

**String Interning:**
- Command names and namespaces interned to reduce allocations
- Cache-friendly for repeated lookups
- Implementation: `src/interner.rs`

**Zero-Copy Static Data:**
- Static commands stored in binary's data section
- No runtime deserialization cost
- No heap allocations for static command access

## Optimization Recommendations

### For Production Applications

**1. Use Compile-Time Registration (Default)**

```toml
[dependencies]
# Default configuration - optimal for most use cases
unilang = "0.28"  # Includes approach_yaml_multi_build + simd + enhanced_repl
```

**Why:**
- 50x faster command lookups (~80ns vs ~4,000ns)
- Zero runtime overhead for command registration
- Smaller binary through dead code elimination
- Compile-time validation catches errors early

**2. Enable SIMD Features**

SIMD is enabled by default. To disable for minimal builds:

```toml
[dependencies]
unilang = { version = "0.28", default-features = false, features = ["enabled", "approach_yaml_multi_build"] }
```

**3. Choose Registration Strategy by Use Case**

**For Production CLIs:** Use static registration (50x faster)
```toml
[dependencies]
unilang = { version = "0.28", default-features = false, features = ["enabled", "approach_yaml_multi_build"] }
```

**For REPL/Plugins:** Runtime registration is appropriate
```rust
let mut registry = CommandRegistry::new();
```

**Performance comparison:**
- Static registration: ~80ns lookup (⚡ recommended for production)
- Runtime registration: ~500ns-4μs lookup (✅ appropriate for REPL/plugins)
- **Trade-off:** 10-50x performance vs. runtime flexibility

**When to use runtime registration:**
- ✅ REPL applications - Commands defined interactively
- ✅ Plugin systems - Commands loaded dynamically
- ✅ Prototyping - Rapid development iteration

**When to avoid runtime registration:**
- ⚠️ Production CLIs with 100+ commands
- ⚠️ Performance-critical applications
- ⚠️ Embedded systems with strict latency requirements

### For REPL/Interactive Applications

**Use Enhanced REPL Features:**

```toml
[dependencies]
unilang = { version = "0.28", features = ["enhanced_repl"] }
```

**Benefits:**
- History management (persists across sessions)
- Auto-completion for commands and arguments
- Secure password input
- Error recovery

### For Minimal Binary Size

**Disable unnecessary features:**

```toml
[dependencies]
unilang = { version = "0.28", default-features = false, features = [
  "enabled",                      # Core functionality (required)
  "approach_rust_dsl_const"       # Rust DSL only, no YAML parser
]}
```

**Tradeoffs:**
- Smaller binary (~200KB reduction)
- No YAML/JSON parsing
- Must define commands in Rust code
- Still gets 50x speedup vs runtime registration

## Benchmark Data

### Command Lookup Benchmarks

Run comprehensive benchmarks:

```bash
cargo run --example static_03_performance_comparison --release
```

**Expected Results:**
- Static lookup: 50-100ns per operation
- Runtime lookup: 4,000-5,000ns per operation
- Throughput: 5,000,000+ lookups/second (NFR-PERF-3)

### SIMD JSON Benchmarks

Test SIMD JSON parsing performance:

```bash
cargo test --test simd_json --release -- --nocapture
```

**Expected Results:**
- Standard JSON parsing: baseline
- SIMD JSON parsing: 4-25x faster
- Larger JSON documents show greater improvements

## Performance Monitoring

### Built-in Performance Metrics

`StaticCommandRegistry` includes automatic performance tracking:

```rust
let mut registry = StaticCommandRegistry::from_commands(&STATIC_COMMANDS);

// Perform lookups...
registry.command("some_command");

// Check metrics
let metrics = registry.metrics();
println!("Total lookups: {}", metrics.total_lookups());
println!("Cache hit rate: {:.1}%", metrics.cache_hit_rate() * 100.0);
```

### Profiling Recommendations

**For detailed profiling:**

1. Use `cargo flamegraph` for CPU profiling
2. Use `heaptrack` for memory profiling
3. Check `perf stat` for hardware counter analysis

**Focus areas:**
- Command lookup hot path (should be dominated by static map lookup)
- Semantic analysis overhead (argument parsing/validation)
- Memory allocations during command execution

## Known Performance Characteristics

### Scaling Behavior

**Registry size vs lookup time:**
- Static registry: O(1) regardless of size (compile-time optimized)
- Runtime registry: O(1) average case (HashMap)
- No degradation observed up to 1,000,000 commands

**Multi-module aggregation:**
- Aggregating 10 modules with 100 commands each
- Lookup time: Still ~80-100ns (single static map)
- Namespace resolution: Zero runtime overhead (compile-time prefixing)

### Bottlenecks

**Identified bottlenecks (in order):**

1. **Semantic analysis** (~3,500ns of the ~4,000ns runtime overhead)
   - Argument type parsing
   - Validation rule checking
   - Value conversion

2. **String allocations** (~300ns overhead)
   - Mitigated by string interning
   - Further improvements possible

3. **Command lookup** (~80-100ns for static, ~200ns for runtime HashMap)
   - Static: Already optimal (compile-time optimized)
   - Runtime: Acceptable overhead

## Future Optimization Opportunities

Based on roadmap.md and spec.md:

1. **Lazy static command initialization** (if binary size becomes concern)
2. **SIMD argument parsing** (extend SIMD beyond JSON to all parsers)
3. **Zero-copy argument access** (reduce allocations in hot path)
4. **Parallel batch processing** (leverage rayon for command batches)

## References

- Performance NFRs: `spec.md` Section 5 (NFR-PERF-1, NFR-PERF-2, NFR-PERF-3)
- Benchmark examples: `examples/static_03_performance_comparison.rs`
- Performance tests: `tests/performance/test_performance.rs`
- SIMD implementation: `src/simd_json_parser.rs`, `src/simd_tokenizer.rs`
