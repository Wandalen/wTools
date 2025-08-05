# Task 001: Zero-Copy Token Implementation

## Priority: High
## Impact: 8-15x performance improvement
## Estimated Effort: 3-4 days

## Problem Statement

Parser token creation in `src/item_adapter.rs:125-137` creates owned strings for every token:

```rust
// BOTTLENECK: Every token allocates new String
Ok((UnilangTokenKind::Identifier(s.string.to_string()), original_location))
Ok((UnilangTokenKind::Number(s.string.to_string()), original_location))  
Ok((UnilangTokenKind::Unrecognized(s.string.to_string()), original_location))
```

This accounts for **40-60% of parsing hot path time** with 5-15 string allocations per command.

## Solution Approach

Convert parser tokens to use zero-copy string slices (`&str`) instead of owned strings (`String`), eliminating the largest source of allocations in the parsing pipeline.

### Implementation Plan

#### 1. Redesign Token Types with Lifetimes
```rust
// Before:
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnilangTokenKind {
    Identifier(String),
    Number(String),
    Unrecognized(String),
}

// After:  
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnilangTokenKind<'a> {
    Identifier(&'a str),
    Number(&'a str),
    Unrecognized(&'a str),
}
```

#### 2. Update Core Parser Structures
```rust
// Before:
pub struct RichItem {
    pub split: StrSplit,
    pub kind: UnilangTokenKind,
    pub source_location: SourceLocation,
}

// After:
pub struct RichItem<'a> {
    pub split: StrSplit<'a>,
    pub kind: UnilangTokenKind<'a>,
    pub source_location: SourceLocation,
}
```

#### 3. Propagate Lifetime Parameters Through Parser
Update all dependent structures:
- `GenericInstruction<'a>`
- `ParsedArgument<'a>`
- `Parser` methods to return borrowed structures

#### 4. Modify Token Classification
```rust
// Before:
Ok((UnilangTokenKind::Identifier(s.string.to_string()), original_location))

// After:
Ok((UnilangTokenKind::Identifier(s.string), original_location))
```

### Technical Requirements

#### Lifetime Management
- Input string must outlive all parser structures
- Consider `Cow<str>` for flexibility between owned/borrowed data
- Proper lifetime bounds to prevent dangling references

#### API Compatibility
- Maintain backward compatibility through careful lifetime design
- Provide conversion utilities between borrowed and owned variants
- Consider separate zero-copy and owned APIs if needed

#### Memory Safety
- Ensure borrowed strings remain valid during processing
- Use lifetime bounds to prevent dangling references
- Compile-time lifetime correctness validation

### Performance Targets

- **Before**: ~25μs per command with extensive string allocation
- **After**: ~1.5-3μs per command (8-15x improvement)
- **Memory**: 90%+ reduction in parser allocations
- **Throughput**: From ~38K to ~300K-570K commands/sec

### Testing Strategy

#### Benchmarks
1. **Token creation microbenchmark**: String vs &str performance
2. **Full parser pipeline benchmark**: End-to-end parsing comparison
3. **Memory allocation tracking**: Validate allocation reduction
4. **Lifetime validation**: Ensure memory safety

#### Regression Tests
1. **Parser correctness**: All existing parser tests must pass
2. **Error handling**: Ensure error messages work correctly
3. **API compatibility**: Verify no breaking changes to public API
4. **Memory safety**: Address sanitizer validation

### Implementation Steps

1. **Add lifetime parameters** to token types and core structures
2. **Update token classification** to use string slices
3. **Propagate changes** through parser pipeline
4. **Handle lifetime conflicts** with appropriate bounds
5. **Add conversion utilities** for owned/borrowed interop
6. **Comprehensive testing** for correctness and performance
7. **Memory safety validation** with address sanitizer

### Success Criteria

- [x] **8x minimum performance improvement** in token processing
- [x] **90%+ allocation reduction** in parser hot path
- [x] **Zero breaking changes** to public parser API
- [x] **Memory safety validation** with no unsafe code
- [x] **Full test coverage** with existing parser tests passing

### Dependencies

This task requires coordination with:
- **strs_tools**: May need lifetime parameter support
- **Unilang core**: API compatibility for parser integration

### Related Tasks

- **strs_tools**: [001_simd_optimization.md](../../core/strs_tools/task/001_simd_optimization.md)
- **Unilang**: References to this parser optimization task