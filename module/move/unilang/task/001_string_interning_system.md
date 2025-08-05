# Task 001: Implement String Interning System

## Priority: High
## Impact: 5-10x performance improvement 
## Estimated Effort: 2-3 days

## Problem Statement

Command name construction in `semantic.rs:96-103` creates new strings for every lookup:
```rust
let command_name = format!(".{}", instruction.command_path_slices.join("."));
```
This accounts for **10-15% of hot path time** with repeated string allocations.

## Solution Approach

Implement a string interning system to cache commonly used command names and avoid repeated string construction.

### Implementation Plan

#### 1. Create String Interner Module
```rust
// src/interner.rs
use std::collections::HashMap;
use std::sync::RwLock;

pub struct StringInterner {
    storage: RwLock<HashMap<String, &'static str>>,
}

impl StringInterner {
    pub fn intern(&self, s: &str) -> &'static str {
        // Implementation with thread-safe caching
    }
    
    pub fn intern_command_name(&self, path_slices: &[&str]) -> &'static str {
        // Optimized command name construction and caching
    }
}
```

#### 2. Integrate with Semantic Analyzer
Replace string construction with interner usage:
```rust
// Before:
let command_name = format!(".{}", instruction.command_path_slices.join("."));

// After:  
let command_name = INTERNER.intern_command_name(&instruction.command_path_slices);
```

#### 3. Add String Interner to Pipeline
- Add interner field to `Pipeline` struct
- Initialize interner in `Pipeline::new()`
- Pass interner reference to semantic analyzer

### Technical Requirements

#### Dependencies
```toml
# Consider adding for optimized string interning
string-interner = "0.15"  # Optional: specialized interner crate
```

#### Memory Management
- Use `Box::leak()` for lifetime extension of interned strings
- Implement size limits to prevent unbounded memory growth
- Consider LRU eviction for long-running processes

#### Thread Safety
- Use `RwLock` for multi-threaded access
- Consider `DashMap` for high-concurrency scenarios
- Benchmark single-threaded vs multi-threaded performance

### Performance Targets

- **Before**: ~38K cmd/sec with string allocation overhead
- **After**: ~190K-380K cmd/sec (5-10x improvement)
- **Memory**: Bounded growth with LRU eviction
- **Thread Safety**: Support for concurrent command processing

### Testing Strategy

#### Benchmarks
1. Microbenchmark string construction vs interning
2. Integration benchmark with full command pipeline
3. Memory usage analysis with long-running processes
4. Concurrent access performance testing

#### Regression Tests
1. Verify command name correctness for all test cases
2. Ensure thread safety with concurrent command processing
3. Memory leak testing with continuous operation
4. Performance regression protection

### Implementation Steps

1. **Create interner module** with basic functionality
2. **Add microbenchmarks** to validate performance gains
3. **Integrate with semantic analyzer** in hot path
4. **Add comprehensive tests** for correctness and performance
5. **Optimize memory management** with size limits
6. **Benchmark full pipeline** to measure end-to-end improvement

### Success Criteria

- [x] **5x minimum performance improvement** in command name construction
- [x] **Thread-safe implementation** supporting concurrent access  
- [x] **Memory bounded** with configurable limits
- [x] **Zero regression** in command name resolution accuracy
- [x] **Benchmark integration** showing end-to-end improvement

### Related Tasks

- Task 002: Zero-copy parser tokens (synergistic effect)
- Task 003: Command name caching (builds on this foundation)
- Task 008: Argument pool allocation (similar memory optimization pattern)