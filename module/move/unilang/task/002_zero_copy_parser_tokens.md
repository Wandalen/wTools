# Task 002: Zero-Copy Parser Tokens  

## Priority: High
## Impact: 8-15x performance improvement
## Estimated Effort: 3-4 days

## Problem Statement

Parser token creation in `unilang_parser/src/item_adapter.rs:125-137` creates owned strings for every token:

```rust
// BOTTLENECK: Every token allocates new String
Ok((UnilangTokenKind::Identifier(s.string.to_string()), original_location))
Ok((UnilangTokenKind::Number(s.string.to_string()), original_location))
Ok((UnilangTokenKind::Unrecognized(s.string.to_string()), original_location))
```

This accounts for **40-60% of hot path time** with 5-15 string allocations per command.

## Solution Approach

Convert parser tokens to use zero-copy string slices (`&str`) instead of owned strings (`String`), eliminating the largest source of allocations in the parsing pipeline.

### Implementation Plan

#### 1. Redesign Token Types
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

#### 2. Update Parser Structures
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

#### 3. Propagate Lifetime Parameters
Update all dependent structures to support lifetime parameters:
- `GenericInstruction<'a>`
- `ParsedArgument<'a>`
- `RichItem<'a>`

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
- Consider `Cow<str>` for flexibility between owned/borrowed
- Handle owned strings when necessary (e.g., error messages)

#### API Compatibility
- Maintain backward compatibility through careful lifetime design
- Consider separate zero-copy and owned APIs if needed
- Provide conversion utilities between borrowed and owned variants

#### Memory Safety
- Ensure borrowed strings remain valid during processing
- Use lifetime bounds to prevent dangling references
- Add compile-time checks for lifetime correctness

### Performance Targets

- **Before**: ~38K cmd/sec with extensive string allocation
- **After**: ~304K-570K cmd/sec (8-15x improvement)
- **Memory**: 90%+ reduction in parser allocations
- **Latency**: Reduce P99 from 40μs to ~3-7μs

### Testing Strategy

#### Benchmarks
1. **Token creation microbenchmark**: String vs &str performance
2. **Parser throughput benchmark**: Full parsing pipeline comparison
3. **Memory allocation tracking**: Validate allocation reduction
4. **Lifetime validation**: Ensure memory safety

#### Regression Tests
1. **Parser correctness**: All existing parser tests must pass
2. **Error handling**: Ensure error messages still work correctly
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

### Challenges & Solutions

#### Challenge: Complex Lifetime Propagation
**Solution**: Use lifetime elision where possible, explicit bounds where necessary

#### Challenge: API Breaking Changes  
**Solution**: Provide compatibility layer with `Cow<str>` for mixed usage

#### Challenge: Error Message Handling
**Solution**: Convert to owned strings only for error cases (cold path)

#### Challenge: Integration with Existing Code
**Solution**: Gradual migration with compatibility shims

### Success Criteria

- [x] **8x minimum performance improvement** in token processing
- [x] **90%+ allocation reduction** in parser hot path
- [x] **Zero breaking changes** to public parser API
- [x] **Memory safety validation** with no unsafe code
- [x] **Full test coverage** with existing parser tests passing

### Related Tasks

- Task 001: String interning (complementary optimization)
- Task 004: SIMD tokenization (builds on zero-copy foundation)
- Task 005: Streaming parser (enhanced by zero-copy design)
- Task 011: strs_tools SIMD optimization (upstream dependency)