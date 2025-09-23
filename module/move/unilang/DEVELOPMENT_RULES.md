# Development Rules for Unilang

**CRITICAL: Read before making ANY changes to this codebase**

This project strictly follows design rules from `$PRO/genai/code/rules/code_design.rulebook.md`. Violations will be rejected.

## Quick Reference Card

### ✅ ALLOWED
| What | Where | Example |
|------|-------|---------|
| Unit tests | `tests/` | `#[test] fn test_correctness() { assert_eq!(result, expected); }` |
| Integration tests | `tests/` | Testing public APIs and workflows |
| Performance optimizations | `src/` | LRU cache, PHF maps, SIMD in production code |
| Production monitoring | `src/` | `metrics.cache_hit_rate()` for logging |

### ❌ PROHIBITED
| What | Where | Why | Use Instead |
|------|-------|-----|-------------|
| Custom timing | `tests/` | `std::time::Instant` in tests | `benchkit` framework |
| Performance assertions | `tests/` | `assert!(ops_per_sec > 1000)` | Functional assertions only |
| Benchmarks as tests | `tests/` | Speed comparisons | Separate `benchkit` infrastructure |
| Missing Test Matrix | `tests/` | No `//! Test Matrix` comment | Add mandatory documentation |

## Common Violations (AVOID THESE)

### 1. ❌ Performance Testing in `tests/` Directory

```rust
// WRONG - This violates design rules
#[test]
fn test_performance() {
    let start = std::time::Instant::now();
    let result = expensive_operation();
    let duration = start.elapsed();
    assert!(duration < Duration::from_millis(100)); // VIOLATION
}
```

**Problem:** Mixing performance measurement with unit testing.
**Solution:** Use `benchkit` framework separately.

### 2. ❌ Speed Comparisons in Tests

```rust
// WRONG - This violates design rules
#[test]
fn test_optimization_effectiveness() {
    let optimized_time = time_optimized_function();
    let baseline_time = time_baseline_function();
    assert!(optimized_time < baseline_time); // VIOLATION
}
```

**Problem:** Performance comparison belongs in benchmarks, not tests.
**Solution:** Test correctness only in `tests/`, use `benchkit` for performance.

### 3. ❌ Missing Test Documentation

```rust
// WRONG - Missing mandatory Test Matrix
#[test]
fn some_test() { /* ... */ }
```

**Problem:** Every test file must have Test Matrix documentation.
**Solution:** Add file-level documentation:

```rust
//! ## Test Matrix for Feature Name
//!
//! | ID | Test Case | Expected Result |
//! |----|-----------|-----------------|
//! | TC1 | Basic functionality | Success |

/// Test basic functionality
///
/// **Test Combination ID:** TC1
#[test]
fn test_basic_functionality() { /* ... */ }
```

## ✅ Correct Patterns

### Performance Optimization Implementation
```rust
// CORRECT - Performance optimization in production code
pub struct OptimizedRegistry {
    cache: LruCache<String, Command>,     // ✅ Production optimization
    metrics: PerformanceMetrics,          // ✅ Production monitoring
}

impl OptimizedRegistry {
    pub fn lookup(&mut self, name: &str) -> Option<Command> {
        // ✅ Production performance optimization
        if let Some(cmd) = self.cache.get(name) {
            self.metrics.cache_hits += 1;
            return Some(cmd.clone());
        }
        // Continue with fallback logic...
    }
}
```

### Correct Testing Approach
```rust
//! ## Test Matrix for Registry
//!
//! | TC1 | Register command | Success |
//! | TC2 | Lookup existing command | Found |
//! | TC3 | Lookup missing command | None |

/// Test command registration functionality
///
/// **Test Combination ID:** TC1
#[test]
fn test_register_command() {
    let mut registry = Registry::new();
    let cmd = Command::new("test");

    // ✅ Test correctness, not performance
    let result = registry.register(cmd);
    assert!(result.is_ok());

    // ✅ Verify functional behavior
    let found = registry.lookup("test");
    assert!(found.is_some());
}
```

### Production Monitoring (Allowed)
```rust
// ✅ CORRECT - Production monitoring and logging
pub fn monitor_performance(&self) {
    let metrics = self.performance_metrics();
    if metrics.cache_hit_rate() < 0.8 {
        log::warn!("Cache hit rate below threshold: {:.2}%",
                   metrics.cache_hit_rate() * 100.0);
    }
}
```

## Directory Structure Rules

```
unilang/
├── src/                    ✅ Production code + optimizations
│   ├── lib.rs             ✅ Core implementation
│   ├── registry.rs        ✅ LRU cache, PHF, performance optimizations
│   └── simd_*.rs          ✅ SIMD optimizations
├── tests/                 ✅ Unit/integration tests (correctness only)
│   ├── README_DESIGN_RULES.md  ✅ This file explains rules
│   └── *.rs               ✅ Functional tests with Test Matrix docs
├── build.rs               ✅ PHF generation (build-time optimization)
└── benches/               ✅ Future: benchkit performance tests (if added)
```

## Emergency Rule Violations

If you accidentally violate rules:

1. **Remove violating code immediately**
2. **Move performance testing to `benchkit` framework**
3. **Add Test Matrix documentation to test files**
4. **Run `cargo test` to verify correctness still works**

## References

- **Primary Rules:** `$PRO/genai/code/rules/code_design.rulebook.md`
- **Style Rules:** `$PRO/genai/code/rules/code_style.rulebook.md`
- **Benchmarking:** Use `benchkit` framework only
- **Test Organization:** `tests/` for correctness, `benchkit` for performance

---

**Remember: Separation of concerns is not optional. Performance belongs in production code and benchkit. Tests belong in tests/ for correctness only.**